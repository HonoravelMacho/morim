use crate::models::{Avatar, PodiumAsset, PodiumAssetType, Quiz};
use crate::server::AppState;
use crate::utils::get_app_data_dir;
use serde::Serialize;
use std::sync::Arc;
use tauri::{command, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ServerInfo {
    pub url: String,
    pub ip: String,
    pub port: u16,
}

#[command]
pub async fn get_local_ip(state: State<'_, Arc<AppState>>) -> Result<String, String> {
    Ok(state.local_ip.read().clone())
}

#[command]
pub async fn get_server_url(state: State<'_, Arc<AppState>>) -> Result<String, String> {
    Ok(state.server_url.read().clone())
}

#[command]
pub async fn list_quizzes(state: State<'_, Arc<AppState>>) -> Result<Vec<crate::models::QuizSummary>, String> {
    Ok(state.list_quizzes())
}

#[command]
pub async fn create_quiz(
    state: State<'_, Arc<AppState>>,
    title: String,
    description: String,
) -> Result<Quiz, String> {
    let quiz = Quiz::new(title, description);
    state.save_quiz(&quiz).map_err(|e| e.to_string())?;
    Ok(quiz)
}

#[command]
pub async fn get_quiz(state: State<'_, Arc<AppState>>, id: Uuid) -> Result<Quiz, String> {
    state
        .get_quiz(id)
        .ok_or_else(|| "Quiz not found".to_string())
}

#[command]
pub async fn list_avatars(state: State<'_, Arc<AppState>>) -> Result<Vec<Avatar>, String> {
    let mut avatars = Vec::new();
    let avatars_dir = state.assets_dir.join("avatars");
    if avatars_dir.exists() {
        let entries = std::fs::read_dir(&avatars_dir).map_err(|e| e.to_string())?;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext_str.as_str(),
                    "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp"
                ) {
                    let stem = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    avatars.push(Avatar {
                        id: stem.clone(),
                        name: stem,
                        file_path: format!(
                            "/assets/avatars/{}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ),
                        file_type: ext_str,
                    });
                }
            }
        }
    }
    Ok(avatars)
}

#[command]
pub async fn list_podiums(state: State<'_, Arc<AppState>>) -> Result<Vec<PodiumAsset>, String> {
    let mut podiums = Vec::new();
    let podiums_dir = state.assets_dir.join("podiums");
    if podiums_dir.exists() {
        let entries = std::fs::read_dir(&podiums_dir).map_err(|e| e.to_string())?;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let asset_type = if matches!(ext_str.as_str(), "mp3" | "wav" | "ogg" | "m4a") {
                    PodiumAssetType::Audio
                } else if matches!(ext_str.as_str(), "json" | "css" | "js") {
                    PodiumAssetType::Theme
                } else {
                    PodiumAssetType::Animation
                };
                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                podiums.push(PodiumAsset {
                    id: stem.clone(),
                    name: stem,
                    file_path: format!(
                        "/assets/podiums/{}",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    ),
                    file_type: ext_str,
                    asset_type,
                });
            }
        }
    }
    Ok(podiums)
}

#[command]
pub async fn delete_quiz(state: State<'_, Arc<AppState>>, quiz_id: Uuid) -> Result<bool, String> {
    state.delete_quiz(quiz_id).map_err(|e| e.to_string())?;
    Ok(true)
}

#[command]
pub async fn open_data_directory() -> Result<bool, String> {
    let dir = get_app_data_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(true)
}