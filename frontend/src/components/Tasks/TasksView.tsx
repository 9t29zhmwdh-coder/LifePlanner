import { useState } from 'react'
import { usePlannerStore } from '../../stores/plannerStore'
import { api, PRIORITY_COLORS, PRIORITY_LABELS, ENERGY_COLORS, ENERGY_LABELS, STATUS_LABELS, formatDate, newUuid,
  type Task, type TaskPriority, type EnergyLevel, type TaskStatus } from '../../lib/tauri'

const ENERGY_GROUPS: EnergyLevel[] = ['high', 'medium', 'low']

export function TasksView() {
  const { tasks, loadAll } = usePlannerStore()
  const [filter, setFilter] = useState<'all' | EnergyLevel>('all')
  const [showDone, setShowDone] = useState(false)
  const [adding, setAdding] = useState(false)
  const [newTitle, setNewTitle] = useState('')

  const filtered = tasks.filter(t => {
    if (!showDone && (t.status === 'done' || t.status === 'cancelled')) return false
    if (filter !== 'all' && t.energy_level !== filter) return false
    return true
  })

  const handleToggleDone = async (t: Task) => {
    const next: TaskStatus = t.status === 'done' ? 'todo' : 'done'
    await api.setTaskStatus(t.id, next)
    await loadAll()
  }

  const handleAddTask = async () => {
    if (!newTitle.trim()) return
    const task: Task = {
      id: newUuid(), title: newTitle.trim(),
      priority: 'medium', energy_level: 'medium', status: 'todo',
      linked_event_ids: [], tags: [], source: 'manual',
      created_at: new Date().toISOString(), updated_at: new Date().toISOString(),
    }
    await api.createTask(task)
    setNewTitle('')
    setAdding(false)
    await loadAll()
  }

  const handleDelete = async (id: string) => {
    await api.deleteTask(id)
    await loadAll()
  }

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="p-4 border-b border-[#30363d] flex items-center gap-3 flex-wrap">
        <div className="flex gap-1">
          <FilterBtn active={filter === 'all'} onClick={() => setFilter('all')}>Alle</FilterBtn>
          {ENERGY_GROUPS.map(e => (
            <FilterBtn key={e} active={filter === e} onClick={() => setFilter(e)}
              color={ENERGY_COLORS[e]}>
              {ENERGY_LABELS[e]}
            </FilterBtn>
          ))}
        </div>
        <label className="flex items-center gap-2 text-xs text-[#8b949e] cursor-pointer ml-auto">
          <input type="checkbox" checked={showDone} onChange={e => setShowDone(e.target.checked)}
            className="accent-[#58a6ff]" />
          Erledigte anzeigen
        </label>
        <button onClick={() => setAdding(true)}
          className="px-3 py-1.5 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors">
          + Aufgabe
        </button>
      </div>

      {/* Add task inline */}
      {adding && (
        <div className="p-3 border-b border-[#30363d] flex gap-2">
          <input autoFocus value={newTitle} onChange={e => setNewTitle(e.target.value)}
            onKeyDown={e => { if (e.key === 'Enter') handleAddTask(); if (e.key === 'Escape') setAdding(false) }}
            placeholder="Neue Aufgabe…"
            className="flex-1 bg-[#21262d] border border-[#30363d] rounded px-3 py-1.5 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
          <button onClick={handleAddTask} className="px-3 text-xs bg-[#238636] text-white rounded">Hinzufügen</button>
          <button onClick={() => setAdding(false)} className="px-3 text-xs text-[#8b949e]">Abbrechen</button>
        </div>
      )}

      <div className="flex-1 overflow-y-auto">
        {filtered.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-[#8b949e] text-sm">Keine Aufgaben</div>
        ) : (
          filtered.map(t => (
            <div key={t.id}
              className={`flex items-center gap-3 px-4 py-2.5 border-b border-[#21262d] hover:bg-[#161b22] transition-colors group
                ${t.status === 'done' ? 'opacity-50' : ''}`}>
              <button onClick={() => handleToggleDone(t)}
                className={`w-4 h-4 rounded border flex-shrink-0 flex items-center justify-center transition-colors
                  ${t.status === 'done' ? 'bg-[#3fb950] border-[#3fb950]' : 'border-[#30363d] hover:border-[#58a6ff]'}`}>
                {t.status === 'done' && <span className="text-[10px] text-white">✓</span>}
              </button>

              <div className="flex-1 min-w-0">
                <div className={`text-sm ${t.status === 'done' ? 'line-through text-[#8b949e]' : 'text-[#e6edf3]'}`}>
                  {t.title}
                </div>
                <div className="flex items-center gap-2 mt-0.5">
                  <span className="text-[9px] px-1.5 py-0.5 rounded"
                    style={{ background: PRIORITY_COLORS[t.priority as TaskPriority] + '20',
                             color: PRIORITY_COLORS[t.priority as TaskPriority] }}>
                    {PRIORITY_LABELS[t.priority as TaskPriority]}
                  </span>
                  <span className="text-[9px] px-1.5 py-0.5 rounded"
                    style={{ background: ENERGY_COLORS[t.energy_level as EnergyLevel] + '20',
                             color: ENERGY_COLORS[t.energy_level as EnergyLevel] }}>
                    {ENERGY_LABELS[t.energy_level as EnergyLevel]}
                  </span>
                  {t.due_date && (
                    <span className={`text-[9px] ${new Date(t.due_date) < new Date() ? 'text-[#f85149]' : 'text-[#8b949e]'}`}>
                      Fällig: {formatDate(t.due_date)}
                    </span>
                  )}
                  {t.estimated_minutes && (
                    <span className="text-[9px] text-[#8b949e]">{t.estimated_minutes} Min</span>
                  )}
                </div>
              </div>

              <button onClick={() => handleDelete(t.id)}
                className="opacity-0 group-hover:opacity-100 text-[#8b949e] hover:text-[#f85149] text-xs transition-opacity">
                ×
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  )
}

function FilterBtn({ active, onClick, color, children }: {
  active: boolean; onClick: () => void; color?: string; children: React.ReactNode
}) {
  return (
    <button onClick={onClick}
      className={`px-2.5 py-1 text-xs rounded-full transition-colors ${active ? 'text-white' : 'text-[#8b949e] hover:text-[#e6edf3]'}`}
      style={active ? { background: color ?? '#30363d' } : {}}>
      {children}
    </button>
  )
}
