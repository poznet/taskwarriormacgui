use crate::taskwarrior;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_tasks(filter: Option<String>) -> Result<Vec<taskwarrior::Task>, String> {
    taskwarrior::get_tasks(filter.as_deref())
}

#[tauri::command]
pub async fn add_task(
    description: String,
    project: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<taskwarrior::Task, String> {
    taskwarrior::add_task(
        &description,
        project.as_deref(),
        priority.as_deref(),
        due.as_deref(),
        tags.as_deref(),
    )
}

#[tauri::command]
pub async fn complete_task(uuid: String) -> Result<(), String> {
    taskwarrior::complete_task(&uuid)
}

#[tauri::command]
pub async fn uncomplete_task(uuid: String) -> Result<(), String> {
    taskwarrior::uncomplete_task(&uuid)
}

#[tauri::command]
pub async fn modify_task(
    uuid: String,
    modifications: HashMap<String, String>,
) -> Result<taskwarrior::Task, String> {
    taskwarrior::modify_task(&uuid, &modifications)
}

#[tauri::command]
pub async fn annotate_task(uuid: String, text: String) -> Result<(), String> {
    taskwarrior::annotate_task(&uuid, &text)
}

#[tauri::command]
pub async fn delete_task(uuid: String) -> Result<(), String> {
    taskwarrior::delete_task(&uuid)
}

#[tauri::command]
pub async fn get_projects() -> Result<Vec<String>, String> {
    taskwarrior::get_projects()
}

#[tauri::command]
pub async fn get_tags() -> Result<Vec<String>, String> {
    taskwarrior::get_tags()
}

#[tauri::command]
pub async fn check_taskwarrior() -> Result<taskwarrior::TwInfo, String> {
    taskwarrior::check_taskwarrior()
}
