import { useState, useRef } from 'react'
import { api, formatDate, formatTime, PRIORITY_LABELS, ENERGY_LABELS, type CalEvent, type Task } from '../../lib/tauri'

interface SearchResults {
  events: CalEvent[]
  tasks: Task[]
}

export function SearchView() {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResults | null>(null)
  const [searching, setSearching] = useState(false)
  const debounce = useRef<ReturnType<typeof setTimeout> | null>(null)

  const doSearch = async (q: string) => {
    if (!q.trim()) { setResults(null); return }
    setSearching(true)
    try {
      const r = await api.search(q)
      setResults(r)
    } catch {
      setResults({ events: [], tasks: [] })
    } finally {
      setSearching(false)
    }
  }

  const handleChange = (q: string) => {
    setQuery(q)
    if (debounce.current) clearTimeout(debounce.current)
    debounce.current = setTimeout(() => doSearch(q), 300)
  }

  const total = results ? results.events.length + results.tasks.length : 0

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="p-4 border-b border-[#30363d]">
        <div className="relative">
          <span className="absolute left-3 top-1/2 -translate-y-1/2 text-[#8b949e] text-sm">🔍</span>
          <input
            autoFocus
            value={query}
            onChange={e => handleChange(e.target.value)}
            placeholder="Termine, Aufgaben und Projekte durchsuchen…"
            className="w-full bg-[#21262d] border border-[#30363d] rounded-lg pl-9 pr-4 py-2.5 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]"
          />
          {searching && (
            <span className="absolute right-3 top-1/2 -translate-y-1/2 text-[#8b949e] text-xs">⟳</span>
          )}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {!query && (
          <div className="flex items-center justify-center h-48">
            <div className="text-center">
              <div className="text-4xl mb-3">🔍</div>
              <div className="text-[#8b949e] text-sm">Suchbegriff eingeben…</div>
              <div className="text-[#484f58] text-xs mt-1">Volltextsuche über alle Termine und Aufgaben</div>
            </div>
          </div>
        )}

        {results && !searching && total === 0 && query && (
          <div className="flex items-center justify-center h-48">
            <div className="text-center">
              <div className="text-3xl mb-2">📭</div>
              <div className="text-[#8b949e] text-sm">Keine Ergebnisse für „{query}"</div>
            </div>
          </div>
        )}

        {results && total > 0 && (
          <div className="p-4 space-y-4">
            <div className="text-xs text-[#8b949e]">{total} Ergebnis{total !== 1 ? 'se' : ''}</div>

            {results.events.length > 0 && (
              <section>
                <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                  Termine ({results.events.length})
                </div>
                {results.events.map(ev => (
                  <div key={ev.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2 hover:border-[#58a6ff] transition-colors">
                    <div className="text-sm text-[#79c0ff] font-medium">{highlight(ev.title, query)}</div>
                    <div className="text-xs text-[#8b949e] mt-1">
                      {formatDate(ev.start)} · {formatTime(ev.start)}
                      {ev.end && ` – ${formatTime(ev.end)}`}
                    </div>
                    {ev.location && (
                      <div className="text-xs text-[#8b949e]">📍 {ev.location}</div>
                    )}
                    {ev.description && (
                      <div className="text-xs text-[#8b949e] mt-1 truncate">{ev.description}</div>
                    )}
                  </div>
                ))}
              </section>
            )}

            {results.tasks.length > 0 && (
              <section>
                <div className="text-xs font-medium text-[#8b949e] mb-2 uppercase tracking-wide">
                  Aufgaben ({results.tasks.length})
                </div>
                {results.tasks.map(t => (
                  <div key={t.id} className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 mb-2 hover:border-[#58a6ff] transition-colors">
                    <div className="text-sm text-[#e6edf3] font-medium">{highlight(t.title, query)}</div>
                    <div className="flex items-center gap-3 mt-1">
                      <span className="text-[9px] text-[#8b949e]">{PRIORITY_LABELS[t.priority as keyof typeof PRIORITY_LABELS]}</span>
                      <span className="text-[9px] text-[#8b949e]">{ENERGY_LABELS[t.energy_level as keyof typeof ENERGY_LABELS]}</span>
                      {t.due_date && (
                        <span className={`text-[9px] ${new Date(t.due_date) < new Date() ? 'text-[#f85149]' : 'text-[#8b949e]'}`}>
                          Fällig: {formatDate(t.due_date)}
                        </span>
                      )}
                      <span className="text-[9px] text-[#8b949e]">{t.status}</span>
                    </div>
                    {t.description && (
                      <div className="text-xs text-[#8b949e] mt-1 truncate">{t.description}</div>
                    )}
                  </div>
                ))}
              </section>
            )}
          </div>
        )}
      </div>
    </div>
  )
}

function highlight(text: string, query: string): React.ReactNode {
  if (!query.trim()) return text
  const idx = text.toLowerCase().indexOf(query.toLowerCase())
  if (idx === -1) return text
  return (
    <>
      {text.slice(0, idx)}
      <mark className="bg-[#d2992230] text-[#d29922] rounded px-0.5">{text.slice(idx, idx + query.length)}</mark>
      {text.slice(idx + query.length)}
    </>
  )
}
