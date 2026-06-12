import { useState } from 'react'
import { usePlannerStore } from '../../stores/plannerStore'
import { api, formatTime, type CalEvent } from '../../lib/tauri'
import { addDays, startOfWeek, format, isSameDay } from 'date-fns'
import { de } from 'date-fns/locale'

const HOURS = Array.from({ length: 14 }, (_, i) => i + 7) // 07:00–20:00

export function CalendarView() {
  const { events, conflicts, loadWeek } = usePlannerStore()
  const [weekOffset, setWeekOffset] = useState(0)
  const [selected, setSelected] = useState<CalEvent | null>(null)

  const weekStart = startOfWeek(addDays(new Date(), weekOffset * 7), { weekStartsOn: 1 })
  const days = Array.from({ length: 7 }, (_, i) => addDays(weekStart, i))

  const handlePrev = () => { setWeekOffset(w => w - 1); loadWeek(weekOffset - 1) }
  const handleNext = () => { setWeekOffset(w => w + 1); loadWeek(weekOffset + 1) }

  const eventsForDay = (day: Date) =>
    events.filter(e => isSameDay(new Date(e.start), day))

  const conflictIds = new Set(conflicts.flatMap(c => [c.event_a.id, c.event_b.id]))

  return (
    <div className="h-full flex flex-col overflow-hidden">
      {/* Header */}
      <div className="p-4 border-b border-[#30363d] flex items-center gap-4">
        <button onClick={handlePrev} className="p-1 text-[#8b949e] hover:text-[#e6edf3]">◀</button>
        <span className="text-sm font-medium text-[#e6edf3]">
          {format(weekStart, 'd. MMMM', { locale: de })} – {format(addDays(weekStart, 6), 'd. MMMM yyyy', { locale: de })}
        </span>
        <button onClick={handleNext} className="p-1 text-[#8b949e] hover:text-[#e6edf3]">▶</button>
        <button onClick={() => { setWeekOffset(0); loadWeek(0) }}
          className="ml-2 text-xs text-[#58a6ff] hover:underline">Heute</button>
      </div>

      <div className="flex-1 overflow-auto">
        {/* Day headers */}
        <div className="grid grid-cols-[60px_repeat(7,1fr)] border-b border-[#30363d] sticky top-0 bg-[#0d1117] z-10">
          <div />
          {days.map(day => (
            <div key={day.toISOString()} className="p-2 text-center border-l border-[#21262d]">
              <div className="text-xs text-[#8b949e]">{format(day, 'EEE', { locale: de })}</div>
              <div className={`text-sm font-medium ${isSameDay(day, new Date()) ? 'text-[#58a6ff]' : 'text-[#e6edf3]'}`}>
                {format(day, 'd')}
              </div>
            </div>
          ))}
        </div>

        {/* Time grid */}
        <div className="grid grid-cols-[60px_repeat(7,1fr)]">
          {HOURS.map(h => (
            <>
              <div key={`h${h}`} className="p-1 text-right pr-2 text-xs text-[#8b949e] border-r border-[#21262d] pt-3">
                {h}:00
              </div>
              {days.map(day => {
                const dayEvs = eventsForDay(day).filter(e => {
                  const evH = new Date(e.start).getHours()
                  return evH === h
                })
                return (
                  <div key={`${day.toISOString()}_${h}`}
                    className="border-l border-t border-[#21262d] min-h-[48px] p-0.5 relative">
                    {dayEvs.map(ev => (
                      <button key={ev.id}
                        onClick={() => setSelected(ev)}
                        className="w-full text-left rounded px-1.5 py-0.5 text-xs truncate mb-0.5 transition-opacity hover:opacity-80"
                        style={{
                          background: conflictIds.has(ev.id) ? '#f8514930' : '#58a6ff20',
                          color: conflictIds.has(ev.id) ? '#f85149' : '#79c0ff',
                          border: `1px solid ${conflictIds.has(ev.id) ? '#f85149' : '#58a6ff'}40`,
                        }}>
                        {formatTime(ev.start)} {ev.title}
                      </button>
                    ))}
                  </div>
                )
              })}
            </>
          ))}
        </div>
      </div>

      {/* Event detail drawer */}
      {selected && (
        <div className="border-t border-[#30363d] p-4 bg-[#161b22]">
          <div className="flex justify-between items-start">
            <div>
              <div className="font-medium text-[#e6edf3]">{selected.title}</div>
              <div className="text-xs text-[#8b949e] mt-1">
                {format(new Date(selected.start), 'dd.MM.yyyy HH:mm')}
                {selected.end && ` – ${formatTime(selected.end)}`}
              </div>
              {selected.location && <div className="text-xs text-[#8b949e]">📍 {selected.location}</div>}
              {selected.description && <div className="text-xs text-[#8b949e] mt-1">{selected.description}</div>}
            </div>
            <button onClick={() => setSelected(null)} className="text-[#8b949e] text-lg leading-none">×</button>
          </div>
        </div>
      )}
    </div>
  )
}
