# UI/UX Design — TaskFloat

## Filozofia designu

Minimalistyczny floating widget, który wygląda jak natywny element macOS, ale z nowoczesnym glass morphism. Nie przeszkadza w pracy, ale jest zawsze widoczny i dostępny.

## Wireframe

```
┌─────────────────────────────────────────┐
│ ≡  TaskFloat                    ─  ✕   │  ← custom titlebar, drag zone
│─────────────────────────────────────────│
│  📋  Wszystkie  │ wywozik │ seo │ +2 ▼ │  ← projekt tabs/pills
│─────────────────────────────────────────│
│  ＋ Nowy task...                    ⏎  │  ← input, Enter dodaje
│─────────────────────────────────────────│
│                                         │
│  🔴 Dokończyć ofertę ZBM               │  priority:H
│     wlasnadomena.pl · za 2 dni          │
│                                         │
│  🟡 Deploy Śląski Magnat MVP            │  priority:M
│     hosting                             │
│                                         │
│  🟡 KFS dokumenty do PUP               │  priority:M
│     elzbietanki · jutro                 │
│                                         │
│  ○  NIS2 gap analysis                   │  no priority
│     seo                                 │
│                                         │
│  ○  SEO Pulse wireframe                 │  no priority
│     seo · za 5 dni                      │
│                                         │
│  ─ ─ ─ Ukończone dziś ─ ─ ─ ─ ─ ─ ─  │
│                                         │
│  ✅ Faktury marzec                      │  strikethrough, faded
│     fv · 14:23                          │
│                                         │
│─────────────────────────────────────────│
│  5 otwartych · 1 ukończony dziś    ⚙  │  ← status bar + settings
└─────────────────────────────────────────┘
```

## Wymiary okna

| Parametr | Wartość | Uzasadnienie |
|----------|---------|-------------|
| Domyślna szerokość | 380px | Mieści tekst bez scrollowania horyzontalnego |
| Domyślna wysokość | 600px | ~8-10 tasków widocznych |
| Min szerokość | 300px | Czytelność na małym rozmiarze |
| Min wysokość | 400px | Minimum 5 tasków widocznych |
| Max szerokość | 600px | Nie zajmuje za dużo pulpitu |

## Paleta kolorów

### Dark theme (domyślny)

```css
:root {
  /* Tło — glass morphism */
  --bg-primary: rgba(30, 30, 30, 0.78);       /* główne tło okna */
  --bg-secondary: rgba(45, 45, 45, 0.60);     /* karty, inputy */
  --bg-hover: rgba(60, 60, 60, 0.50);         /* hover na elementach */
  --bg-active: rgba(70, 70, 70, 0.50);        /* active/pressed */
  
  /* Tekst */
  --text-primary: rgba(255, 255, 255, 0.92);  /* główny tekst */
  --text-secondary: rgba(255, 255, 255, 0.55);/* meta info, projekty */
  --text-muted: rgba(255, 255, 255, 0.30);    /* disabled, placeholder */
  --text-done: rgba(255, 255, 255, 0.25);     /* completed tasks */
  
  /* Akcenty — priorytety */
  --priority-high: #FF453A;     /* czerwony — Apple system red */
  --priority-medium: #FFD60A;   /* żółty — Apple system yellow */
  --priority-low: #30D158;      /* zielony — Apple system green */
  --priority-none: rgba(255, 255, 255, 0.20);
  
  /* Akcent — interakcje */
  --accent: #0A84FF;            /* Apple system blue */
  --accent-hover: #409CFF;
  
  /* Obramowania */
  --border: rgba(255, 255, 255, 0.08);
  --border-focus: rgba(10, 132, 255, 0.50);
  
  /* Glass effect */
  --blur-radius: 40px;
  --glass-border: 1px solid rgba(255, 255, 255, 0.12);
}
```

### Light theme (przyszłość)
Planowany, ale nie w MVP. Dark theme to standard dla floating widgetów na macOS.

## Komponenty

### 1. TitleBar

```
┌─────────────────────────────────────┐
│ ≡  TaskFloat                  ─  ✕ │
└─────────────────────────────────────┘
```

- **Cały element**: Drag zone (`data-tauri-drag-region`)
- **≡**: Menu hamburger (przyszłość: settings, filtry)
- **TaskFloat**: Nazwa app, font-weight: 500
- **─**: Minimize (hide window, dostępny przez tray/hotkey)
- **✕**: Close (minimize to tray, nie quit)
- **Wysokość**: 36px
- **Background**: Nieco ciemniejsze niż reszta okna

### 2. ProjectFilter

```
┌─────────────────────────────────────────┐
│  📋  Wszystkie  │ wywozik │ seo │ +2 ▼ │
└─────────────────────────────────────────┘
```

- **Horizontal scroll** z pill-shaped buttons
- **"Wszystkie"**: Zawsze pierwszy, pokazuje count
- **Projekty**: Posortowane wg ilości pending tasków (desc)
- **"+N ▼"**: Dropdown z resztą projektów jeśli >4 widocznych
- **Active pill**: Background `--accent`, biały tekst
- **Inactive pill**: Background `--bg-secondary`, szary tekst
- **Klik**: Filtruje listę tasków

### 3. AddTask

```
┌─────────────────────────────────────┐
│  ＋ Nowy task...                 ⏎  │
└─────────────────────────────────────┘
```

- **Input**: Placeholder "Nowy task...", pełna szerokość
- **Enter**: Dodaje task do aktywnego projektu (z ProjectFilter)
- **Shift+Enter**: Otwiera expanded form (projekt, priorytet, due date)
- **Escape**: Czyści input
- **⏎ icon**: Fade-in gdy input niepusty, klik = submit
- **Background**: `--bg-secondary`, border `--border`
- **Focus**: Border `--border-focus`, subtle glow

### 4. TaskItem

```
┌─────────────────────────────────────┐
│  🔴 Dokończyć ofertę ZBM           │
│     wlasnadomena.pl · za 2 dni      │
└─────────────────────────────────────┘
```

**Layout**:
- **Linia 1**: Priority dot + description
- **Linia 2**: Project name (muted) + due date (relative)
- **Hover**: Background `--bg-hover`, pojawia się menu kontekstowe (edit, delete)
- **Klik na checkbox**: Toggle done z animacją

**Priority indicator**:
- Kolorowa kropka (8px) po lewej stronie
- `H` → `--priority-high` (czerwony)
- `M` → `--priority-medium` (żółty)
- `L` → `--priority-low` (zielony)
- brak → `--priority-none` (szary, mniejsza)

**Due date display**:
- Przyszłość: "za 2 dni", "za tydzień", "15 kwi"
- Dziś: "dziś" (pogrubione, accent color)
- Przeterminowane: "2 dni temu" (czerwony tekst)
- Brak: nie wyświetla się

**Animacja complete**:
```
1. Checkbox: empty circle → checkmark (spring animation, 200ms)
2. Description: fade + strikethrough (300ms ease-out)
3. Card: slide down do sekcji "done" (400ms ease-in-out)
4. Po 2s: task przesuwa się do sekcji "Ukończone dziś"
```

**Completed task**:
```
┌─────────────────────────────────────┐
│  ✅ Faktury marzec                  │  ← strikethrough, opacity: 0.4
│     fv · 14:23                      │  ← czas ukończenia zamiast due
└─────────────────────────────────────┘
```

### 5. TaskList

- **Scroll**: Vertical, custom scrollbar (thin, subtle)
- **Sortowanie**: Wg urgency (TW liczy automatycznie) desc → priorytet + due date
- **Grupowanie**:
  1. Pending tasks (posortowane wg urgency)
  2. Separator "Ukończone dziś"
  3. Completed today (fade, max 5 widocznych)
- **Empty state**: "Brak tasków" + emoji (party popper jeśli wszystko done)

### 6. StatusBar

```
┌─────────────────────────────────────┐
│  5 otwartych · 1 ukończony dziś  ⚙ │
└─────────────────────────────────────┘
```

- **Counters**: Dynamiczne, reactive
- **⚙ Settings**: Otwiera panel ustawień (autostart, hotkey, polling interval)
- **Wysokość**: 32px
- **Font**: 11px, `--text-secondary`

## Glass Morphism — implementacja CSS

```css
.taskfloat-window {
  background: var(--bg-primary);
  backdrop-filter: blur(var(--blur-radius));
  -webkit-backdrop-filter: blur(var(--blur-radius));
  border: var(--glass-border);
  border-radius: 12px;
  box-shadow: 
    0 8px 32px rgba(0, 0, 0, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
  overflow: hidden;
}
```

**Uwaga**: `backdrop-filter` w Tauri WebView działa na macOS gdy okno ma `transparent: true`. Blur łapie contentu pod oknem (pulpit, inne okna).

## Interakcje i animacje

### Drag
- Cały titlebar = drag zone
- Cursor: `grab` → `grabbing`
- Tauri: `data-tauri-drag-region` attribute

### Resize
- Standardowy resize z Tauri (handles na krawędziach)
- Min/max constraints w `tauri.conf.json`

### Hover effects
- Task items: subtle background change (100ms)
- Buttons: opacity/color transition (150ms)
- Checkboxes: scale pulse on hover (1.1x)

### Keyboard shortcuts
| Skrót | Akcja |
|-------|-------|
| `⌥⌘T` | Toggle show/hide (global) |
| `⌘N` | Focus na input "Nowy task" |
| `Escape` | Clear input / close expanded form |
| `⌘,` | Otwórz settings |
| `↑/↓` | Navigate tasks |
| `Space` | Toggle selected task done/pending |

## Responsywność

Okno jest resizable, więc UI musi się adaptować:

| Szerokość | Adaptacja |
|-----------|-----------|
| 300-350px | Ukryj project pills, pokaż dropdown. Due date: krótki format |
| 350-450px | Standard layout z pills |
| 450-600px | Wider cards, task meta na jednej linii z description |

## Tray icon

- **Ikona**: Checkmark w kółku, monochromatyczny (macOS template image)
- **Klik lewym**: Toggle okno show/hide
- **Klik prawym**: Menu (Show/Hide, Settings, Quit)
- **Badge**: Opcjonalnie — liczba pending tasków

## Fonty

- **Primary**: System font (`-apple-system`, `BlinkMacSystemFont`) — natywny macOS look
- **Monospace** (opcja w settings): `JetBrains Mono` lub `SF Mono` — dla geekowskiego vibe
- **Sizes**: 
  - Title bar: 13px, weight 500
  - Task description: 13px, weight 400
  - Meta info: 11px, weight 400
  - Status bar: 11px, weight 400
