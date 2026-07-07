import { create } from 'zustand'

export type Lang = 'en' | 'de'

const STORAGE_KEY = 'lifeplanner_lang'

let currentLang: Lang = (localStorage.getItem(STORAGE_KEY) as Lang) || 'en'

export function getLang(): Lang {
  return currentLang
}

interface LangState {
  lang: Lang
  setLang: (l: Lang) => void
  toggle: () => void
}

export const useLangStore = create<LangState>((set) => ({
  lang: currentLang,
  setLang: (l) => {
    currentLang = l
    localStorage.setItem(STORAGE_KEY, l)
    set({ lang: l })
  },
  toggle: () => {
    const next: Lang = currentLang === 'en' ? 'de' : 'en'
    currentLang = next
    localStorage.setItem(STORAGE_KEY, next)
    set({ lang: next })
  },
}))

const translations = {
  en: {
    // Nav
    navToday: 'Today', navCalendar: 'Calendar', navTasks: 'Tasks', navProjects: 'Projects',
    navCapture: 'Capture', navSearch: 'Search', navSettings: 'Settings', refresh: 'Refresh',

    // Labels shared across views
    priorityCritical: 'Critical', priorityHigh: 'High', priorityMedium: 'Medium', priorityLow: 'Low', prioritySomeday: 'Someday',
    energyHigh: 'Focus', energyMedium: 'Creative', energyLow: 'Routine',
    statusTodo: 'Open', statusInProgress: 'In progress', statusDone: 'Done', statusCancelled: 'Cancelled',
    dayMon: 'Mon', dayTue: 'Tue', dayWed: 'Wed', dayThu: 'Thu', dayFri: 'Fri', daySat: 'Sat', daySun: 'Sun',
    add: 'Add', cancel: 'Cancel', due: 'Due', minUnit: 'min', today: 'Today', viewAll: 'All →',

    // Today
    todayOverview: 'Today at a glance',
    aiUnavailable: 'AI unavailable. Make sure Ollama is running.',
    aiReady: 'AI ready', aiOffline: 'AI offline',
    generating: 'Generating…', aiSummary: 'AI Summary',
    loadingDay: 'Loading day…', dailyScore: 'Daily score',
    eventsAndTasks: '{{events}} appointments · {{tasks}} tasks',
    conflict: 'conflict', conflicts: 'conflicts',
    freeSlots: 'Free time slots', noFreeSlots: 'No free slots',
    overdue: 'Overdue', allOnTrack: '✓ All on track',
    todaysEvents: "Today's appointments", noEventsToday: 'No appointments today',
    priority: 'Priority', noOpenTasks: 'No open tasks',
    scheduleConflict: 'Schedule conflict', scheduleConflicts: 'Schedule conflicts',
    overlapText: '"{{a}}" and "{{b}}" overlap by {{n}} min.',

    // Tasks
    filterAll: 'All', showDone: 'Show completed', addTask: '+ Task',
    newTaskPlaceholder: 'New task…', noTasks: 'No tasks',

    // Projects
    projectsTitle: 'Projects ({{n}})', addProject: '+ Project',
    projectNamePlaceholder: 'Project name…', descriptionOptionalPlaceholder: 'Description (optional)…',
    create: 'Create', noProjects: 'No projects', tasksCount: '{{done}}/{{total}} tasks',
    deleteProject: 'Delete project',
    statusActive: 'Active', statusOnHold: 'Paused', statusCompleted: 'Completed',

    // Capture
    captureTitle: 'Capture text',
    captureSubtitle: 'Paste emails, notes or text, appointments and tasks will be detected automatically.',
    textPlaceholder: 'Paste text here…\n\nExample:\nMeeting tomorrow at 2pm with the team.\nSubmit the report by Friday.\nNext Monday: prepare the presentation.',
    detecting: 'Detecting…', detect: '🔍 Detect',
    ollamaNotAvailable: 'Ollama not available', aiAnalyzing: 'AI analyzing…', aiDetection: '✨ AI Detection',
    clear: 'Clear', noItemsDetected: 'No appointments or tasks detected.',
    detectedEvents: 'Detected appointments ({{n}})', detectedTasks: 'Detected tasks ({{n}})',
    saveAll: 'Save & apply all', saved: '✓ Saved', goToToday: 'Go to Today tab →',

    // Search
    searchPlaceholder: 'Search appointments, tasks and projects…',
    enterSearchTerm: 'Enter a search term…',
    fullTextSearchHint: 'Full-text search across all appointments and tasks',
    noResultsFor: 'No results for "{{q}}"',
    resultOne: '{{n}} result', resultOther: '{{n}} results',
    events: 'Appointments ({{n}})', tasks: 'Tasks ({{n}})',

    // Settings
    settingsTitle: 'Settings', localAiSection: 'Local AI (Ollama)',
    ollamaUrl: 'Ollama URL', model: 'Model', testing: 'Testing…', testConnection: 'Test connection',
    connected: '✓ Connected', notReachable: '✗ Not reachable',
    workingHoursSection: 'Working Hours', workStart: 'Work start (hour)', workEnd: 'Work end (hour)',
    minFreeSlot: 'Minimum free slot length (min)', defaultEventDuration: 'Default appointment length (min)',
    behaviorSection: 'Behavior', autoExtract: 'Automatically extract from clipboard',
    enableNotifications: 'Enable notifications', languageLocale: 'Date/time locale',
    calendarAccountsSection: 'Calendar Accounts', addCalendar: '+ Add calendar',
    name: 'Name', type: 'Type', filePath: 'File path', url: 'URL', username: 'Username', color: 'Color',
    saving: 'Saving…', saveSettings: 'Save settings', calendarNamePlaceholder: 'My calendar',
    calIcsFile: 'ICS file', calCaldav: 'CalDAV',
  },
  de: {
    navToday: 'Heute', navCalendar: 'Kalender', navTasks: 'Aufgaben', navProjects: 'Projekte',
    navCapture: 'Erfassen', navSearch: 'Suche', navSettings: 'Einstellungen', refresh: 'Aktualisieren',

    priorityCritical: 'Kritisch', priorityHigh: 'Hoch', priorityMedium: 'Mittel', priorityLow: 'Niedrig', prioritySomeday: 'Irgendwann',
    energyHigh: 'Fokus', energyMedium: 'Kreativ', energyLow: 'Routine',
    statusTodo: 'Offen', statusInProgress: 'In Arbeit', statusDone: 'Erledigt', statusCancelled: 'Abgebrochen',
    dayMon: 'Mo', dayTue: 'Di', dayWed: 'Mi', dayThu: 'Do', dayFri: 'Fr', daySat: 'Sa', daySun: 'So',
    add: 'Hinzufügen', cancel: 'Abbrechen', due: 'Fällig', minUnit: 'Min', today: 'Heute', viewAll: 'Alle →',

    todayOverview: 'Heute im Überblick',
    aiUnavailable: 'KI nicht verfügbar. Stelle sicher, dass Ollama läuft.',
    aiReady: 'KI bereit', aiOffline: 'KI offline',
    generating: 'Generiere…', aiSummary: 'KI-Zusammenfassung',
    loadingDay: 'Lade Tagesdaten…', dailyScore: 'Tages-Score',
    eventsAndTasks: '{{events}} Termine · {{tasks}} Aufgaben',
    conflict: 'Konflikt', conflicts: 'Konflikte',
    freeSlots: 'Freie Zeitfenster', noFreeSlots: 'Keine freien Fenster',
    overdue: 'Überfällig', allOnTrack: '✓ Alles im Plan',
    todaysEvents: 'Heutige Termine', noEventsToday: 'Keine Termine heute',
    priority: 'Priorität', noOpenTasks: 'Keine offenen Aufgaben',
    scheduleConflict: 'Terminkonflikt', scheduleConflicts: 'Terminkonflikte',
    overlapText: '"{{a}}" und "{{b}}" überschneiden sich um {{n}} Min.',

    filterAll: 'Alle', showDone: 'Erledigte anzeigen', addTask: '+ Aufgabe',
    newTaskPlaceholder: 'Neue Aufgabe…', noTasks: 'Keine Aufgaben',

    projectsTitle: 'Projekte ({{n}})', addProject: '+ Projekt',
    projectNamePlaceholder: 'Projektname…', descriptionOptionalPlaceholder: 'Beschreibung (optional)…',
    create: 'Erstellen', noProjects: 'Keine Projekte', tasksCount: '{{done}}/{{total}} Aufgaben',
    deleteProject: 'Projekt löschen',
    statusActive: 'Aktiv', statusOnHold: 'Pausiert', statusCompleted: 'Abgeschlossen',

    captureTitle: 'Text erfassen',
    captureSubtitle: 'Füge E-Mails, Notizen oder Texte ein, Termine und Aufgaben werden automatisch erkannt.',
    textPlaceholder: 'Text hier einfügen…\n\nBeispiel:\nMeeting morgen um 14:00 Uhr mit dem Team.\nBericht bis Freitag einreichen.\nNächsten Montag: Präsentation vorbereiten.',
    detecting: 'Erkenne…', detect: '🔍 Erkennen',
    ollamaNotAvailable: 'Ollama nicht verfügbar', aiAnalyzing: 'KI analysiert…', aiDetection: '✨ KI-Erkennung',
    clear: 'Leeren', noItemsDetected: 'Keine Termine oder Aufgaben erkannt.',
    detectedEvents: 'Erkannte Termine ({{n}})', detectedTasks: 'Erkannte Aufgaben ({{n}})',
    saveAll: 'Alle speichern & übernehmen', saved: '✓ Gespeichert', goToToday: 'Zum Heute-Tab →',

    searchPlaceholder: 'Termine, Aufgaben und Projekte durchsuchen…',
    enterSearchTerm: 'Suchbegriff eingeben…',
    fullTextSearchHint: 'Volltextsuche über alle Termine und Aufgaben',
    noResultsFor: 'Keine Ergebnisse für "{{q}}"',
    resultOne: '{{n}} Ergebnis', resultOther: '{{n}} Ergebnisse',
    events: 'Termine ({{n}})', tasks: 'Aufgaben ({{n}})',

    settingsTitle: 'Einstellungen', localAiSection: 'Lokale KI (Ollama)',
    ollamaUrl: 'Ollama URL', model: 'Modell', testing: 'Teste…', testConnection: 'Verbindung testen',
    connected: '✓ Verbunden', notReachable: '✗ Nicht erreichbar',
    workingHoursSection: 'Arbeitszeiten', workStart: 'Arbeitsbeginn (Stunde)', workEnd: 'Arbeitsende (Stunde)',
    minFreeSlot: 'Mindestlänge freies Zeitfenster (Min)', defaultEventDuration: 'Standard-Terminlänge (Min)',
    behaviorSection: 'Verhalten', autoExtract: 'Automatisch aus Zwischenablage extrahieren',
    enableNotifications: 'Benachrichtigungen aktivieren', languageLocale: 'Datum-/Zeit-Locale',
    calendarAccountsSection: 'Kalenderkonten', addCalendar: '+ Kalender hinzufügen',
    name: 'Name', type: 'Typ', filePath: 'Dateipfad', url: 'URL', username: 'Benutzername', color: 'Farbe',
    saving: 'Speichern…', saveSettings: 'Einstellungen speichern', calendarNamePlaceholder: 'Mein Kalender',
    calIcsFile: 'ICS-Datei', calCaldav: 'CalDAV',
  },
} as const

type TranslationKey = keyof typeof translations.en

function interpolate(str: string, vars?: Record<string, string | number>): string {
  if (!vars) return str
  return str.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''))
}

export function t(key: TranslationKey, vars?: Record<string, string | number>): string {
  const str = translations[currentLang][key] ?? key
  return interpolate(str, vars)
}

export function useT() {
  const lang = useLangStore((s) => s.lang)
  return (key: TranslationKey, vars?: Record<string, string | number>) => {
    const str = translations[lang][key] ?? key
    return interpolate(str, vars)
  }
}
