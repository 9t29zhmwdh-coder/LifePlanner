import { useState } from 'react'
import { api, newUuid, type CalEvent, type Task } from '../../lib/tauri'
import { usePlannerStore } from '../../stores/plannerStore'

type Tab = 'today' | 'calendar' | 'tasks' | 'projects' | 'capture' | 'search' | 'settings'
interface Props { onNavigate: (t: Tab) => void }

interface ExtractionResult {
  events: CalEvent[]
  tasks: Task[]
}

export function CaptureView({ onNavigate }: Props) {
  const { loadAll, ollamaOnline } = usePlannerStore()
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
      const r = await api.aiExtractFromText(text)
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
        <div className="text-sm font-medium text-[#e6edf3] mb-1">Text erfassen</div>
        <div className="text-xs text-[#8b949e]">
          Fügen Sie E-Mails, Notizen oder Texte ein — Termine und Aufgaben werden automatisch erkannt.
        </div>
      </div>

      <div className="flex-1 flex flex-col overflow-hidden p-4 gap-4">
        {/* Input */}
        <div className="flex flex-col gap-2">
          <textarea
            value={text}
            onChange={e => setText(e.target.value)}
            placeholder="Text hier einfügen…&#10;&#10;Beispiel:&#10;Meeting morgen um 14:00 Uhr mit dem Team.&#10;Bericht bis Freitag einreichen.&#10;Nächsten Montag: Präsentation vorbereiten."
            className="w-full h-40 bg-[#21262d] border border-[#30363d] rounded-lg px-3 py-2.5 text-sm text-[#e6edf3] resize-none focus:outline-none focus:border-[#58a6ff] placeholder-[#484f58] font-mono"
          />
          <div className="flex gap-2 items-center">
            <button onClick={handleExtract} disabled={extracting || !text.trim()}
              className="px-4 py-2 text-xs bg-[#21262d] border border-[#30363d] hover:border-[#58a6ff] text-[#e6edf3] rounded-md transition-colors disabled:opacity-50">
              {extracting ? '⟳ Erkenne…' : '🔍 Erkennen'}
            </button>
            <button onClick={handleAiExtract} disabled={aiExtracting || !text.trim() || !ollamaOnline}
              className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors disabled:opacity-50"
              title={!ollamaOnline ? 'Ollama nicht verfügbar' : ''}>
              {aiExtracting ? '⟳ KI analysiert…' : '✨ KI-Erkennung'}
            </button>
            {text && (
              <button onClick={handleClear} className="ml-auto text-xs text-[#8b949e] hover:text-[#e6edf3]">
                Leeren
              </button>
            )}
          </div>
        </div>

        {/* Results */}
        {result && (
          <div className="flex-1 overflow-y-auto space-y-4">
            {result.events.length === 0 && result.tasks.length === 0 ? (
              <div className="text-center text-[#8b949e] text-sm py-8">
                Keine Termine oder Aufgaben erkannt.
              </div>
            ) : (
              <>
                {result.events.length > 0 && (
                  <div>
                    <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                      Erkannte Termine ({result.events.length})
                    </div>
                    {result.events.map(ev => (
                      <div key={ev.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2">
                        <div className="text-sm text-[#79c0ff] font-medium">{ev.title}</div>
                        <div className="text-xs text-[#8b949e] mt-1">
                          {new Date(ev.start).toLocaleString('de-CH')}
                          {ev.end && ` – ${new Date(ev.end).toLocaleTimeString('de-CH', { hour: '2-digit', minute: '2-digit' })}`}
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
                      Erkannte Aufgaben ({result.tasks.length})
                    </div>
                    {result.tasks.map(t => (
                      <div key={t.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2">
                        <div className="text-sm text-[#e6edf3]">{t.title}</div>
                        <div className="flex items-center gap-2 mt-1">
                          {t.due_date && (
                            <span className="text-[9px] text-[#d29922]">
                              Fällig: {new Date(t.due_date).toLocaleDateString('de-CH')}
                            </span>
                          )}
                          <span className="text-[9px] text-[#8b949e]">{t.priority}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                <div className="flex gap-2 pt-2">
                  {!saved ? (
                    <button onClick={handleSave}
                      className="px-4 py-2 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors">
                      Alle speichern & übernehmen
                    </button>
                  ) : (
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-[#3fb950]">✓ Gespeichert</span>
                      <button onClick={() => onNavigate('today')} className="text-xs text-[#58a6ff] hover:underline">
                        Zum Heute-Tab →
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
