import { useState } from 'react'
import { useSettingsStore } from '../../stores/settingsStore'
import { api, newUuid, type CalendarAccount, type CalendarKind } from '../../lib/tauri'

const CALENDAR_KINDS: { value: CalendarKind; label: string }[] = [
  { value: 'ics_file', label: 'ICS-Datei' },
  { value: 'caldav', label: 'CalDAV' },
  { value: 'exchange', label: 'Exchange' },
  { value: 'google', label: 'Google Kalender' },
  { value: 'apple', label: 'Apple Kalender' },
]

export function SettingsView() {
  const { settings, save } = useSettingsStore()
  const [draft, setDraft] = useState(settings)
  const [testing, setTesting] = useState(false)
  const [testResult, setTestResult] = useState<'ok' | 'fail' | null>(null)
  const [saving, setSaving] = useState(false)
  const [addingCal, setAddingCal] = useState(false)
  const [newCal, setNewCal] = useState<Partial<CalendarAccount>>({ kind: 'ics_file', color: '#58a6ff', enabled: true })

  const handleSave = async () => {
    setSaving(true)
    try { await save(draft) } finally { setSaving(false) }
  }

  const handleTestOllama = async () => {
    setTesting(true)
    setTestResult(null)
    try {
      const ok = await api.checkOllama()
      setTestResult(ok ? 'ok' : 'fail')
    } catch {
      setTestResult('fail')
    } finally {
      setTesting(false)
    }
  }

  const handleAddCalendar = () => {
    if (!newCal.name) return
    const acc: CalendarAccount = {
      id: newUuid(), name: newCal.name!, kind: newCal.kind ?? 'ics_file',
      url: newCal.url, ics_path: newCal.ics_path, username: newCal.username,
      color: newCal.color ?? '#58a6ff', enabled: true,
    }
    setDraft(d => ({ ...d, calendar_accounts: [...d.calendar_accounts, acc] }))
    setNewCal({ kind: 'ics_file', color: '#58a6ff', enabled: true })
    setAddingCal(false)
  }

  const handleRemoveCal = (id: string) => {
    setDraft(d => ({ ...d, calendar_accounts: d.calendar_accounts.filter(a => a.id !== id) }))
  }

  return (
    <div className="h-full overflow-y-auto p-6 max-w-2xl">
      <h1 className="text-lg font-semibold text-[#e6edf3] mb-6">Einstellungen</h1>

      {/* KI / Ollama */}
      <Section title="Lokale KI (Ollama)">
        <Field label="Ollama URL">
          <input value={draft.ollama_url} onChange={e => setDraft(d => ({ ...d, ollama_url: e.target.value }))}
            className={input} />
        </Field>
        <Field label="Modell">
          <input value={draft.text_model} onChange={e => setDraft(d => ({ ...d, text_model: e.target.value }))}
            placeholder="llama3" className={input} />
        </Field>
        <div className="flex items-center gap-3 mt-2">
          <button onClick={handleTestOllama} disabled={testing}
            className="px-3 py-1.5 text-xs bg-[#21262d] border border-[#30363d] hover:border-[#58a6ff] text-[#e6edf3] rounded transition-colors disabled:opacity-50">
            {testing ? 'Teste…' : 'Verbindung testen'}
          </button>
          {testResult === 'ok' && <span className="text-xs text-[#3fb950]">✓ Verbunden</span>}
          {testResult === 'fail' && <span className="text-xs text-[#f85149]">✗ Nicht erreichbar</span>}
        </div>
      </Section>

      {/* Arbeitszeiten */}
      <Section title="Arbeitszeiten">
        <div className="grid grid-cols-2 gap-4">
          <Field label="Arbeitsbeginn (Stunde)">
            <input type="number" min={0} max={23} value={draft.work_start_hour}
              onChange={e => setDraft(d => ({ ...d, work_start_hour: +e.target.value }))}
              className={input} />
          </Field>
          <Field label="Arbeitsende (Stunde)">
            <input type="number" min={0} max={23} value={draft.work_end_hour}
              onChange={e => setDraft(d => ({ ...d, work_end_hour: +e.target.value }))}
              className={input} />
          </Field>
        </div>
        <Field label="Mindestlänge freies Zeitfenster (Min)">
          <input type="number" min={5} max={240} value={draft.min_free_slot_minutes}
            onChange={e => setDraft(d => ({ ...d, min_free_slot_minutes: +e.target.value }))}
            className={input} />
        </Field>
        <Field label="Standard-Terminlänge (Min)">
          <input type="number" min={5} max={480} value={draft.default_event_duration_minutes}
            onChange={e => setDraft(d => ({ ...d, default_event_duration_minutes: +e.target.value }))}
            className={input} />
        </Field>
      </Section>

      {/* Verhalten */}
      <Section title="Verhalten">
        <Toggle label="Automatisch aus Zwischenablage extrahieren"
          value={draft.auto_extract_on_paste}
          onChange={v => setDraft(d => ({ ...d, auto_extract_on_paste: v }))} />
        <Toggle label="Benachrichtigungen aktivieren"
          value={draft.enable_notifications}
          onChange={v => setDraft(d => ({ ...d, enable_notifications: v }))} />
        <Field label="Sprache / Locale">
          <select value={draft.locale} onChange={e => setDraft(d => ({ ...d, locale: e.target.value }))}
            className={input}>
            <option value="de-CH">Deutsch (Schweiz)</option>
            <option value="de-DE">Deutsch (Deutschland)</option>
            <option value="de-AT">Deutsch (Österreich)</option>
            <option value="en-US">English (US)</option>
          </select>
        </Field>
      </Section>

      {/* Kalenderkonten */}
      <Section title="Kalenderkonten">
        {draft.calendar_accounts.length > 0 && (
          <div className="space-y-2 mb-3">
            {draft.calendar_accounts.map(acc => (
              <div key={acc.id} className="flex items-center gap-3 p-2 bg-[#21262d] rounded-lg">
                <div className="w-3 h-3 rounded-full flex-shrink-0" style={{ background: acc.color ?? '#58a6ff' }} />
                <div className="flex-1 min-w-0">
                  <div className="text-xs font-medium text-[#e6edf3] truncate">{acc.name}</div>
                  <div className="text-[9px] text-[#8b949e]">{CALENDAR_KINDS.find(k => k.value === acc.kind)?.label}</div>
                </div>
                <button onClick={() => handleRemoveCal(acc.id)}
                  className="text-[#8b949e] hover:text-[#f85149] text-xs">×</button>
              </div>
            ))}
          </div>
        )}

        {!addingCal ? (
          <button onClick={() => setAddingCal(true)}
            className="w-full px-3 py-2 text-xs border border-dashed border-[#30363d] hover:border-[#58a6ff] text-[#8b949e] hover:text-[#58a6ff] rounded-lg transition-colors">
            + Kalender hinzufügen
          </button>
        ) : (
          <div className="border border-[#30363d] rounded-lg p-3 space-y-2">
            <Field label="Name">
              <input value={newCal.name ?? ''} onChange={e => setNewCal(c => ({ ...c, name: e.target.value }))}
                placeholder="Mein Kalender" className={input} />
            </Field>
            <Field label="Typ">
              <select value={newCal.kind} onChange={e => setNewCal(c => ({ ...c, kind: e.target.value as CalendarKind }))}
                className={input}>
                {CALENDAR_KINDS.map(k => <option key={k.value} value={k.value}>{k.label}</option>)}
              </select>
            </Field>
            {newCal.kind === 'ics_file' ? (
              <Field label="Dateipfad">
                <input value={newCal.ics_path ?? ''} onChange={e => setNewCal(c => ({ ...c, ics_path: e.target.value }))}
                  placeholder="/Users/me/kalender.ics" className={input} />
              </Field>
            ) : (
              <>
                <Field label="URL">
                  <input value={newCal.url ?? ''} onChange={e => setNewCal(c => ({ ...c, url: e.target.value }))}
                    placeholder="https://dav.example.com/calendars/me/" className={input} />
                </Field>
                <Field label="Benutzername">
                  <input value={newCal.username ?? ''} onChange={e => setNewCal(c => ({ ...c, username: e.target.value }))}
                    className={input} />
                </Field>
              </>
            )}
            <Field label="Farbe">
              <input type="color" value={newCal.color ?? '#58a6ff'}
                onChange={e => setNewCal(c => ({ ...c, color: e.target.value }))}
                className="w-10 h-8 rounded bg-transparent cursor-pointer" />
            </Field>
            <div className="flex gap-2 pt-1">
              <button onClick={handleAddCalendar} className="px-3 py-1 text-xs bg-[#238636] text-white rounded">Hinzufügen</button>
              <button onClick={() => setAddingCal(false)} className="px-3 py-1 text-xs text-[#8b949e]">Abbrechen</button>
            </div>
          </div>
        )}
      </Section>

      {/* Save */}
      <div className="mt-6">
        <button onClick={handleSave} disabled={saving}
          className="px-6 py-2.5 text-sm bg-[#238636] hover:bg-[#2ea043] text-white rounded-lg transition-colors disabled:opacity-50">
          {saving ? 'Speichern…' : 'Einstellungen speichern'}
        </button>
      </div>
    </div>
  )
}

const input = 'w-full bg-[#21262d] border border-[#30363d] rounded px-3 py-1.5 text-sm text-[#e6edf3] focus:outline-none focus:border-[#58a6ff]'

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="mb-6">
      <div className="text-xs font-semibold text-[#8b949e] uppercase tracking-wide mb-3 pb-1 border-b border-[#21262d]">
        {title}
      </div>
      <div className="space-y-3">{children}</div>
    </div>
  )
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div>
      <label className="block text-xs text-[#8b949e] mb-1">{label}</label>
      {children}
    </div>
  )
}

function Toggle({ label, value, onChange }: { label: string; value: boolean; onChange: (v: boolean) => void }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-sm text-[#e6edf3]">{label}</span>
      <button onClick={() => onChange(!value)}
        className={`w-10 h-5 rounded-full transition-colors relative ${value ? 'bg-[#238636]' : 'bg-[#30363d]'}`}>
        <span className={`absolute top-0.5 w-4 h-4 bg-white rounded-full transition-transform ${value ? 'translate-x-5' : 'translate-x-0.5'}`} />
      </button>
    </div>
  )
}
