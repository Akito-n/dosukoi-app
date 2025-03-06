use serde_json::json;
use std::process::Command;
use tauri::command;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreBuilder;

#[command]
pub fn request_docker_path(app: AppHandle) -> Result<String, String> {
    let output = Command::new("which")
        .arg("docker")
        .output()
        .map_err(|e| format!("Failed to execute which docker: {}", e))?;

    if output.status.success() {
        let docker_path = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if docker_path.is_empty() {
            return Err("Docker not found".to_string());
        }

        let _ = app.dialog().message(format!(
            "Dockerの実行に必要な権限を許可しますか？\nDocker パス: {}",
            docker_path
        ));

        Ok(docker_path)
    } else {
        Err("Failed to find Docker".to_string())
    }
}

#[command]
pub fn save_docker_path(app: AppHandle, docker_path: String) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?; // `tauri::Error` を `String` に変換

    let store_path = app_data_dir.join("docker_path.json");

    let store = StoreBuilder::new(&app, store_path)
        .build()
        .map_err(|e| format!("Failed to create store: {}", e))?;

    store.set("docker_path", json!(docker_path));

    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}
