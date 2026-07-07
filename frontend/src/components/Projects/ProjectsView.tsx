import { useState } from 'react'
import { usePlannerStore } from '../../stores/plannerStore'
import { api, newUuid, formatDate, PRIORITY_COLORS, priorityLabel, type Project, type TaskPriority } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

export function ProjectsView() {
  const { projects, tasks, loadAll } = usePlannerStore()
  const [expanded, setExpanded] = useState<string | null>(null)
  const [adding, setAdding] = useState(false)
  const [newName, setNewName] = useState('')
  const [newDesc, setNewDesc] = useState('')
  const t = useT()

  const tasksByProject = (pid: string) => tasks.filter(t => t.project_id === pid)

  const handleAdd = async () => {
    if (!newName.trim()) return
    const p: Project = {
      id: newUuid(), title: newName.trim(), description: newDesc.trim() || undefined,
      status: 'active', task_ids: [], event_ids: [], tags: [],
      auto_detected: false, color: '#58a6ff',
      created_at: new Date().toISOString(), updated_at: new Date().toISOString(),
    }
    await api.createProject(p)
    setNewName('')
    setNewDesc('')
    setAdding(false)
    await loadAll()
  }

  const handleDelete = async (id: string) => {
    await api.deleteProject(id)
    await loadAll()
  }

  const statusColor = (s: string) =>
    s === 'active' ? '#3fb950' : s === 'on_hold' ? '#d29922' : s === 'done' ? '#8b949e' : '#8b949e'

  const statusLabel = (s: string) =>
    s === 'active' ? t('statusActive') : s === 'on_hold' ? t('statusOnHold') : s === 'done' ? t('statusCompleted') : s

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="p-4 border-b border-[#30363d] flex items-center justify-between">
        <div className="text-sm font-medium text-[#e6edf3]">{t('projectsTitle', { n: projects.length })}</div>
        <button onClick={() => setAdding(true)}
          className="px-3 py-1.5 text-xs bg-[#238636] hover:bg-[#2ea043] text-white rounded-md transition-colors">
          {t('addProject')}
        </button>
      </div>

      {adding && (
        <div className="p-3 border-b border-[#30363d] space-y-2">
          <input autoFocus value={newName} onChange={e => setNewName(e.target.value)}
            onKeyDown={e => { if (e.key === 'Enter') handleAdd(); if (e.key === 'Escape') setAdding(false) }}
            placeholder={t('projectNamePlaceholder')}
            className="w-full bg-[#21262d] border border-[#30363d] rounded px-3 py-1.5 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
          <input value={newDesc} onChange={e => setNewDesc(e.target.value)}
            placeholder={t('descriptionOptionalPlaceholder')}
            className="w-full bg-[#21262d] border border-[#30363d] rounded px-3 py-1.5 text-xs text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]" />
          <div className="flex gap-2">
            <button onClick={handleAdd} className="px-3 py-1 text-xs bg-[#238636] text-white rounded">{t('create')}</button>
            <button onClick={() => setAdding(false)} className="px-3 py-1 text-xs text-[#8b949e]">{t('cancel')}</button>
          </div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {projects.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-[#8b949e] text-sm">{t('noProjects')}</div>
        ) : (
          projects.map(p => {
            const pts = tasksByProject(p.id)
            const done = pts.filter(t => t.status === 'done').length
            const pct = pts.length > 0 ? Math.round((done / pts.length) * 100) : 0
            const open = pts.filter(t => t.status !== 'done' && t.status !== 'cancelled')

            return (
              <div key={p.id} className="bg-[#161b22] border border-[#30363d] rounded-xl overflow-hidden">
                <button onClick={() => setExpanded(expanded === p.id ? null : p.id)}
                  className="w-full flex items-center gap-3 p-4 hover:bg-[#21262d] transition-colors text-left">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-medium text-[#e6edf3] truncate">{p.title}</span>
                      <span className="text-[9px] px-1.5 py-0.5 rounded-full"
                        style={{ background: statusColor(p.status) + '20', color: statusColor(p.status) }}>
                        {statusLabel(p.status)}
                      </span>
                    </div>
                    {p.description && (
                      <div className="text-xs text-[#8b949e] mt-0.5 truncate">{p.description}</div>
                    )}
                    <div className="flex items-center gap-3 mt-2">
                      <div className="flex-1 bg-[#0d1117] rounded-full h-1">
                        <div className="h-1 rounded-full bg-[#3fb950] transition-all"
                          style={{ width: `${pct}%` }} />
                      </div>
                      <span className="text-xs text-[#8b949e] flex-shrink-0">{t('tasksCount', { done, total: pts.length })}</span>
                    </div>
                  </div>
                  <span className="text-[#8b949e] text-sm">{expanded === p.id ? '▲' : '▼'}</span>
                </button>

                {expanded === p.id && (
                  <div className="border-t border-[#21262d]">
                    {open.length === 0 ? (
                      <div className="p-3 text-xs text-[#8b949e]">{t('noOpenTasks')}</div>
                    ) : (
                      open.map(task => (
                        <div key={task.id} className="flex items-center gap-2 px-4 py-2 border-b border-[#21262d] last:border-b-0">
                          <div className="w-2 h-2 rounded-full flex-shrink-0"
                            style={{ background: PRIORITY_COLORS[task.priority as TaskPriority] }} />
                          <span className="flex-1 text-xs text-[#e6edf3] truncate">{task.title}</span>
                          {task.due_date && (
                            <span className={`text-[9px] ${new Date(task.due_date) < new Date() ? 'text-[#f85149]' : 'text-[#8b949e]'}`}>
                              {formatDate(task.due_date)}
                            </span>
                          )}
                          <span className="text-[9px] px-1 rounded"
                            style={{ background: PRIORITY_COLORS[task.priority as TaskPriority] + '20',
                                     color: PRIORITY_COLORS[task.priority as TaskPriority] }}>
                            {priorityLabel(task.priority as TaskPriority)}
                          </span>
                        </div>
                      ))
                    )}
                    <div className="flex justify-end p-2">
                      <button onClick={() => handleDelete(p.id)}
                        className="text-[9px] text-[#8b949e] hover:text-[#f85149] px-2 py-1 rounded transition-colors">
                        {t('deleteProject')}
                      </button>
                    </div>
                  </div>
                )}
              </div>
            )
          })
        )}
      </div>
    </div>
  )
}
