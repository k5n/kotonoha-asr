use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
pub fn save_transcription_file(filepath: String, content: String) -> Result<(), String> {
    let path = Path::new(&filepath);

    // Expect absolute path from Tauri dialog save
    if !path.is_absolute() {
        return Err("filepath must be absolute".into());
    }

    // Simple safety check: disallow parent dir components
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("invalid path: parent directory components (`..`) are not allowed".into());
    }

    let parent = path
        .parent()
        .ok_or_else(|| "invalid filepath: missing parent".to_string())?;

    // Per your note, parent directory should already exist; verify to be safe.
    if !parent.exists() {
        return Err(format!(
            "parent directory does not exist: {}",
            parent.display()
        ));
    }

    let file_name = path
        .file_name()
        .ok_or_else(|| "invalid filepath: missing filename".to_string())?;

    // Create a unique temp file name in the same directory to allow atomic rename.
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("time error: {}", e))?
        .as_nanos();

    let mut attempt: u32 = 0;
    let tmp_path = loop {
        let tmp_name = format!(".{}.tmp.{}.{}", file_name.to_string_lossy(), ts, attempt);
        let candidate = parent.join(&tmp_name);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(mut f) => {
                f.write_all(content.as_bytes())
                    .map_err(|e| format!("failed to write temp file: {}", e))?;
                f.sync_all()
                    .map_err(|e| format!("failed to sync temp file to disk: {}", e))?;
                break candidate;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    attempt = attempt.saturating_add(1);
                    if attempt > 16 {
                        return Err(
                            "failed to create unique temp file after several attempts".into()
                        );
                    }
                    continue;
                } else {
                    return Err(format!("failed to create temp file: {}", e));
                }
            }
        }
    };

    // Atomic move into place
    fs::rename(&tmp_path, &path).map_err(|e| format!("failed to rename temp file: {}", e))?;

    Ok(())
}
