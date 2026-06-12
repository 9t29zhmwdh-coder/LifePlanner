import { create } from 'zustand'
import { api, AppSettings } from '../lib/tauri'

const DEFAULTS: AppSettings = {
  ollama_url: 'http://localhost:11434',
  text_model: 'llama3',
  auto_extract_on_paste: true,
  default_event_duration_minutes: 60,
  work_start_hour: 8,
  work_end_hour: 18,
  min_free_slot_minutes: 30,
  enable_notifications: true,
  locale: 'de-CH',
  calendar_accounts: [],
}

interface SettingsStore {
  settings: AppSettings
  setSettings: (s: AppSettings) => void
  load: () => Promise<void>
  save: (s: AppSettings) => Promise<void>
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: DEFAULTS,
  setSettings: s => set({ settings: s }),
  load: async () => {
    try { set({ settings: await api.getSettings() }) } catch {}
  },
  save: async (s) => {
    await api.saveSettings(s)
    set({ settings: s })
  },
}))
