use std::process::Command;
use tauri::command;

#[command]
pub fn stop_container(container_name: String) -> String {
    println!("Received container_name: {:?}", container_name);
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker stop {}", container_name))
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                format!("Container '{}' stopped successfully.", container_name)
            } else {
                format!(
                    "Failed to stop container '{}': {}",
                    container_name,
                    String::from_utf8_lossy(&output.stderr)
                )
            }
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}

/// コンテナを強制終了する（docker kill）
#[command]
pub fn kill_container(container_name: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker kill {}", container_name))
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                format!("Container '{}' killed successfully.", container_name)
            } else {
                format!(
                    "Failed to kill container '{}': {}",
                    container_name,
                    String::from_utf8_lossy(&output.stderr)
                )
            }
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}

#[command]
pub fn kill_group_containers(group_name: String) -> String {
    // 指定したグループ（プロジェクト）に属するコンテナの名前を取得
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "docker ps --format '{{{{.Names}}}}' --filter 'label=com.docker.compose.project={}'",
            group_name
        ))
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let containers: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

                if containers.is_empty() {
                    return format!("No running containers found in group '{}'.", group_name);
                }

                // 取得したコンテナリストを stop_or_kill_containers に渡して kill する
                stop_or_kill_containers(&containers, true);
                format!(
                    "All containers in group '{}' killed successfully.",
                    group_name
                )
            } else {
                format!(
                    "Failed to get containers in group '{}': {}",
                    group_name,
                    String::from_utf8_lossy(&output.stderr)
                )
            }
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}

pub fn stop_or_kill_containers(containers: &[String], kill: bool) {
    let command = if kill { "kill" } else { "stop" };
    let container_list = containers.join(" ");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker {} {}", command, container_list))
        .output()
        .expect("Failed to execute docker command");

    if output.status.success() {
        println!("(╯°□°）╯︵ ┻━┻\nSelected containers dosukoi!");
    } else {
        eprintln!(
            "Error stopping containers: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
