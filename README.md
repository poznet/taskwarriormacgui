# TaskFloat

A floating task manager for macOS, Windows and Linux that sits on top of your windows. Powered by [Taskwarrior](https://taskwarrior.org/) as the backend engine.

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

- **Taskwarrior 3.x** (tested with 3.4.1)
- **macOS** 13.0+ / **Windows** 10+ / **Linux** (Ubuntu 22.04+, Fedora, Arch)

## Installation

### macOS

Download the latest `.dmg` from [Releases](../../releases), open it and drag **TaskFloat.app** to Applications.

**Install Taskwarrior:**
```bash
brew install task
```

### Windows

Download the latest `.msi` or `_x64-setup.exe` from [Releases](../../releases) and run the installer.

**Install Taskwarrior:**

Taskwarrior on Windows can be installed via:
- [Scoop](https://scoop.sh/): `scoop install task`
- [Chocolatey](https://chocolatey.org/): `choco install taskwarrior`
- [MSYS2](https://www.msys2.org/): `pacman -S task`
- Manual build from [source](https://github.com/GothenburgBitFactory/taskwarrior)

### Linux

Download the latest package from [Releases](../../releases):
- **Ubuntu/Debian**: `.deb` — `sudo dpkg -i TaskFloat_*.deb`
- **Fedora/RHEL**: `.rpm` — `sudo rpm -i TaskFloat_*.rpm`
- **Any distro**: `.AppImage` — `chmod +x TaskFloat_*.AppImage && ./TaskFloat_*.AppImage`

**Required system dependencies (Ubuntu/Debian):**
```bash
sudo apt install libwebkit2gtk-4.1-0
```

If you get dependency errors when installing the `.deb`, run:
```bash
sudo apt install -f
```

**Install Taskwarrior:**
```bash
# Ubuntu/Debian
sudo apt install taskwarrior

# Fedora
sudo dnf install task

# Arch
sudo pacman -S task
```

### Taskwarrior path detection

TaskFloat automatically searches for the `task` binary in common locations:

| Platform | Searched paths |
|----------|---------------|
| macOS | `/opt/homebrew/bin/task`, `/usr/local/bin/task`, `/usr/bin/task` |
| Linux | `/usr/local/bin/task`, `/usr/bin/task` |
| Windows | `C:\Program Files\Taskwarrior\bin\task.exe`, `C:\Program Files (x86)\Taskwarrior\bin\task.exe` |
| All | Falls back to `task` in PATH |

If `task` is installed in a non-standard location, you can set the path manually in **Settings > Sciezka do Taskwarrior**.

### Build from source

```bash
# Prerequisites: Taskwarrior 3.x, Rust 1.88+, pnpm

# Clone and build
git clone https://github.com/poznet/taskwarriormacgui.git
cd taskwarriormacgui
pnpm install
pnpm tauri build
```

Build output locations:
- **macOS**: `src-tauri/target/release/bundle/macos/TaskFloat.app`
- **Windows**: `src-tauri/target/release/bundle/msi/TaskFloat_*.msi`
- **Linux**: `src-tauri/target/release/bundle/deb/TaskFloat_*.deb`

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
- **Taskwarrior path** — custom path to `task` binary (empty = auto-detect)

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
