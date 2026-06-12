import { useEffect, useState } from 'react'
import { usePlannerStore } from './stores/plannerStore'
import { useSettingsStore } from './stores/settingsStore'
import { api } from './lib/tauri'
import { TodayView }    from './components/Today/TodayView'
import { CalendarView } from './components/Calendar/CalendarView'
import { TasksView }    from './components/Tasks/TasksView'
import { ProjectsView } from './components/Projects/ProjectsView'
import { CaptureView }  from './components/Capture/CaptureView'
import { SearchView }   from './components/Search/SearchView'
import { SettingsView } from './components/Settings/SettingsView'

type Tab = 'today' | 'calendar' | 'tasks' | 'projects' | 'capture' | 'search' | 'settings'

export default function App() {
  const [tab, setTab] = useState<Tab>('today')
  const { loadAll, setOllamaOnline, summary } = usePlannerStore()
  const { load: loadSettings } = useSettingsStore()

  useEffect(() => {
    loadSettings()
    loadAll()
    api.checkOllama().then(setOllamaOnline).catch(() => {})
  }, [])

  const overdueCount = summary?.tasks_overdue.length ?? 0
  const conflictCount = summary?.conflicts.length ?? 0

  const nav = (id: Tab, icon: string, label: string, badge?: number) => (
    <button key={id} onClick={() => setTab(id)}
      className={`flex items-center gap-2.5 w-full px-3 py-2 rounded-md text-sm transition-colors relative
        ${tab === id ? 'bg-[#21262d] text-[#e6edf3]' : 'text-[#8b949e] hover:bg-[#161b22] hover:text-[#e6edf3]'}`}>
      <span className="text-base">{icon}</span>
      <span className="flex-1 text-left">{label}</span>
      {badge != null && badge > 0 && (
        <span className="text-xs bg-[#f85149] text-white px-1.5 py-0.5 rounded-full min-w-[20px] text-center leading-none">
          {badge > 99 ? '99+' : badge}
        </span>
      )}
    </button>
  )

  return (
    <div className="flex h-screen bg-[#0d1117] text-[#e6edf3] overflow-hidden">
      {/* Sidebar */}
      <div className="w-52 flex-shrink-0 border-r border-[#30363d] flex flex-col">
        <div className="p-4 border-b border-[#30363d]">
          <div className="flex items-center gap-2">
            <span className="text-xl">📅</span>
            <span className="font-semibold">LifePlanner</span>
          </div>
        </div>
        <nav className="flex-1 p-2 space-y-0.5 overflow-y-auto">
          {nav('today',    '🌅', 'Heute',       (overdueCount + conflictCount) || undefined)}
          {nav('calendar', '📆', 'Kalender')}
          {nav('tasks',    '✅', 'Aufgaben')}
          {nav('projects', '📁', 'Projekte')}
          {nav('capture',  '✏️', 'Erfassen')}
          {nav('search',   '🔍', 'Suche')}
          {nav('settings', '⚙️', 'Einstellungen')}
        </nav>
        <div className="p-3 border-t border-[#30363d]">
          <button onClick={loadAll}
            className="w-full text-xs text-[#8b949e] hover:text-[#e6edf3] py-1.5 rounded hover:bg-[#21262d] transition-colors">
            ↻ Aktualisieren
          </button>
        </div>
      </div>

      {/* Main */}
      <div className="flex-1 overflow-hidden">
        {tab === 'today'    && <TodayView onNavigate={setTab} />}
        {tab === 'calendar' && <CalendarView />}
        {tab === 'tasks'    && <TasksView />}
        {tab === 'projects' && <ProjectsView />}
        {tab === 'capture'  && <CaptureView onNavigate={setTab} />}
        {tab === 'search'   && <SearchView />}
        {tab === 'settings' && <SettingsView />}
      </div>
    </div>
  )
}
