# Integracja z Taskwarrior

## Przegląd

TaskFloat komunikuje się z Taskwarrior wyłącznie przez CLI `task`. Żadnego bezpośredniego dostępu do bazy danych. Taskwarrior jest single source of truth — TaskFloat to tylko widok + kontroler.

## Środowisko użytkownika

```
Taskwarrior: 3.4.1
Storage:     ~/.local/share/task/taskchampion.sqlite3
Projekty:    elzbietanki, hosting, panda, rodo, seo, suchanek.sklep, 
             wlasnadomena.pl, wywozik, as
Aktywne:     ~21 tasków pending
```

## Komendy CLI

### Odczyt

#### Pobranie tasków (JSON)
```bash
# Wszystkie pending
task rc.json.array=on status:pending export

# Filtrowane po projekcie
task rc.json.array=on status:pending project:wywozik export

# Pending + completed dzisiaj (do wyświetlenia sekcji "done today")
task rc.json.array=on "(status:pending or (status:completed and end:today))" export

# Pojedynczy task po UUID
task rc.json.array=on uuid:31e54937-043b-418d-9a64-c2708ddc23f6 export
```

**Flaga `rc.json.array=on`**: Wymusza output jako JSON array `[...]` zamiast pojedynczych obiektów — ułatwia parsowanie w Rust.

#### Lista projektów
```bash
task _projects
# Output: jeden projekt na linię
```

#### Lista tagów
```bash
task _tags
# Output: jeden tag na linię
```

### Mutacje

#### Dodanie taska
```bash
# Prosty
task add "Opis taska"

# Z projektem i priorytetem
task add "Opis taska" project:wywozik priority:H

# Z datą due
task add "Opis taska" project:seo due:2026-04-15

# Z tagami
task add "Opis taska" +urgent +work
```

**Priority mapping**:
| TW wartość | Znaczenie | UI kolor |
|-----------|-----------|----------|
| `H` | High | Czerwony |
| `M` | Medium | Żółty |
| `L` | Low | Zielony |
| (brak) | None | Szary |

#### Oznaczenie jako done
```bash
task <uuid> done
# Lub po ID (ale UUID jest stabilniejszy)
task uuid:31e54937-043b-418d-9a64-c2708ddc23f6 done
```

#### Modyfikacja
```bash
task <uuid> modify description:"Nowy opis"
task <uuid> modify project:inny_projekt
task <uuid> modify priority:M
task <uuid> modify due:2026-04-20
task <uuid> modify +tag_name
task <uuid> modify -tag_name
```

#### Usunięcie
```bash
task <uuid> delete rc.confirmation=off
# rc.confirmation=off — nie pyta o potwierdzenie (ważne dla non-interactive)
```

## Struktura JSON taska

Taskwarrior export zwraca obiekty o następującej strukturze (przykład z realnych danych):

```json
{
  "id": 1,
  "description": "Polaczyc z HRD i Uruchoamic",
  "entry": "20251009T165759Z",
  "modified": "20251009T165759Z",
  "project": "wlasnadomena.pl",
  "status": "pending",
  "uuid": "31e54937-043b-418d-9a64-c2708ddc23f6",
  "urgency": 1.95342
}
```

### Pola używane w TaskFloat

| Pole | Typ | Wymagane | Opis |
|------|-----|----------|------|
| `id` | `u32` | tak | Numer sekwencyjny (zmienia się!) |
| `uuid` | `String` | tak | Stabilny identyfikator (klucz główny) |
| `description` | `String` | tak | Treść taska |
| `status` | `String` | tak | `pending`, `completed`, `deleted`, `waiting` |
| `project` | `String?` | nie | Nazwa projektu |
| `priority` | `String?` | nie | `H`, `M`, `L` lub null |
| `due` | `String?` | nie | Data w formacie ISO `20260415T230000Z` |
| `tags` | `Vec<String>?` | nie | Lista tagów |
| `entry` | `String` | tak | Data utworzenia |
| `modified` | `String` | tak | Data ostatniej modyfikacji |
| `end` | `String?` | nie | Data zakończenia (completed/deleted) |
| `urgency` | `f64` | tak | Obliczona przez TW wartość urgency |
| `annotations` | `Vec<Annotation>?` | nie | Notatki/komentarze |
| `depends` | `String?` | nie | UUID tasków blokujących (CSV) |
| `recur` | `String?` | nie | Cykl powtarzania (`daily`, `weekly`, etc.) |

### Pola ignorowane (nie wyświetlane, ale zachowywane)
- `imask`, `parent`, `rtype`, `mask` — internale recurring tasks
- `reminderID` — legacy z poprzedniej integracji z Apple Reminders
- UDA (User Defined Attributes) — przekazywane transparentnie

## Rust: Moduł taskwarrior.rs

### Struktura modułu

```rust
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub uuid: String,
    pub description: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub entry: String,
    pub modified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    pub urgency: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recur: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    pub entry: String,
    pub description: String,
}
```

### Executor

```rust
/// Wykonuje komendę `task` i zwraca stdout
fn exec_task(args: &[&str]) -> Result<String, String> {
    let output = Command::new("task")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute task: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("task command failed: {}", stderr));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in output: {}", e))
}

/// Pobiera taski z filtrem
pub fn get_tasks(filter: &str) -> Result<Vec<Task>, String> {
    let output = exec_task(&["rc.json.array=on", filter, "export"])?;
    serde_json::from_str(&output)
        .map_err(|e| format!("Failed to parse tasks JSON: {}", e))
}

/// Pobiera listę projektów
pub fn get_projects() -> Result<Vec<String>, String> {
    let output = exec_task(&["_projects"])?;
    Ok(output.lines().map(String::from).collect())
}
```

## Strategia synchronizacji

### Polling (MVP)

```
Frontend                    Rust                      Taskwarrior
   │                         │                            │
   │──invoke("get_tasks")──►│                            │
   │                         │──task export──────────────►│
   │                         │◄──JSON──────────────────── │
   │◄──Vec<Task>────────────│                            │
   │                         │                            │
   │  (15 sekund później)    │                            │
   │──invoke("get_tasks")──►│                            │
   │        ...              │                            │
```

**Interwał**: 15 sekund
- Wystarczająco responsywny dla task managera
- Minimalny koszt CPU (~1ms per call)
- `task export` na 21 taskach: <50ms

### Optimistic updates

Przy mutacjach (done, add, modify) natychmiast aktualizujemy Svelte store **przed** potwierdzeniem z CLI:

```
User klika checkbox
    → Store: task.status = "completed" (natychmiastowy feedback)
    → invoke("complete_task") w tle
    → Sukces: nic nie robimy (store już aktualny)
    → Błąd: rollback store + toast z błędem
```

### Przyszłe ulepszenia (post-MVP)

1. **Smart polling**: Porównanie `modified` timestamp — jeśli nic się nie zmieniło, skip re-render
2. **Taskwarrior hooks**: Hook `on-modify` / `on-add` zapisuje do pliku flag → Rust `fs::watch` → natychmiastowy refresh
3. **TaskChampion sync**: Jeśli TW3 dostanie sync API, bezpośrednia integracja

## Obsługa błędów

### Task CLI niedostępne
```rust
// Przy starcie aplikacji — sprawdzenie czy `task` jest w PATH
fn check_taskwarrior() -> Result<String, String> {
    exec_task(&["--version"])
}
```
Frontend pokazuje error state z instrukcją instalacji jeśli `task` nie znaleziony.

### Concurrent access
Taskwarrior 3 (TaskChampion) obsługuje locking na poziomie SQLite. Jeśli inny process (np. terminal) modyfikuje taski w tym samym czasie:
- `task export` zawsze zwraca spójny stan
- Mutacje mogą chwilowo failować — retry po 1s

### Non-interactive mode
Wszystkie komendy mutujące używają flag wyłączających interaktywność:
```bash
task <uuid> delete rc.confirmation=off
task <uuid> done rc.confirmation=off
```

## Bezpieczeństwo

- TaskFloat **nie** przechowuje żadnych danych — wszystko w Taskwarrior
- Żadne dane nie opuszczają maszyny (zero networku)
- CLI execution ograniczone do `task` binary (whitelist w capabilities)
- Brak eval/exec na user input — parametry przekazywane jako args, nie shell string
