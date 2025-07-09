use std::env;
use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
pub async fn get_binary_version(binary_name: Option<String>) -> Result<String, String> {
    let bin = binary_name.as_deref().unwrap_or("openlist");
    let mut binary_path: PathBuf =
        env::current_exe().map_err(|e| format!("Failed to get current exe path: {e}"))?;
    binary_path.pop();

    #[cfg(windows)]
    let file_name = format!("{bin}.exe");
    #[cfg(not(windows))]
    let file_name = bin.to_string();

    binary_path.push(file_name);

    let mut cmd = Command::new(&binary_path);
    cmd.arg("version");

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn {:?}: {}", &binary_path, e))?;

    if !output.status.success() {
        return Err(format!(
            "{:?} exited with status: {}",
            &binary_path, output.status
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let version = stdout
        .lines()
        .filter(|l| l.starts_with("Version:") || l.starts_with("rclone"))
        .filter_map(|l| l.split_whitespace().nth(1))
        .next()
        .ok_or_else(|| "Version not found in output".to_string())?;

    Ok(version.to_string())
}
