# API — Tauri Commands (IPC Contract)

## Przegląd

Frontend komunikuje się z Rustem przez `invoke()` z `@tauri-apps/api/core`. Każdy command jest async i zwraca `Result<T, String>`. Frontend obsługuje błędy przez `try/catch`.

```typescript
import { invoke } from '@tauri-apps/api/core';

const tasks = await invoke<Task[]>('get_tasks', { filter: 'status:pending' });
```

## Typy danych

### Task (shared: Rust serde + TypeScript interface)

```typescript
// src/lib/types/task.ts

interface Task {
  id: number;              // TW sequential ID (niestabilny!)
  uuid: string;            // Stabilny identyfikator — używaj tego
  description: string;     // Treść taska
  status: 'pending' | 'completed' | 'deleted' | 'waiting' | 'recurring';
  project?: string;        // Nazwa projektu
  priority?: 'H' | 'M' | 'L';  // High, Medium, Low
  due?: string;            // ISO datetime "20260415T230000Z"
  tags?: string[];         // Lista tagów
  entry: string;           // Data utworzenia
  modified: string;        // Data modyfikacji
  end?: string;            // Data zakończenia
  urgency: number;         // Obliczona urgency (float)
  annotations?: Annotation[];
  depends?: string;        // UUID zależności (CSV)
  recur?: string;          // Cykl powtarzania
}

interface Annotation {
  entry: string;           // Data dodania
  description: string;     // Treść
}

interface TaskCreateParams {
  description: string;
  project?: string;
  priority?: 'H' | 'M' | 'L';
  due?: string;            // Format: "2026-04-15" lub "tomorrow" (TW parsuje)
  tags?: string[];
}

interface TaskModifyParams {
  uuid: string;
  description?: string;
  project?: string;
  priority?: 'H' | 'M' | 'L' | '';  // '' = usunięcie priorytetu
  due?: string;
  tags_add?: string[];     // Tagi do dodania
  tags_remove?: string[];  // Tagi do usunięcia
}
```

## Commands

### get_tasks

Pobiera listę tasków z opcjonalnym filtrem.

```
Command:   get_tasks
Params:    { filter?: string }
Returns:   Task[]
Errors:    "task command failed: ...", "Failed to parse tasks JSON: ..."
```

**Rust signature**:
```rust
#[tauri::command]
async fn get_tasks(filter: Option<String>) -> Result<Vec<Task>, String>
```

**Frontend usage**:
```typescript
// Wszystkie pending
const tasks = await invoke<Task[]>('get_tasks');

// Filtrowane po projekcie
const tasks = await invoke<Task[]>('get_tasks', { 
  filter: 'project:wywozik' 
});

// Pending + completed dziś
const tasks = await invoke<Task[]>('get_tasks', { 
  filter: '(status:pending or (status:completed and end:today))' 
});
```

**Implementacja CLI**:
```bash
task rc.json.array=on ${filter:-status:pending} export
```

---

### add_task

Tworzy nowy task w Taskwarrior.

```
Command:   add_task
Params:    TaskCreateParams
Returns:   Task  (nowo utworzony, pobrany z export)
Errors:    "task command failed: ..."
```

**Rust signature**:
```rust
#[tauri::command]
async fn add_task(
    description: String,
    project: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Task, String>
```

**Frontend usage**:
```typescript
const newTask = await invoke<Task>('add_task', {
  description: 'Nowy task',
  project: 'wywozik',
  priority: 'M',
  due: '2026-04-15',
  tags: ['urgent']
});
```

**Implementacja CLI**:
```bash
task add "Nowy task" project:wywozik priority:M due:2026-04-15 +urgent
# Następnie: task +LATEST export → zwraca nowy task
```

---

### complete_task

Oznacza task jako ukończony.

```
Command:   complete_task
Params:    { uuid: string }
Returns:   void
Errors:    "task command failed: ...", "Task not found: ..."
```

**Rust signature**:
```rust
#[tauri::command]
async fn complete_task(uuid: String) -> Result<(), String>
```

**Frontend usage**:
```typescript
await invoke('complete_task', { uuid: '31e54937-043b-418d-9a64-c2708ddc23f6' });
```

**Implementacja CLI**:
```bash
task uuid:${uuid} done rc.confirmation=off
```

---

### uncomplete_task

Przywraca completed task do pending (undo).

```
Command:   uncomplete_task
Params:    { uuid: string }
Returns:   void
Errors:    "task command failed: ..."
```

**Rust signature**:
```rust
#[tauri::command]
async fn uncomplete_task(uuid: String) -> Result<(), String>
```

**Implementacja CLI**:
```bash
task uuid:${uuid} modify status:pending rc.confirmation=off
```

---

### modify_task

Modyfikuje istniejący task.

```
Command:   modify_task
Params:    TaskModifyParams
Returns:   Task  (zaktualizowany)
Errors:    "task command failed: ..."
```

**Rust signature**:
```rust
#[tauri::command]
async fn modify_task(
    uuid: String,
    description: Option<String>,
    project: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    tags_add: Option<Vec<String>>,
    tags_remove: Option<Vec<String>>,
) -> Result<Task, String>
```

**Frontend usage**:
```typescript
const updated = await invoke<Task>('modify_task', {
  uuid: '31e54937-...',
  priority: 'H',
  due: 'tomorrow',
  tags_add: ['urgent']
});
```

**Implementacja CLI**:
```bash
task uuid:${uuid} modify description:"..." project:X priority:H due:tomorrow +urgent -oldtag
# Następnie: task uuid:${uuid} export
```

---

### delete_task

Usuwa task (soft delete — TW zmienia status na 'deleted').

```
Command:   delete_task
Params:    { uuid: string }
Returns:   void
Errors:    "task command failed: ..."
```

**Implementacja CLI**:
```bash
task uuid:${uuid} delete rc.confirmation=off
```

---

### get_projects

Zwraca listę aktywnych projektów.

```
Command:   get_projects
Params:    (brak)
Returns:   string[]
Errors:    "task command failed: ..."
```

**Implementacja CLI**:
```bash
task _projects
```

---

### get_tags

Zwraca listę tagów.

```
Command:   get_tags
Params:    (brak)
Returns:   string[]
Errors:    "task command failed: ..."
```

**Implementacja CLI**:
```bash
task _tags
```

---

### check_taskwarrior

Sprawdza czy Taskwarrior jest zainstalowany i dostępny.

```
Command:   check_taskwarrior
Params:    (brak)
Returns:   { version: string, data_location: string }
Errors:    "Taskwarrior not found in PATH"
```

**Implementacja CLI**:
```bash
task --version
task _show | grep data.location
```

---

## Error handling

### Frontend pattern

```typescript
// src/lib/stores/tasks.ts

async function fetchTasks(filter?: string): Promise<void> {
  try {
    const result = await invoke<Task[]>('get_tasks', { filter });
    tasks.set(result);
    error.set(null);
  } catch (e) {
    error.set(String(e));
    // Nie czyścimy tasks — pokazujemy stale dane + error toast
  }
}
```

### Rust error mapping

Wszystkie błędy jako `String` (Tauri wymaga `Serialize` na error type, `String` jest najprostszy):

```rust
// Wzorzec: mapowanie std::io::Error i serde błędów na String
.map_err(|e| format!("Failed to ...: {}", e))
```

### Error types (frontend)

| Typ | Przyczyna | UI reakcja |
|-----|-----------|-----------|
| `Taskwarrior not found` | `task` nie w PATH | Error screen + instrukcja instalacji |
| `task command failed` | TW zwrócił błąd | Toast z opisem błędu |
| `Failed to parse JSON` | Nieoczekiwany output | Toast + retry przy następnym poll |
| `Network error` | IPC failure | Nie powinno wystąpić (local) |

## Polling contract

```typescript
// src/lib/stores/tasks.ts

const POLL_INTERVAL = 15_000; // 15 sekund

let pollTimer: ReturnType<typeof setInterval>;

export function startPolling(filter?: string) {
  fetchTasks(filter); // natychmiast
  pollTimer = setInterval(() => fetchTasks(filter), POLL_INTERVAL);
}

export function stopPolling() {
  clearInterval(pollTimer);
}

// Zmiana filtra resetuje polling
export function setFilter(newFilter: string) {
  stopPolling();
  startPolling(newFilter);
}
```

## Optimistic updates contract

```typescript
// Przy complete_task:
function optimisticComplete(uuid: string) {
  // 1. Natychmiastowy update store
  tasks.update(list => 
    list.map(t => t.uuid === uuid 
      ? { ...t, status: 'completed', end: new Date().toISOString() } 
      : t
    )
  );
  
  // 2. Wywołanie backend
  invoke('complete_task', { uuid }).catch(() => {
    // 3. Rollback przy błędzie
    fetchTasks(); // re-fetch pełny stan
    showToast('Nie udało się oznaczyć taska');
  });
}
```
