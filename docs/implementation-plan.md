# Plan implementacji — TaskFloat

## Fazy

### Faza 1: Scaffold + okno (MVP-0)
**Cel**: Puste floating window Tauri z SvelteKit, buduje się i uruchamia.

1. `pnpm create tauri-app` z template SvelteKit
2. Konfiguracja `tauri.conf.json`: frameless, transparent, always-on-top, skipTaskbar
3. SvelteKit static adapter
4. Custom titlebar z drag region
5. Glass morphism CSS na root element
6. Tray icon z show/hide toggle

**Kryterium sukcesu**: `pnpm tauri dev` uruchamia półprzezroczyste okno bez titlebar, draggable, always-on-top.

---

### Faza 2: Taskwarrior bridge (MVP-1)
**Cel**: Rust czyta i modyfikuje taski przez CLI.

1. `taskwarrior.rs` — executor `std::process::Command`
2. Struct `Task` z serde (de)serialization
3. Tauri commands: `get_tasks`, `get_projects`, `check_taskwarrior`
4. Tauri commands: `add_task`, `complete_task`, `modify_task`, `delete_task`
5. Error handling i non-interactive flags

**Kryterium sukcesu**: `invoke("get_tasks")` z frontend zwraca realne taski z TW jako JSON.

---

### Faza 3: Lista tasków (MVP-2)
**Cel**: Wyświetlanie i interakcja z taskami.

1. Svelte store `tasks` z polling co 15s
2. Komponent `TaskList.svelte` + `TaskItem.svelte`
3. Checkbox → complete z optimistic update
4. Priority indicators (kolorowe kropki)
5. Due date display (relative: "za 2 dni", "dziś", "przeterminowane")
6. Sortowanie wg urgency
7. Sekcja "Ukończone dziś"

**Kryterium sukcesu**: Widać realne taski, można je odhaczać, lista się odświeża.

---

### Faza 4: Dodawanie i filtrowanie (MVP-3)
**Cel**: Pełna interakcja z taskami.

1. `AddTask.svelte` — input z Enter = dodaj task
2. `ProjectFilter.svelte` — pills z projektami, filtrowanie
3. Expanded form (Shift+Enter): projekt, priorytet, due date
4. `StatusBar.svelte` — countery

**Kryterium sukcesu**: Można dodawać taski, filtrować po projekcie, widzieć statystyki.

---

### Faza 5: Plugins i system integration (MVP-4)
**Cel**: Autostart, hotkey, position memory.

1. `tauri-plugin-autostart` — launch at login
2. `tauri-plugin-global-shortcut` — `⌥⌘T` toggle
3. Zapisywanie pozycji okna (localStorage lub Tauri store)
4. Settings panel (autostart on/off, hotkey config, polling interval)
5. Tray menu: Show/Hide, Settings, Quit

**Kryterium sukcesu**: App startuje z Macem, `⌥⌘T` toggle działa, pozycja się zapamiętuje.

---

### Faza 6: Polish (v1.0)
**Cel**: Dopracowanie wizualne i UX.

1. Animacje complete (checkbox spring, strikethrough fade, slide)
2. Keyboard navigation (↑/↓, Space, ⌘N)
3. Context menu na task (edit, delete, change priority)
4. Empty states (no tasks, all done)
5. Error states (TW not found, command failed)
6. Resize adaptacja (ukrywanie pills na narrow)
7. Performance: memoizacja, virtualized list jeśli >100 tasków

**Kryterium sukcesu**: Produkcyjna jakość UI, płynne animacje, brak bugów UX.

---

## Zależności npm (frontend)

```json
{
  "devDependencies": {
    "@sveltejs/adapter-static": "^3",
    "@sveltejs/kit": "^2",
    "@tauri-apps/cli": "^2",
    "svelte": "^5",
    "vite": "^6",
    "tailwindcss": "^4"
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-autostart": "^2",
    "@tauri-apps/plugin-global-shortcut": "^2",
    "@tauri-apps/plugin-positioner": "^2"
  }
}
```

## Zależności Cargo (Rust)

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-autostart = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-positioner = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Szacunkowy czas

| Faza | Czas | Kumulatywny |
|------|------|-------------|
| Faza 1: Scaffold | 2-3h | 2-3h |
| Faza 2: TW bridge | 2-3h | 4-6h |
| Faza 3: Task list | 3-4h | 7-10h |
| Faza 4: Add + filter | 2-3h | 9-13h |
| Faza 5: Plugins | 2-3h | 11-16h |
| Faza 6: Polish | 4-6h | 15-22h |

**MVP używalny (Fazy 1-3)**: ~10h
**Pełna v1.0**: ~20h
