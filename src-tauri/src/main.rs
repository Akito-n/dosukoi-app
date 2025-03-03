use serde::Serialize;
use std::{collections::HashMap, thread, time::Duration};
use tauri::{Emitter, Manager, Runtime, WebviewWindow};

#[derive(Serialize, Clone, PartialEq)]
struct ContainerGroup {
    name: String,
    containers: Vec<String>,
}

#[tauri::command]
fn get_grouped_containers() -> Vec<ContainerGroup> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}|{{.Label \"com.docker.compose.project\"}}'")
        .output()
        .expect("Failed to execute docker ps");

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let container_name = parts[0].to_string();
                let project_name = parts[1].to_string();

                // `project_name` が空の場合は "Other" する
                let group = if project_name.is_empty() {
                    "Other".to_string()
                } else {
                    project_name
                };

                groups
                    .entry(group)
                    .or_insert_with(Vec::new)
                    .push(container_name);
            }
        }
    }

    let mut grouped_containers: Vec<ContainerGroup> = groups
        .into_iter()
        .map(|(name, mut containers)| {
            containers.sort();
            ContainerGroup { name, containers }
        })
        .collect();

    grouped_containers.sort_by(|a, b| a.name.cmp(&b.name));

    grouped_containers
}

fn emit_containers_update<R: Runtime>(window: &WebviewWindow<R>, groups: Vec<ContainerGroup>) {
    if let Err(e) = window.emit("containers_updated", groups) {
        eprintln!("Failed to emit containers_updated event: {}", e);
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            // 🔹 Docker の監視を別スレッドで実行
            thread::spawn(move || {
                let mut last_groups = Vec::new();

                loop {
                    let new_groups = get_grouped_containers();

                    if new_groups != last_groups {
                        if let Some(window) = handle.get_webview_window("main") {
                            emit_containers_update(&window, new_groups.clone());
                        }
                        last_groups = new_groups;
                    }

                    thread::sleep(Duration::from_secs(3)); // 一旦3秒待つ
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_grouped_containers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
