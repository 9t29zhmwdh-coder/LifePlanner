import { useState } from 'react'
import { usePlannerStore } from '../../stores/plannerStore'
import { api, formatTime, formatDate, PRIORITY_COLORS, energyLabel, ENERGY_COLORS, type TaskPriority, type EnergyLevel } from '../../lib/tauri'
import { useT, getLang } from '../../lib/i18n'

type Tab = 'today' | 'calendar' | 'tasks' | 'projects' | 'capture' | 'search' | 'settings'
interface Props { onNavigate: (t: Tab) => void }

export function TodayView({ onNavigate }: Props) {
  const { summary, freeSlots, aiText, setAiText, ollamaOnline, loadAll } = usePlannerStore()
  const [generating, setGenerating] = useState(false)
  const t = useT()

  const handleGenerateSummary = async () => {
    setGenerating(true)
    try {
      const text = await api.generateSummary()
      setAiText(text)
    } catch {
      setAiText(t('aiUnavailable'))
    } finally {
      setGenerating(false)
    }
  }

  const scoreColor = (s: number) =>
    s >= 80 ? '#3fb950' : s >= 60 ? '#d29922' : s >= 40 ? '#f0883e' : '#f85149'

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-xl font-semibold text-[#e6edf3]">
            {new Date().toLocaleDateString(getLang() === 'de' ? 'de-CH' : 'en-US', { weekday: 'long', day: '2-digit', month: 'long' })}
          </h1>
          <div className="text-xs text-[#8b949e] mt-0.5">{t('todayOverview')}</div>
        </div>
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1.5">
            <div className={`w-1.5 h-1.5 rounded-full ${ollamaOnline ? 'bg-[#3fb950]' : 'bg-[#f85149]'}`} />
            <span className="text-xs text-[#8b949e]">{ollamaOnline ? t('aiReady') : t('aiOffline')}</span>
          </div>
          <button onClick={handleGenerateSummary} disabled={generating || !ollamaOnline}
            className="px-3 py-1.5 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-lg transition-colors disabled:opacity-50">
            {generating ? `⟳ ${t('generating')}` : `✨ ${t('aiSummary')}`}
          </button>
        </div>
      </div>

      {/* AI Summary */}
      {aiText && (
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 mb-6">
          <div className="text-xs text-[#8b949e] mb-2 flex items-center gap-1">
            <span>🤖</span> {t('aiSummary')}
          </div>
          <div className="text-sm text-[#e6edf3] leading-relaxed">{aiText}</div>
        </div>
      )}

      {!summary ? (
        <div className="flex items-center justify-center h-48">
          <div className="text-center">
            <div className="text-4xl mb-2">📅</div>
            <div className="text-[#8b949e] text-sm">{t('loadingDay')}</div>
          </div>
        </div>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Score */}
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 flex items-center gap-4">
            <div className="w-16 h-16 rounded-full flex items-center justify-center text-2xl font-bold"
              style={{ background: scoreColor(summary.score) + '20', color: scoreColor(summary.score) }}>
              {summary.score}
            </div>
            <div>
              <div className="text-xs text-[#8b949e] mb-1">{t('dailyScore')}</div>
              <div className="text-sm text-[#e6edf3]">
                {t('eventsAndTasks', { events: summary.events.length, tasks: summary.tasks_due.length })}
              </div>
              {summary.conflicts.length > 0 && (
                <div className="text-xs text-[#f85149] mt-0.5">
                  ⚠ {summary.conflicts.length} {summary.conflicts.length > 1 ? t('conflicts') : t('conflict')}
                </div>
              )}
            </div>
          </div>

          {/* Free slots */}
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
            <div className="text-xs text-[#8b949e] mb-2">{t('freeSlots')}</div>
            {freeSlots.length === 0 ? (
              <div className="text-xs text-[#8b949e]">{t('noFreeSlots')}</div>
            ) : (
              <div className="space-y-1">
                {freeSlots.slice(0, 3).map((s, i) => (
                  <div key={i} className="flex items-center justify-between text-xs">
                    <span className="text-[#e6edf3]">{formatTime(s.start)} → {formatTime(s.end)}</span>
                    <span className="text-[#8b949e]">{s.duration_minutes} {t('minUnit')}</span>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Overdue */}
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
            <div className="text-xs text-[#8b949e] mb-2">{t('overdue')} ({summary.tasks_overdue.length})</div>
            {summary.tasks_overdue.length === 0 ? (
              <div className="text-xs text-[#3fb950]">{t('allOnTrack')}</div>
            ) : (
              <div className="space-y-1">
                {summary.tasks_overdue.slice(0, 3).map(t => (
                  <div key={t.id} className="text-xs text-[#f85149] truncate">{t.title}</div>
                ))}
              </div>
            )}
          </div>

          {/* Today's events */}
          <div className="lg:col-span-2 bg-[#161b22] border border-[#30363d] rounded-xl p-4">
            <div className="flex items-center justify-between mb-3">
              <div className="text-sm font-medium text-[#e6edf3]">{t('todaysEvents')}</div>
              <button onClick={() => onNavigate('calendar')} className="text-xs text-[#58a6ff] hover:underline">
                {t('viewAll')}
              </button>
            </div>
            {summary.events.length === 0 ? (
              <div className="text-xs text-[#8b949e]">{t('noEventsToday')}</div>
            ) : (
              <div className="space-y-2">
                {summary.events.map(ev => (
                  <div key={ev.id} className="flex items-center gap-3 p-2 bg-[#0d1117] rounded-md">
                    <div className="text-xs text-[#8b949e] w-12 flex-shrink-0">{formatTime(ev.start)}</div>
                    <div className="flex-1 min-w-0">
                      <div className="text-sm text-[#e6edf3] truncate">{ev.title}</div>
                      {ev.location && <div className="text-xs text-[#8b949e] truncate">📍 {ev.location}</div>}
                    </div>
                    {ev.end && (
                      <div className="text-xs text-[#8b949e] flex-shrink-0">
                        {Math.round((new Date(ev.end).getTime() - new Date(ev.start).getTime()) / 60000)} {t('minUnit')}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Priority tasks */}
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
            <div className="flex items-center justify-between mb-3">
              <div className="text-sm font-medium text-[#e6edf3]">{t('priority')}</div>
              <button onClick={() => onNavigate('tasks')} className="text-xs text-[#58a6ff] hover:underline">
                {t('viewAll')}
              </button>
            </div>
            {summary.priority_tasks.length === 0 ? (
              <div className="text-xs text-[#8b949e]">{t('noOpenTasks')}</div>
            ) : (
              <div className="space-y-2">
                {summary.priority_tasks.map(pt => (
                  <div key={pt.id} className="flex items-start gap-2">
                    <div className="w-2 h-2 rounded-full mt-1.5 flex-shrink-0"
                      style={{ background: PRIORITY_COLORS[pt.priority as TaskPriority] }} />
                    <div className="flex-1 min-w-0">
                      <div className="text-xs text-[#e6edf3] truncate">{pt.title}</div>
                      <div className="flex items-center gap-1 mt-0.5">
                        <span className="text-[9px] px-1 rounded"
                          style={{ background: ENERGY_COLORS[pt.energy_level as EnergyLevel] + '20',
                                   color: ENERGY_COLORS[pt.energy_level as EnergyLevel] }}>
                          {energyLabel(pt.energy_level as EnergyLevel)}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Conflicts */}
          {summary.conflicts.length > 0 && (
            <div className="lg:col-span-3 bg-[#f8514920] border border-[#f85149] rounded-xl p-4">
              <div className="text-sm font-medium text-[#f85149] mb-2">⚠ {summary.conflicts.length > 1 ? t('scheduleConflicts') : t('scheduleConflict')}</div>
              {summary.conflicts.map((c, i) => (
                <div key={i} className="text-xs text-[#e6edf3]">
                  {t('overlapText', { a: c.event_a.title, b: c.event_b.title, n: c.overlap_minutes })}
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  )
}
