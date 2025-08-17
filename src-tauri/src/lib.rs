use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StartedPayload {
    total_duration_ms: u64,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressPayload {
    text: String,
    start_time_ms: u64,
    end_time_ms: u64,
}

#[tauri::command]
fn start_asr_process(app_handle: AppHandle, file_path: String) {
    log::info!("Starting ASR process for: {}", file_path);

    thread::spawn(move || {
        let total_duration_ms = 120000; // 2 minutes dummy data
        app_handle
            .emit("asr-started", StartedPayload { total_duration_ms })
            .unwrap();
        thread::sleep(Duration::from_secs(1));

        let sentences = vec![
            ("本日はお集まりいただきありがとうございます。", 0, 5000),
            (
                "今回は新プロジェクトの進捗についてお話します。",
                6000,
                15000,
            ),
            (
                "まず、現在の開発状況ですが、主要機能の実装が完了し、",
                16000,
                28000,
            ),
            ("現在テストフェーズに移行しております。", 29000, 35000),
            ("いくつかの軽微なバグが報告されていますが、", 36000, 45000),
            ("リリーススケジュールに影響はない見込みです。", 46000, 55000),
            ("次に、マーケティング戦略についてですが、", 58000, 68000),
            (
                "来週からSNSでのティザーキャンペーンを開始します。",
                69000,
                80000,
            ),
            ("ご清聴ありがとうございました。", 90000, 95000),
            ("何かご質問はありますでしょうか？", 96000, 105000),
        ];

        for (text, start, end) in sentences {
            app_handle
                .emit(
                    "asr-progress",
                    ProgressPayload {
                        text: text.to_string(),
                        start_time_ms: start,
                        end_time_ms: end,
                    },
                )
                .unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        app_handle.emit("asr-finished", ()).unwrap();
        log::info!("ASR process finished for: {}", file_path);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(if cfg!(debug_assertions) {
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .level(log::LevelFilter::Debug)
                .build()
        } else {
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .max_file_size(1 * 1024 * 1024)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
                .build()
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_asr_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
