import { useState } from 'react'
import { api, newUuid, type CalEvent, type Task } from '../../lib/tauri'
import { usePlannerStore } from '../../stores/plannerStore'
import { useT } from '../../lib/i18n'

type Tab = 'today' | 'calendar' | 'tasks' | 'projects' | 'capture' | 'search' | 'settings'
interface Props { onNavigate: (t: Tab) => void }

interface ExtractionResult {
  events: CalEvent[]
  tasks: Task[]
}

export function CaptureView({ onNavigate }: Props) {
  const { loadAll, ollamaOnline } = usePlannerStore()
  const t = useT()
  const [text, setText] = useState('')
  const [extracting, setExtracting] = useState(false)
  const [aiExtracting, setAiExtracting] = useState(false)
  const [result, setResult] = useState<ExtractionResult | null>(null)
  const [saved, setSaved] = useState(false)

  const handleExtract = async () => {
    if (!text.trim()) return
    setExtracting(true)
    setResult(null)
    setSaved(false)
    try {
      const r = await api.extractText(text)
      setResult(r)
    } catch {
      setResult({ events: [], tasks: [] })
    } finally {
      setExtracting(false)
    }
  }

  const handleAiExtract = async () => {
    if (!text.trim() || !ollamaOnline) return
    setAiExtracting(true)
    setResult(null)
    setSaved(false)
    try {
      const r = await api.aiExtract(text)
      setResult(r)
    } catch {
      setResult({ events: [], tasks: [] })
    } finally {
      setAiExtracting(false)
    }
  }

  const handleSave = async () => {
    if (!result) return
    await loadAll()
    setSaved(true)
  }

  const handleClear = () => {
    setText('')
    setResult(null)
    setSaved(false)
  }

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="p-4 border-b border-[#30363d]">
        <div className="text-sm font-medium text-[#e6edf3] mb-1">{t('captureTitle')}</div>
        <div className="text-xs text-[#8b949e]">
          {t('captureSubtitle')}
        </div>
      </div>

      <div className="flex-1 flex flex-col overflow-hidden p-4 gap-4">
        {/* Input */}
        <div className="flex flex-col gap-2">
          <textarea
            value={text}
            onChange={e => setText(e.target.value)}
            placeholder={t('textPlaceholder')}
            className="w-full h-40 bg-[#21262d] border border-[#30363d] rounded-lg px-3 py-2.5 text-sm text-[#e6edf3] resize-none focus:outline-hidden focus:border-[#58a6ff] placeholder-[#484f58] font-mono"
          />
          <div className="flex gap-2 items-center">
            <button onClick={handleExtract} disabled={extracting || !text.trim()}
              className="px-4 py-2 text-xs bg-[#21262d] border border-[#30363d] hover:border-[#58a6ff] text-[#e6edf3] rounded-md transition-colors disabled:opacity-50">
              {extracting ? `⟳ ${t('detecting')}` : t('detect')}
            </button>
            <button onClick={handleAiExtract} disabled={aiExtracting || !text.trim() || !ollamaOnline}
              className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors disabled:opacity-50"
              title={!ollamaOnline ? t('ollamaNotAvailable') : ''}>
              {aiExtracting ? `⟳ ${t('aiAnalyzing')}` : t('aiDetection')}
            </button>
            {text && (
              <button onClick={handleClear} className="ml-auto text-xs text-[#8b949e] hover:text-[#e6edf3]">
                {t('clear')}
              </button>
            )}
          </div>
        </div>

        {/* Results */}
        {result && (
          <div className="flex-1 overflow-y-auto space-y-4">
            {result.events.length === 0 && result.tasks.length === 0 ? (
              <div className="text-center text-[#8b949e] text-sm py-8">
                {t('noItemsDetected')}
              </div>
            ) : (
              <>
                {result.events.length > 0 && (
                  <div>
                    <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                      {t('detectedEvents', { n: result.events.length })}
                    </div>
                    {result.events.map(ev => (
                      <div key={ev.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2">
                        <div className="text-sm text-[#79c0ff] font-medium">{ev.title}</div>
                        <div className="text-xs text-[#8b949e] mt-1">
                          {new Date(ev.start).toLocaleString()}
                          {ev.end && ` → ${new Date(ev.end).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`}
                        </div>
                        {ev.location && <div className="text-xs text-[#8b949e]">📍 {ev.location}</div>}
                        {ev.description && <div className="text-xs text-[#8b949e] mt-1 italic">{ev.description}</div>}
                      </div>
                    ))}
                  </div>
                )}

                {result.tasks.length > 0 && (
                  <div>
                    <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                      {t('detectedTasks', { n: result.tasks.length })}
                    </div>
                    {result.tasks.map(task => (
                      <div key={task.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2">
                        <div className="text-sm text-[#e6edf3]">{task.title}</div>
                        <div className="flex items-center gap-2 mt-1">
                          {task.due_date && (
                            <span className="text-[9px] text-[#d29922]">
                              {t('due')}: {new Date(task.due_date).toLocaleDateString()}
                            </span>
                          )}
                          <span className="text-[9px] text-[#8b949e]">{task.priority}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                <div className="flex gap-2 pt-2">
                  {!saved ? (
                    <button onClick={handleSave}
                      className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors">
                      {t('saveAll')}
                    </button>
                  ) : (
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-[#3fb950]">{t('saved')}</span>
                      <button onClick={() => onNavigate('today')} className="text-xs text-[#58a6ff] hover:underline">
                        {t('goToToday')}
                      </button>
                    </div>
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
