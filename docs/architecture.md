# Architektura TaskFloat

## Przegląd

TaskFloat to natywna aplikacja macOS zbudowana w Tauri 2.0, wyświetlająca floating window z listą zadań z Taskwarrior. Okno jest zawsze na wierzchu, półprzezroczyste (glass morphism), bez standardowego titlebar.

```
┌─────────────────────────────────────────────────────────────┐
│                        macOS Desktop                         │
│                                                              │
│   ┌──────────────────────┐                                   │
│   │    TaskFloat Window   │  ← NSPanel, always-on-top        │
│   │   (SvelteKit WebView) │    frameless, transparent         │
│   │                       │    draggable, resizable           │
│   │  ┌─────────────────┐ │                                   │
│   │  │  Svelte Frontend │ │  ← Reactive UI, glass morphism   │
│   │  │                  │ │                                   │
│   │  │  invoke() ──────────────┐                             │
│   │  └─────────────────┘ │    │                              │
│   └──────────────────────┘    │  IPC (JSON)                  │
│                               │                              │
│   ┌──────────────────────┐    │                              │
│   │    Tauri Rust Core    │◄───┘                              │
│   │                       │                                   │
│   │  #[tauri::command]    │                                   │
│   │  get_tasks()          │                                   │
│   │  add_task()           │                                   │
│   │  complete_task()      │──── std::process::Command         │
│   │  modify_task()        │          │                        │
│   │  delete_task()        │          │                        │
│   │  get_projects()       │          ▼                        │
│   │                       │   ┌─────────────┐                │
│   └──────────────────────┘   │ task CLI     │                │
│                               │ (Taskwarrior)│                │
│                               └──────┬──────┘                │
│                                      │                        │
│                               ┌──────▼──────┐                │
│                               │ SQLite DB    │                │
│                               │ taskchampion │                │
│                               └─────────────┘                │
└──────────────────────────────────────────────────────────────┘
```

## Warstwy systemu

### 1. Warstwa prezentacji (SvelteKit)

**Odpowiedzialność**: Renderowanie UI, obsługa interakcji użytkownika, zarządzanie stanem.

- **Adapter**: `@sveltejs/adapter-static` (SPA, bez SSR)
- **Routing**: Jedna strona `+page.svelte` (single-view app)
- **State management**: Svelte stores (`writable`, `derived`)
- **Styling**: CSS custom properties + Tailwind CSS, glass morphism via `backdrop-filter`
- **IPC**: `invoke()` z `@tauri-apps/api/core` do wywołania Rust commands

**Komponenty**:
```
+page.svelte              # Root — layout, drag zone, polling loop
├── TitleBar.svelte       # Custom titlebar: logo, minimize, close, drag handle
├── ProjectFilter.svelte  # Dropdown/tabs z listą projektów TW
├── AddTask.svelte        # Input field + Enter = nowy task
├── TaskList.svelte       # Lista z scroll, grupowanie done/pending
│   └── TaskItem.svelte   # Checkbox, opis, projekt, priorytet, due date, akcje
└── StatusBar.svelte      # "X otwartych · Y ukończonych dziś"
```

### 2. Warstwa IPC (Tauri Commands)

**Odpowiedzialność**: Serializacja/deserializacja, walidacja, mapowanie typów.

Każdy command to `async fn` w Rust oznaczony `#[tauri::command]`:

```rust
#[tauri::command]
async fn get_tasks(filter: Option<String>) -> Result<Vec<Task>, String>

#[tauri::command]
async fn add_task(description: String, project: Option<String>, 
                  priority: Option<String>, due: Option<String>) -> Result<Task, String>

#[tauri::command]
async fn complete_task(uuid: String) -> Result<(), String>

#[tauri::command]
async fn modify_task(uuid: String, modifications: HashMap<String, String>) -> Result<Task, String>

#[tauri::command]
async fn delete_task(uuid: String) -> Result<(), String>

#[tauri::command]
async fn get_projects() -> Result<Vec<String>, String>

#[tauri::command]
async fn get_tags() -> Result<Vec<String>, String>
```

### 3. Warstwa integracji (Taskwarrior Bridge)

**Odpowiedzialność**: Wykonywanie `task` CLI, parsowanie JSON, obsługa błędów.

- Wszystko przez `std::process::Command` — zero bezpośredniego dostępu do DB
- JSON output: `task rc.json.array=on <filter> export`
- Mutacje: `task add`, `task <uuid> done`, `task <uuid> modify`, `task <uuid> delete`
- Projekty: `task _projects`
- Tagi: `task _tags`

### 4. Warstwa okna (Tauri Window Manager)

**Odpowiedzialność**: Natywne okno macOS z floating behavior.

**Konfiguracja okna** (`tauri.conf.json`):
```json
{
  "windows": [{
    "label": "main",
    "title": "TaskFloat",
    "width": 380,
    "height": 600,
    "minWidth": 300,
    "minHeight": 400,
    "resizable": true,
    "decorations": false,
    "transparent": true,
    "alwaysOnTop": true,
    "skipTaskbar": true,
    "visible": true
  }]
}
```

**Plugins**:
| Plugin | Cel | Konfiguracja |
|--------|-----|-------------|
| `tauri-plugin-autostart` | Uruchomienie przy logowaniu | `MacosLauncher::LaunchAgent` |
| `tauri-plugin-global-shortcut` | `⌥⌘T` toggle show/hide | Rejestracja w `setup()` |
| `tauri-plugin-positioner` | Zapamiętywanie pozycji okna | `Position::Center` default |

## Przepływ danych

### Odczyt tasków (polling)
```
[co 15s] Frontend setInterval()
    → invoke("get_tasks", { filter })
    → Rust: std::process::Command("task", ["rc.json.array=on", filter, "export"])
    → parse JSON → Vec<Task>
    → serialize → IPC → Frontend
    → Svelte store update → reactive re-render
```

### Dodanie taska
```
User: Enter w input
    → invoke("add_task", { description, project, priority, due })
    → Rust: Command("task", ["add", description, "project:X", ...])
    → re-fetch tasks → update store
```

### Oznaczenie jako done
```
User: click checkbox
    → invoke("complete_task", { uuid })
    → Rust: Command("task", [uuid, "done"])
    → optimistic update w store (natychmiast checkbox)
    → re-fetch w tle (potwierdzenie)
```

## Konfiguracja Tauri

### tauri.conf.json — kluczowe sekcje
```json
{
  "productName": "TaskFloat",
  "identifier": "pl.glajc.taskfloat",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  },
  "app": {
    "withGlobalTauri": true,
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"
    },
    "trayIcon": {
      "iconPath": "icons/tray.png",
      "tooltip": "TaskFloat"
    }
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "app"],
    "icon": ["icons/icon.icns"],
    "macOS": {
      "minimumSystemVersion": "13.0"
    }
  }
}
```

### Uprawnienia (capabilities)
```json
{
  "identifier": "main-capability",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "autostart:default",
    "global-shortcut:default",
    "positioner:default",
    "shell:allow-execute"
  ]
}
```

## Wymagania systemowe

| Wymaganie | Minimum |
|-----------|---------|
| macOS | 13.0 (Ventura) |
| Taskwarrior | 3.0+ |
| Rozmiar apki | ~10-15 MB |
| RAM | ~30-50 MB |
| CPU | Minimalny (polling co 15s) |

## Decyzje architektoniczne

### Dlaczego `task` CLI zamiast bezpośredniego dostępu do SQLite?
1. **Stabilność**: CLI to publiczne API Taskwarrior, format DB może się zmienić
2. **Hooks**: `task` CLI odpala hooki TW (pre/post), bezpośredni dostęp je omija
3. **Concurrent access**: TW3 (TaskChampion) obsługuje locking — CLI to respektuje
4. **Prostota**: Zero zależności od wewnętrznych struktur danych

### Dlaczego polling zamiast file watch?
1. TW3 używa SQLite, nie plików tekstowych — `fs::watch` nie złapie zmian
2. 15s polling to ~0.001% CPU — praktycznie niezauważalny koszt
3. Prostota implementacji i debugowania
4. Ewentualna migracja na TaskChampion sync protocol w przyszłości

### Dlaczego SvelteKit zamiast plain Svelte?
1. Lepszy DX (routing, layouts, built-in tooling)
2. Static adapter daje czysty SPA bez narzutu SSR
3. Przyszłościowy — łatwa migracja gdyby app urósł

### Dlaczego frameless window?
1. Custom titlebar = pełna kontrola nad wyglądem
2. Glass morphism wymaga transparent background
3. Mniejszy footprint wizualny na pulpicie
4. Natywne dekoracje psują efekt floating widget
