use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

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
pub fn start_asr_process(app_handle: AppHandle, file_path: String) {
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
