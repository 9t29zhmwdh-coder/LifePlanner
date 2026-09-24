import { useRef, useState, type ClipboardEvent, type DragEvent } from 'react'
import { api, errorText, formatDateTime, formatTime, titleOf, type ExtractionPreview } from '../../lib/tauri'
import { usePlannerStore } from '../../stores/plannerStore'
import { useSettingsStore } from '../../stores/settingsStore'
import { useT } from '../../lib/i18n'

type Tab = 'today' | 'calendar' | 'tasks' | 'projects' | 'capture' | 'search' | 'settings'
interface Props { onNavigate: (t: Tab) => void }

const FILE_TYPES = '.pdf,.eml,.txt,.md'

/** An .eml starts with mail headers; those go through the email extractor, which uses the subject as title. */
function looksLikeEmail(text: string): boolean {
  return /^(from|subject|date|to|received|return-path):/im.test(text.slice(0, 2000))
}

export function CaptureView({ onNavigate }: Props) {
  const { loadAll, ollamaOnline } = usePlannerStore()
  const autoExtract = useSettingsStore(s => s.settings.auto_extract_on_paste)
  const t = useT()
  const fileInput = useRef<HTMLInputElement>(null)
  const [text, setText] = useState('')
  const [busy, setBusy] = useState<'detect' | 'ai' | 'file' | 'save' | null>(null)
  const [preview, setPreview] = useState<ExtractionPreview | null>(null)
  const [skipped, setSkipped] = useState<Set<string>>(new Set())
  // Detected titles are a guess; the person can correct them before saving.
  const [titles, setTitles] = useState<Record<string, string>>({})
  const [error, setError] = useState<string | null>(null)
  const [savedCount, setSavedCount] = useState<number | null>(null)
  const [dragging, setDragging] = useState(false)

  const run = async (kind: 'detect' | 'ai' | 'file', job: () => Promise<ExtractionPreview>) => {
    setBusy(kind)
    setPreview(null)
    setSkipped(new Set())
    setTitles({})
    setError(null)
    setSavedCount(null)
    try {
      setPreview(await job())
    } catch (e) {
      setError(errorText(e))
    } finally {
      setBusy(null)
    }
  }

  const detect = (source: string) => {
    if (!source.trim()) return
    run('detect', () => looksLikeEmail(source) ? api.extractEmail(source) : api.extractText(source))
  }

  const handlePaste = (e: ClipboardEvent<HTMLTextAreaElement>) => {
    if (!autoExtract) return
    const pasted = e.clipboardData.getData('text')
    // The textarea updates after this handler; detect on what it will contain.
    const next = text + pasted
    if (next.trim()) setTimeout(() => detect(next), 0)
  }

  const openFile = async (file: File) => {
    if (file.name.toLowerCase().endsWith('.pdf')) {
      setText('')
      await run('file', async () => api.extractPdf(await file.arrayBuffer()))
      return
    }
    const content = await file.text()
    setText(content)
    detect(content)
  }

  const handleDrop = (e: DragEvent) => {
    e.preventDefault()
    setDragging(false)
    const file = e.dataTransfer.files[0]
    if (file) openFile(file)
  }

  const toggle = (id: string) => setSkipped(s => {
    const next = new Set(s)
    if (next.has(id)) next.delete(id); else next.add(id)
    return next
  })

  const handleSave = async () => {
    if (!preview) return
    setBusy('save')
    setError(null)
    const { result } = preview
    const keep = <T extends { id: string; title: string }>(items: T[]) =>
      items.filter(i => !skipped.has(i.id)).map(i => ({ ...i, title: titleOf({ title: titles[i.id] ?? i.title }) }))
    try {
      const count = await api.saveExtraction({ ...result, events: keep(result.events), tasks: keep(result.tasks) })
      await loadAll()
      setSavedCount(count)
    } catch (e) {
      setError(errorText(e))
    } finally {
      setBusy(null)
    }
  }

  const handleClear = () => {
    setText('')
    setPreview(null)
    setError(null)
    setSavedCount(null)
  }

  const result = preview?.result
  const conflictsOf = (id: string) => preview?.conflicts.filter(c => c.event_id === id) ?? []
  const selected = result
    ? result.events.length + result.tasks.length - skipped.size
    : 0

  return (
    <div className="h-full flex flex-col overflow-hidden"
      onDragOver={e => { e.preventDefault(); setDragging(true) }}
      onDragLeave={() => setDragging(false)}
      onDrop={handleDrop}>
      <div className="p-4 border-b border-[#30363d]">
        <div className="text-sm font-medium text-[#e6edf3] mb-1">{t('captureTitle')}</div>
        <div className="text-xs text-[#8b949e]">{t('captureSubtitle')}</div>
      </div>

      <div className="flex-1 flex flex-col overflow-hidden p-4 gap-4">
        <div className="flex flex-col gap-2">
          <textarea
            value={text}
            onChange={e => setText(e.target.value)}
            onPaste={handlePaste}
            placeholder={t('textPlaceholder')}
            className={`w-full h-40 bg-[#21262d] border rounded-lg px-3 py-2.5 text-sm text-[#e6edf3] resize-none focus:outline-hidden focus:border-[#58a6ff] placeholder-[#484f58] font-mono ${dragging ? 'border-[#58a6ff] border-dashed' : 'border-[#30363d]'}`}
          />
          <div className="flex gap-2 items-center flex-wrap">
            <button onClick={() => detect(text)} disabled={busy !== null || !text.trim()}
              className="px-4 py-2 text-xs bg-[#21262d] border border-[#30363d] hover:border-[#58a6ff] text-[#e6edf3] rounded-md transition-colors disabled:opacity-50">
              {busy === 'detect' ? `⟳ ${t('detecting')}` : t('detect')}
            </button>
            <button onClick={() => run('ai', () => api.aiExtract(text))}
              disabled={busy !== null || !text.trim() || !ollamaOnline}
              className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors disabled:opacity-50"
              title={!ollamaOnline ? t('ollamaNotAvailable') : ''}>
              {busy === 'ai' ? `⟳ ${t('aiAnalyzing')}` : t('aiDetection')}
            </button>
            <button onClick={() => fileInput.current?.click()} disabled={busy !== null}
              className="px-4 py-2 text-xs bg-[#21262d] border border-[#30363d] hover:border-[#58a6ff] text-[#e6edf3] rounded-md transition-colors disabled:opacity-50">
              {busy === 'file' ? `⟳ ${t('readingFile')}` : t('openFile')}
            </button>
            <input ref={fileInput} type="file" accept={FILE_TYPES} className="hidden"
              onChange={e => { const f = e.target.files?.[0]; if (f) openFile(f); e.target.value = '' }} />
            <span className="text-xs text-[#8b949e]">{t('dropHint')}</span>
            {(text || preview) && (
              <button onClick={handleClear} className="ml-auto text-xs text-[#8b949e] hover:text-[#e6edf3]">
                {t('clear')}
              </button>
            )}
          </div>
        </div>

        {error && (
          <div className="text-xs text-[#f85149] bg-[#f85149]/10 border border-[#f85149]/40 rounded-md px-3 py-2">
            {error}
          </div>
        )}

        {result && (
          <div className="flex-1 overflow-y-auto space-y-4">
            {result.events.length === 0 && result.tasks.length === 0 ? (
              <div className="text-center text-[#8b949e] text-sm py-8">{t('noItemsDetected')}</div>
            ) : (
              <>
                {result.events.length > 0 && (
                  <div>
                    <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                      {t('detectedEvents', { n: result.events.length })}
                    </div>
                    {result.events.map(ev => (
                      <label key={ev.id} className="flex gap-3 bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2 cursor-pointer">
                        <input type="checkbox" checked={!skipped.has(ev.id)} onChange={() => toggle(ev.id)}
                          disabled={savedCount !== null} className="mt-1" />
                        <div className="flex-1 min-w-0">
                          <TitleInput value={titles[ev.id] ?? ev.title} onChange={v => setTitles(s => ({ ...s, [ev.id]: v }))}
                            className="text-[#79c0ff] font-medium" disabled={savedCount !== null} />
                          <div className="text-xs text-[#8b949e] mt-1">
                            {formatDateTime(ev.start)}
                            {ev.end && ` → ${formatTime(ev.end)}`}
                          </div>
                          {ev.location && <div className="text-xs text-[#8b949e]">📍 {ev.location}</div>}
                          {conflictsOf(ev.id).map(c => (
                            <div key={c.existing_title + c.existing_start} className="text-xs text-[#d29922] mt-1">
                              ⚠ {t('collidesWith', { title: c.existing_title || t('untitled'), time: formatDateTime(c.existing_start) })}
                            </div>
                          ))}
                        </div>
                      </label>
                    ))}
                  </div>
                )}

                {result.tasks.length > 0 && (
                  <div>
                    <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                      {t('detectedTasks', { n: result.tasks.length })}
                    </div>
                    {result.tasks.map(task => (
                      <label key={task.id} className="flex gap-3 bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2 cursor-pointer">
                        <input type="checkbox" checked={!skipped.has(task.id)} onChange={() => toggle(task.id)}
                          disabled={savedCount !== null} className="mt-1" />
                        <div className="flex-1 min-w-0">
                          <TitleInput value={titles[task.id] ?? task.title} onChange={v => setTitles(s => ({ ...s, [task.id]: v }))}
                            className="text-[#e6edf3]" disabled={savedCount !== null} />
                          {task.due_date && (
                            <div className="text-xs text-[#d29922] mt-1">
                              {t('due')}: {formatDateTime(task.due_date)}
                            </div>
                          )}
                        </div>
                      </label>
                    ))}
                  </div>
                )}

                <div className="flex gap-2 pt-2 items-center">
                  {savedCount === null ? (
                    <button onClick={handleSave} disabled={busy !== null || selected === 0}
                      className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors disabled:opacity-50">
                      {busy === 'save' ? `⟳ ${t('saving')}` : t('saveSelected', { n: selected })}
                    </button>
                  ) : (
                    <>
                      <span className="text-xs text-[#3fb950]">{t('savedCount', { n: savedCount })}</span>
                      <button onClick={() => onNavigate('today')} className="text-xs text-[#58a6ff] hover:underline">
                        {t('goToToday')}
                      </button>
                    </>
                  )}
                </div>
              </>
            )}
          </div>
        )}
      </div>
    </div>
  )
}

function TitleInput({ value, onChange, className, disabled }:
  { value: string; onChange: (v: string) => void; className: string; disabled: boolean }) {
  const t = useT()
  return (
    <input value={value} onChange={e => onChange(e.target.value)} disabled={disabled}
      placeholder={t('untitled')} onClick={e => e.preventDefault()}
      className={`w-full text-sm bg-transparent border-b border-transparent hover:border-[#30363d] focus:border-[#58a6ff] focus:outline-hidden ${className}`} />
  )
}
