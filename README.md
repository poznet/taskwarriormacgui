# TaskFloat

A floating task manager for macOS that sits on top of your windows. Powered by [Taskwarrior](https://taskwarrior.org/) as the backend engine.

Built with **Tauri 2.0** + **SvelteKit** + **Svelte 5**.

## Features

- **Floating window** — frameless, transparent, always-on-top with glass morphism UI
- **Taskwarrior integration** — all tasks stored in Taskwarrior, no vendor lock-in
- **List & Kanban views** — switch between classic list and project-based kanban board
- **Inline editing** — double-click any task to edit description, project, priority, due date
- **Quick-add syntax** — `p:project +H due:tomorrow` parsed automatically
- **Search** — `Cmd+F` to filter tasks by description, project, or tags
- **Urgency heatmap** — left border color intensity reflects Taskwarrior urgency score
- **Due date notifications** — native macOS notifications for overdue and due-today tasks
- **Annotations** — expandable notes per task, stored as Taskwarrior annotations
- **Pomodoro timer** — built-in 25/5 timer with notification on completion
- **Progress bar** — daily completion ratio at a glance
- **Configurable** — opacity, accent color, compact mode, poll interval, autostart
- **Keyboard shortcuts** — `Alt+Cmd+T` global toggle, `Cmd+F` search
- **Remembers window size** — resizes persist across restarts
- **Tray icon** — left-click toggle, right-click menu

## Screenshots

*TODO: Add screenshots*

## Requirements

- **macOS** 13.0+
- **Taskwarrior 3.x** (tested with 3.4.1)

### Instalacja Taskwarrior

```bash
brew install task
```

TaskFloat szuka binarki `task` w nastepujacych lokalizacjach (w tej kolejnosci):

1. `/opt/homebrew/bin/task` — Homebrew na Apple Silicon (M1/M2/M3)
2. `/usr/local/bin/task` — Homebrew na Intel Mac
3. `/usr/bin/task` — systemowa instalacja

Jesli `task` jest zainstalowany w innej sciezce, upewnij sie ze jest dostepny w jednej z powyzszych lokalizacji (np. przez symlink).

Dane Taskwarrior przechowywane sa w `~/.local/share/task/` (format TW3 — SQLite).

## Installation

### From DMG

Download the latest `.dmg` from [Releases](../../releases), open it and drag **TaskFloat.app** to Applications.

### Build from source

```bash
# Prerequisites
brew install task          # Taskwarrior 3.x
rustup update stable       # Rust 1.88+
npm install -g pnpm        # pnpm

# Clone and build
git clone https://github.com/YOUR_USER/taskfloat.git
cd taskfloat
pnpm install
pnpm tauri build
```

The built app will be at `src-tauri/target/release/bundle/macos/TaskFloat.app`.

## Development

```bash
pnpm install
pnpm tauri dev             # Dev mode with hot-reload
```

Frontend runs on `http://localhost:5173`, Tauri wraps it in a native window.

## Quick-add syntax

When adding a task, you can use shorthand:

| Syntax | Meaning |
|--------|---------|
| `p:project` | Set project |
| `+H` / `+M` / `+L` | Priority High / Medium / Low |
| `due:tomorrow` | Due date (any Taskwarrior date format) |
| `d:friday` | Short alias for `due:` |

Example: `Fix login bug p:webapp +H due:monday`

## Keyboard shortcuts

| Shortcut | Action |
|----------|--------|
| `Alt+Cmd+T` | Toggle window visibility (global) |
| `Cmd+F` | Open search bar |
| `Escape` | Close search / cancel edit |
| `Enter` | Save edit / add annotation |
| Double-click | Edit task inline |

## Settings

Click the gear icon in the status bar to open settings (separate window):

- **Opacity** — window transparency (30%-100%)
- **Accent color** — 7 presets (blue, green, purple, orange, red, gold, gray)
- **Compact mode** — reduced padding for more tasks on screen
- **Always on top** — keep window above all others
- **Show completed today** — toggle visibility of done tasks
- **Autostart** — launch at login
- **Poll interval** — how often to sync with Taskwarrior (5s / 15s / 30s / 1min)
- **View mode** — list or kanban
- **Pomodoro timer** — show/hide
- **Due notifications** — enable/disable

## Tech stack

| Layer | Technology |
|-------|-----------|
| Backend | Tauri 2.0 (Rust) |
| Frontend | SvelteKit + Svelte 5 |
| Task engine | Taskwarrior 3.x CLI |
| Build | pnpm, Cargo, Vite |

## Project structure

```
src/                      # SvelteKit frontend
  routes/                 # Pages (+page.svelte, settings/)
  lib/
    components/           # UI components
    stores/               # Reactive stores (tasks, projects, settings)
    types/                # TypeScript interfaces
src-tauri/                # Rust backend
  src/
    lib.rs                # App setup, plugins, tray, shortcuts
    taskwarrior.rs        # Taskwarrior CLI bridge
    commands/             # Tauri IPC commands
```

## How it works

1. Rust backend wraps Taskwarrior CLI calls (`task export`, `task add`, `task done`, etc.)
2. Frontend polls Rust commands at a configurable interval
3. Svelte stores manage reactive state with optimistic updates
4. Settings persisted to `localStorage`, synced across windows via `storage` events
5. Window size remembered between sessions

## License

MIT
