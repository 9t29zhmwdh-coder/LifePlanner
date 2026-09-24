import { invoke } from '@tauri-apps/api/core'
import { getLang, t } from './i18n'

// ─── Types ──────────────────────────────────────────────────────────────────

export type EventStatus = 'tentative' | 'confirmed' | 'cancelled'
export type EventSource = 'manual' | 'extracted' | 'ics_file' | 'cal_dav'
export type TaskPriority = 'critical' | 'high' | 'medium' | 'low' | 'someday'
export type EnergyLevel = 'high' | 'medium' | 'low'
export type TaskStatus = 'todo' | 'in_progress' | 'done' | 'cancelled'
export type ProjectStatus = 'active' | 'on_hold' | 'completed' | 'cancelled'
export type CalendarKind = 'ics_file' | 'cal_dav' | 'local'

export interface RecurrenceRule {
  freq: string; interval: number; until?: string; count?: number; by_day: string[]
}

export interface CalEvent {
  id: string; title: string; description?: string
  start: string; end?: string; all_day: boolean
  location?: string; source: EventSource; calendar_id?: string
  status: EventStatus; recurrence?: RecurrenceRule
  linked_task_ids: string[]; linked_doc_ids: string[]; tags: string[]
  created_at: string; updated_at: string
}

export interface Task {
  id: string; title: string; description?: string
  due_date?: string; priority: TaskPriority; energy_level: EnergyLevel
  status: TaskStatus; project_id?: string; estimated_minutes?: number
  linked_event_ids: string[]; tags: string[]; source: string
  created_at: string; updated_at: string
}

export interface Project {
  id: string; title: string; description?: string
  status: ProjectStatus; deadline?: string
  task_ids: string[]; event_ids: string[]; tags: string[]
  auto_detected: boolean; color: string
  created_at: string; updated_at: string
}

export interface CalendarAccount {
  id: string; name: string; kind: CalendarKind
  url?: string; username?: string; ics_path?: string
  color: string; enabled: boolean; last_synced?: string
}

export interface AppSettings {
  ollama_url: string; text_model: string
  auto_extract_on_paste: boolean
  default_event_duration_minutes: number
  work_start_hour: number; work_end_hour: number
  min_free_slot_minutes: number
  default_calendar_id?: string
  calendar_accounts: CalendarAccount[]
}

export interface ExtractionResult { events: CalEvent[]; tasks: Task[]; source_text: string }
export interface PreviewConflict { event_id: string; existing_title: string; existing_start: string }
export interface ExtractionPreview { result: ExtractionResult; conflicts: PreviewConflict[] }

export interface TimeSlot { start: string; end: string; duration_minutes: number }
export interface EventConflict { event_a: CalEvent; event_b: CalEvent; overlap_minutes: number }
export interface RecurringPattern { title: string; count: number; typical_day_of_week?: number; typical_hour?: number }

export interface DailySummary {
  date: string; events: CalEvent[]; tasks_due: Task[]
  tasks_overdue: Task[]; conflicts: EventConflict[]
  free_slots: TimeSlot[]; priority_tasks: Task[]; score: number
}

export interface SearchResults { events: CalEvent[]; tasks: Task[] }

// ─── API ────────────────────────────────────────────────────────────────────

export const api = {
  // Events
  getEvents:       (from: string, to: string) => invoke<CalEvent[]>('get_events', { from, to }),
  getAllEvents:     ()                          => invoke<CalEvent[]>('get_all_events_cmd'),
  createEvent:     (event: CalEvent)           => invoke<CalEvent>('create_event', { event }),
  updateEvent:     (event: CalEvent)           => invoke<CalEvent>('update_event', { event }),
  deleteEvent:     (id: string)                => invoke<void>('delete_event_cmd', { id }),

  // Tasks
  getTasks:        (includeDone: boolean)      => invoke<Task[]>('get_tasks_cmd', { includeDone }),
  createTask:      (task: Task)                => invoke<Task>('create_task', { task }),
  updateTask:      (task: Task)                => invoke<Task>('update_task', { task }),
  setTaskStatus:   (id: string, status: TaskStatus) => invoke<void>('set_task_status', { id, status }),
  deleteTask:      (id: string)                => invoke<void>('delete_task_cmd', { id }),

  // Projects
  getProjects:     ()                          => invoke<Project[]>('get_projects_cmd'),
  createProject:   (project: Project)          => invoke<Project>('create_project', { project }),
  updateProject:   (project: Project)          => invoke<Project>('update_project', { project }),
  deleteProject:   (id: string)                => invoke<void>('delete_project_cmd', { id }),

  // Calendar
  syncCalendar:    (accountId: string)         => invoke<number>('sync_calendar', { accountId }),
  addAccount:      (account: CalendarAccount, password?: string) =>
    invoke<void>('add_calendar_account', { account, password }),
  removeAccount:   (accountId: string)         => invoke<void>('remove_calendar_account', { accountId }),

  // Extract
  extractText:     (text: string)              => invoke<ExtractionPreview>('extract_text', { text }),
  extractEmail:    (emailText: string)         => invoke<ExtractionPreview>('extract_email', { emailText }),
  // Raw bytes as the IPC body; a JSON array would be several times the file size.
  extractPdf:      (bytes: ArrayBuffer)        => invoke<ExtractionPreview>('extract_pdf', bytes),
  saveExtraction:  (result: ExtractionResult)  => invoke<number>('save_extraction', { result }),

  // Analysis
  getDailySummary: ()                          => invoke<DailySummary>('get_daily_summary'),
  getConflicts:    (from: string, to: string)  => invoke<EventConflict[]>('get_conflicts', { from, to }),
  getFreeSlots:    ()                          => invoke<TimeSlot[]>('get_free_slots'),
  getPatterns:     ()                          => invoke<RecurringPattern[]>('get_patterns'),
  search:          (query: string)             => invoke<SearchResults>('search', { query }),

  // AI
  checkOllama:     ()                          => invoke<boolean>('check_ollama'),
  generateSummary: ()                          => invoke<string>('generate_daily_summary_ai'),
  classifyTask:    (title: string, description: string) =>
    invoke<any>('ai_classify_task', { title, description }),
  aiExtract:       (text: string)              => invoke<ExtractionPreview>('ai_extract_from_text', { text }),

  // Settings
  getSettings:     ()                          => invoke<AppSettings>('get_settings'),
  saveSettings:    (settings: AppSettings)     => invoke<void>('save_settings_cmd', { settings }),
}

// ─── Helpers ────────────────────────────────────────────────────────────────

export function priorityLabel(p: TaskPriority): string {
  return { critical: t('priorityCritical'), high: t('priorityHigh'), medium: t('priorityMedium'),
    low: t('priorityLow'), someday: t('prioritySomeday') }[p]
}
export const PRIORITY_COLORS: Record<TaskPriority, string> = {
  critical: '#f85149', high: '#f0883e', medium: '#d29922', low: '#58a6ff', someday: '#8b949e',
}
export function energyLabel(e: EnergyLevel): string {
  return { high: t('energyHigh'), medium: t('energyMedium'), low: t('energyLow') }[e]
}
export const ENERGY_COLORS: Record<EnergyLevel, string> = {
  high: '#f85149', medium: '#79c0ff', low: '#3fb950',
}
export function statusLabel(s: TaskStatus): string {
  return { todo: t('statusTodo'), in_progress: t('statusInProgress'), done: t('statusDone'),
    cancelled: t('statusCancelled') }[s]
}

/** Extraction and calendars may leave the title empty; show it in the app language. */
export function titleOf(item: { title: string }): string {
  return item.title.trim() || t('untitled')
}

/** Backend errors arrive as { kind, message }; PDF errors carry a translation key. */
export function errorText(error: unknown): string {
  const message = typeof error === 'object' && error && 'message' in error
    ? String((error as { message: unknown }).message) : String(error)
  return message.startsWith('pdf') ? t(message as never) : message
}

function locale(): string {
  return getLang() === 'de' ? 'de-CH' : 'en-US'
}
export function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString(locale(), { dateStyle: 'medium', timeStyle: 'short' })
}
export function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString(locale(), { weekday: 'short', day: '2-digit', month: '2-digit' })
}
export function formatTime(iso: string): string {
  return new Date(iso).toLocaleTimeString(locale(), { hour: 'numeric', minute: '2-digit' })
}
export function dayOfWeekLabel(n: number): string {
  return [t('dayMon'), t('dayTue'), t('dayWed'), t('dayThu'), t('dayFri'), t('daySat'), t('daySun')][n] ?? '?'
}

export function newUuid(): string {
  return crypto.randomUUID()
}
