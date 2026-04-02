use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(default)]
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

#[derive(Debug, Serialize)]
pub struct TwInfo {
    pub version: String,
    pub task_count: usize,
}

use std::sync::RwLock;

static CUSTOM_TASK_PATH: RwLock<Option<String>> = RwLock::new(None);

pub fn set_custom_binary_path(path: &str) {
    let mut guard = CUSTOM_TASK_PATH.write().unwrap();
    if path.is_empty() {
        *guard = None;
    } else {
        *guard = Some(path.to_string());
    }
}

fn find_task_binary() -> String {
    // Check custom path first
    if let Ok(guard) = CUSTOM_TASK_PATH.read() {
        if let Some(ref custom) = *guard {
            return custom.clone();
        }
    }

    #[cfg(target_os = "windows")]
    let candidates = [
        r"C:\Program Files\Taskwarrior\bin\task.exe",
        r"C:\Program Files (x86)\Taskwarrior\bin\task.exe",
    ];
    #[cfg(not(target_os = "windows"))]
    let candidates = [
        "/opt/homebrew/bin/task",
        "/usr/local/bin/task",
        "/usr/bin/task",
    ];
    for path in &candidates {
        if std::path::Path::new(path).exists() {
            return path.to_string();
        }
    }
    // Fallback — try PATH
    "task".to_string()
}

fn exec_task(args: &[&str]) -> Result<String, String> {
    let task_bin = find_task_binary();
    let output = Command::new(&task_bin)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute task ({}): {}. Is Taskwarrior installed?", task_bin, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Taskwarrior prints warnings to stderr but still succeeds for some commands
        // Only treat as error if exit code is non-zero AND stdout is empty
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Err(format!("task command failed: {}", stderr));
        }
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in task output: {}", e))
}

pub fn get_tasks(filter: Option<&str>) -> Result<Vec<Task>, String> {
    let filter_str = filter.unwrap_or("status:pending");
    let output = exec_task(&["rc.json.array=on", filter_str, "export"])?;

    if output.trim().is_empty() || output.trim() == "[]" {
        return Ok(vec![]);
    }

    serde_json::from_str(&output)
        .map_err(|e| format!("Failed to parse tasks JSON: {}", e))
}

pub fn add_task(
    description: &str,
    project: Option<&str>,
    priority: Option<&str>,
    due: Option<&str>,
    tags: Option<&[String]>,
) -> Result<Task, String> {
    let mut args: Vec<String> = vec!["add".to_string(), description.to_string()];

    if let Some(p) = project {
        args.push(format!("project:{}", p));
    }
    if let Some(pri) = priority {
        args.push(format!("priority:{}", pri));
    }
    if let Some(d) = due {
        args.push(format!("due:{}", d));
    }
    if let Some(t) = tags {
        for tag in t {
            args.push(format!("+{}", tag));
        }
    }

    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    exec_task(&arg_refs)?;

    // Fetch the newly created task
    let tasks = get_tasks(Some("+LATEST"))?;
    tasks.into_iter().next().ok_or_else(|| "Failed to retrieve newly created task".to_string())
}

pub fn complete_task(uuid: &str) -> Result<(), String> {
    exec_task(&[&format!("uuid:{}", uuid), "done", "rc.confirmation=off"])?;
    Ok(())
}

pub fn uncomplete_task(uuid: &str) -> Result<(), String> {
    exec_task(&[
        &format!("uuid:{}", uuid),
        "modify",
        "status:pending",
        "rc.confirmation=off",
    ])?;
    Ok(())
}

pub fn modify_task(
    uuid: &str,
    modifications: &HashMap<String, String>,
) -> Result<Task, String> {
    let mut args: Vec<String> = vec![format!("uuid:{}", uuid), "modify".to_string()];

    for (key, value) in modifications {
        match key.as_str() {
            "tags_add" => {
                for tag in value.split(',') {
                    args.push(format!("+{}", tag.trim()));
                }
            }
            "tags_remove" => {
                for tag in value.split(',') {
                    args.push(format!("-{}", tag.trim()));
                }
            }
            _ => {
                args.push(format!("{}:{}", key, value));
            }
        }
    }

    args.push("rc.confirmation=off".to_string());

    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    exec_task(&arg_refs)?;

    // Fetch updated task
    let tasks = get_tasks(Some(&format!("uuid:{}", uuid)))?;
    tasks.into_iter().next().ok_or_else(|| "Task not found after modification".to_string())
}

pub fn annotate_task(uuid: &str, text: &str) -> Result<(), String> {
    exec_task(&[
        &format!("uuid:{}", uuid),
        "annotate",
        text,
        "rc.confirmation=off",
    ])?;
    Ok(())
}

pub fn delete_task(uuid: &str) -> Result<(), String> {
    exec_task(&[
        &format!("uuid:{}", uuid),
        "delete",
        "rc.confirmation=off",
    ])?;
    Ok(())
}

pub fn get_projects() -> Result<Vec<String>, String> {
    let output = exec_task(&["_projects"])?;
    Ok(output.lines().filter(|l| !l.is_empty()).map(String::from).collect())
}

pub fn get_tags() -> Result<Vec<String>, String> {
    let output = exec_task(&["_tags"])?;
    Ok(output.lines().filter(|l| !l.is_empty()).map(String::from).collect())
}

pub fn check_taskwarrior() -> Result<TwInfo, String> {
    let version = exec_task(&["--version"])?;
    let tasks = get_tasks(Some("status:pending"))?;

    Ok(TwInfo {
        version: version.trim().to_string(),
        task_count: tasks.len(),
    })
}
