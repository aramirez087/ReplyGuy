use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use tauri::Manager;

/// Managed state holding the tuitbot-server child process.
struct ServerProcess(Mutex<Option<Child>>);

/// Returns the tuitbot config directory (~/.tuitbot/).
fn tuitbot_config_dir() -> std::path::PathBuf {
    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".tuitbot")
}

/// Read the API token from ~/.tuitbot/api_token.
fn read_api_token() -> Result<String, String> {
    let token_path = tuitbot_config_dir().join("api_token");
    std::fs::read_to_string(&token_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Failed to read API token at {}: {}", token_path.display(), e))
}

/// Tauri command: returns the API token to the frontend.
#[tauri::command]
fn get_api_token() -> Result<String, String> {
    read_api_token()
}

/// Check if tuitbot-server is already listening on the given port.
fn is_port_in_use(port: u16) -> bool {
    std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
}

/// Spawn the tuitbot-server process.
fn spawn_server() -> Option<Child> {
    let port: u16 = 3001;

    // If the server is already running (e.g. started manually), don't spawn another.
    if is_port_in_use(port) {
        log::info!("tuitbot-server already running on port {}", port);
        return None;
    }

    log::info!("Spawning tuitbot-server on port {}...", port);

    // Try to find the server binary in common locations.
    let binary = find_server_binary();

    match Command::new(&binary)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => {
            log::info!("tuitbot-server started (pid: {})", child.id());

            // Give the server a moment to start up.
            std::thread::sleep(std::time::Duration::from_millis(500));

            Some(child)
        }
        Err(e) => {
            log::error!("Failed to spawn tuitbot-server at '{}': {}", binary, e);
            None
        }
    }
}

/// Locate the tuitbot-server binary.
///
/// Search order:
/// 1. Next to the Tauri binary (bundled release)
/// 2. In PATH (development / cargo install)
fn find_server_binary() -> String {
    // Check next to the current executable (for bundled releases).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let bundled = dir.join("tuitbot-server");
            if bundled.exists() {
                return bundled.to_string_lossy().into_owned();
            }
        }
    }

    // Fall back to PATH.
    "tuitbot-server".to_string()
}

/// Gracefully shut down the server process.
fn shutdown_server(server: &Mutex<Option<Child>>) {
    if let Ok(mut guard) = server.lock() {
        if let Some(mut child) = guard.take() {
            log::info!("Shutting down tuitbot-server (pid: {})...", child.id());

            // Send a kill signal. On Unix, ideally we'd send SIGTERM first,
            // but Child::kill() is cross-platform.
            let _ = child.kill();

            // Collect any remaining stderr for diagnostics.
            if let Some(mut stderr) = child.stderr.take() {
                let mut output = String::new();
                let _ = stderr.read_to_string(&mut output);
                if !output.is_empty() {
                    log::debug!("tuitbot-server stderr: {}", output);
                }
            }

            let _ = child.wait();
            log::info!("tuitbot-server stopped.");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Spawn the server as a child process.
            let child = spawn_server();
            app.manage(ServerProcess(Mutex::new(child)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_api_token])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            let state: tauri::State<ServerProcess> = app_handle.state();
            shutdown_server(&state.0);
        }
    });
}
