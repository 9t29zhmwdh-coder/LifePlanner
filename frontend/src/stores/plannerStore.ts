import { create } from 'zustand'
import { api, CalEvent, Task, Project, DailySummary, TimeSlot, EventConflict } from '../lib/tauri'
import { addDays, formatISO, startOfWeek } from 'date-fns'

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
        // Done tasks too: "Show completed" and project progress need them.
        api.getTasks(true),
        api.getProjects(),
        api.getFreeSlots(),
      ])
      // `events` and `conflicts` belong to the calendar week; overwriting them with
      // today's list emptied the calendar on every refresh.
      set({ summary, tasks, projects, freeSlots })
    } catch {}
    set({ loading: false })
  },

  loadWeek: async (offset = 0) => {
    // Monday to Monday, matching the week the calendar draws.
    const start = startOfWeek(addDays(new Date(), offset * 7), { weekStartsOn: 1 })
    const from = formatISO(start)
    const to = formatISO(addDays(start, 7))
    try {
      const [events, conflicts] = await Promise.all([
        api.getEvents(from, to),
        api.getConflicts(from, to),
      ])
      set({ events, conflicts })
    } catch {}
  },
}))
