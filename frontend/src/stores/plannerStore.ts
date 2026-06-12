import { create } from 'zustand'
import { api, CalEvent, Task, Project, DailySummary, TimeSlot, EventConflict } from '../lib/tauri'
import { startOfDay, endOfDay, addDays, formatISO } from 'date-fns'

interface PlannerStore {
  summary: DailySummary | null
  events: CalEvent[]
  tasks: Task[]
  projects: Project[]
  freeSlots: TimeSlot[]
  conflicts: EventConflict[]
  aiText: string
  ollamaOnline: boolean
  loading: boolean

  setSummary: (s: DailySummary) => void
  setAiText: (t: string) => void
  setOllamaOnline: (v: boolean) => void
  setLoading: (v: boolean) => void
  loadAll: () => Promise<void>
  loadWeek: (offset?: number) => Promise<void>
}

export const usePlannerStore = create<PlannerStore>((set, get) => ({
  summary: null,
  events: [],
  tasks: [],
  projects: [],
  freeSlots: [],
  conflicts: [],
  aiText: '',
  ollamaOnline: false,
  loading: false,

  setSummary: s => set({ summary: s }),
  setAiText: t => set({ aiText: t }),
  setOllamaOnline: v => set({ ollamaOnline: v }),
  setLoading: v => set({ loading: v }),

  loadAll: async () => {
    set({ loading: true })
    try {
      const [summary, tasks, projects, freeSlots] = await Promise.all([
        api.getDailySummary(),
        api.getTasks(false),
        api.getProjects(),
        api.getFreeSlots(),
      ])
      set({ summary, tasks, projects, freeSlots, events: summary.events, conflicts: summary.conflicts })
    } catch {}
    set({ loading: false })
  },

  loadWeek: async (offset = 0) => {
    const base = addDays(new Date(), offset * 7)
    const from = formatISO(startOfDay(base))
    const to = formatISO(endOfDay(addDays(base, 6)))
    try {
      const [events, conflicts] = await Promise.all([
        api.getEvents(from, to),
        api.getConflicts(from, to),
      ])
      set({ events, conflicts })
    } catch {}
  },
}))
