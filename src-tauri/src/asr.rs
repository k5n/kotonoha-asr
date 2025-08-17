use sherpa_rs::transducer::{TransducerConfig, TransducerRecognizer};
use std::thread;
use std::time::Instant;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager};

const CHUNK_SECONDS: usize = 30;
const OVERLAP_SECONDS: usize = 8;
const REQUIRED_SAMPLE_RATE: u32 = 16_000;
const FEATURE_DIM: i32 = 80;

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

#[derive(Debug, Clone)]
struct Sentence {
    start: f32,
    end: f32,
    text: String,
}

// チャンクの中央部分の範囲計算
fn central_range_for_chunk(
    start: usize,
    end: usize,
    samples_len: usize,
    sample_rate: u32,
    chunk_seconds: usize,
    overlap_seconds: usize,
) -> (f32, f32) {
    let central_start = if start == 0 {
        0.0
    } else {
        overlap_seconds as f32 / 2.0
    };
    let central_end = if end == samples_len {
        (end - start) as f32 / sample_rate as f32
    } else {
        chunk_seconds as f32 - overlap_seconds as f32 / 2.0
    };
    (central_start, central_end)
}

// トークンとタイムスタンプの中央部分抽出
fn filter_central_tokens(
    tokens: &[String],
    timestamps: &[f32],
    chunk_start_sec: f32,
    central_start: f32,
    central_end: f32,
) -> (Vec<String>, Vec<f32>) {
    let mut filtered_tokens = Vec::new();
    let mut filtered_timestamps = Vec::new();
    for (token, &timestamp) in tokens.iter().zip(timestamps.iter()) {
        let abs_timestamp = timestamp + chunk_start_sec;
        let rel_timestamp = timestamp;
        if rel_timestamp >= central_start && rel_timestamp < central_end {
            filtered_tokens.push(token.clone());
            filtered_timestamps.push(abs_timestamp);
        }
    }
    (filtered_tokens, filtered_timestamps)
}

// チャンクから中央部分のトークンと絶対時間を抽出
fn extract_central_tokens(
    samples: &[f32],
    start: usize,
    end: usize,
    sample_rate: u32,
    chunk_seconds: usize,
    overlap_seconds: usize,
    recognizer: &mut TransducerRecognizer,
) -> (Vec<String>, Vec<f32>) {
    let chunk = &samples[start..end];
    let result = recognizer.transcribe(sample_rate, chunk);

    let (central_start, central_end) = central_range_for_chunk(
        start,
        end,
        samples.len(),
        sample_rate,
        chunk_seconds,
        overlap_seconds,
    );
    let chunk_start_sec = start as f32 / sample_rate as f32;

    filter_central_tokens(
        &result.tokens,
        &result.timestamps,
        chunk_start_sec,
        central_start,
        central_end,
    )
}

// トークン列とタイムスタンプ列からセンテンス情報を抽出
fn extract_sentences(
    tokens: &[String],
    timestamps: &[f32],
) -> (Vec<Sentence>, Vec<String>, Vec<f32>) {
    let mut sentences = Vec::new();
    let mut sentence_tokens = Vec::new();
    let mut sentence_timestamps = Vec::new();
    let mut sentence_start: Option<f32> = None;

    let len = tokens.len();
    for i in 0..len {
        let token = &tokens[i];
        let timestamp = timestamps[i];
        if sentence_start.is_none() {
            sentence_start = Some(timestamp);
        }
        sentence_tokens.push(token.clone());
        sentence_timestamps.push(timestamp);

        let next_token = if i + 1 < len {
            Some(tokens[i + 1].as_str())
        } else {
            None
        };

        if is_sentence_end(token, next_token) {
            let start = sentence_start.unwrap();
            let end = timestamp;
            let sentence = sentence_tokens.join("");
            sentences.push(Sentence {
                start,
                end,
                text: sentence.clone(),
            });
            sentence_tokens.clear();
            sentence_timestamps.clear();
            sentence_start = None;
        }
    }

    (sentences, sentence_tokens, sentence_timestamps)
}

// センテンス区切り
fn is_sentence_end(token: &str, next_token: Option<&str>) -> bool {
    let sentence_end_tokens = [".", "?", "!", "!?", "?!"];
    if sentence_end_tokens.contains(&token) {
        // 次も文末記号なら文末としない
        if let Some(next) = next_token {
            if sentence_end_tokens.contains(&next) {
                return false;
            }
        }
        return true;
    }
    false
}

fn chunk_indices(
    total_samples: usize,
    chunk_size: usize,
    overlap_size: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut start = 0;
    std::iter::from_fn(move || {
        if start >= total_samples {
            None
        } else {
            let end = std::cmp::min(start + chunk_size, total_samples);
            let current = (start, end);
            start = if end == total_samples {
                end
            } else {
                end - overlap_size
            };
            Some(current)
        }
    })
}

fn process_chunks(
    samples: &[f32],
    sample_rate: u32,
    chunk_seconds: usize,
    overlap_seconds: usize,
    recognizer: &mut TransducerRecognizer,
    app_handle: &AppHandle,
) -> Vec<Sentence> {
    let chunk_size = chunk_seconds * sample_rate as usize;
    let overlap_size = overlap_seconds * sample_rate as usize;
    let mut all_sentences: Vec<Sentence> = Vec::new();
    let mut carry_tokens: Vec<String> = Vec::new();
    let mut carry_timestamps: Vec<f32> = Vec::new();

    for (start, end) in chunk_indices(samples.len(), chunk_size, overlap_size) {
        let (local_tokens, local_timestamps) = extract_central_tokens(
            samples,
            start,
            end,
            sample_rate,
            chunk_seconds,
            overlap_seconds,
            recognizer,
        );

        let mut tokens = carry_tokens.clone();
        let mut timestamps = carry_timestamps.clone();
        tokens.extend(local_tokens);
        timestamps.extend(local_timestamps);

        let (sentences, remain_tokens, remain_timestamps) = extract_sentences(&tokens, &timestamps);
        for sentence in &sentences {
            app_handle
                .emit(
                    "asr-progress",
                    ProgressPayload {
                        text: sentence.text.clone(),
                        start_time_ms: (sentence.start * 1000.0) as u64,
                        end_time_ms: (sentence.end * 1000.0) as u64,
                    },
                )
                .unwrap();
        }
        all_sentences.extend(sentences.clone());

        carry_tokens = remain_tokens;
        carry_timestamps = remain_timestamps;
    }

    // 最後に未確定分が残っていれば文末記号なしとして追加
    if !carry_tokens.is_empty() {
        let start = *carry_timestamps.first().unwrap_or(&0.0);
        let end = *carry_timestamps.last().unwrap_or(&start);
        let sentence_text = carry_tokens.join("");
        app_handle
            .emit(
                "asr-progress",
                ProgressPayload {
                    text: sentence_text.clone(),
                    start_time_ms: (start * 1000.0) as u64,
                    end_time_ms: (end * 1000.0) as u64,
                },
            )
            .unwrap();
        all_sentences.push(Sentence {
            start,
            end,
            text: sentence_text,
        });
    }

    all_sentences
}

#[tauri::command]
pub fn start_asr_process(app_handle: AppHandle, file_path: String) {
    log::info!("Starting ASR process for: {}", file_path);

    thread::spawn(move || {
        let start_t = Instant::now();

        let samples = match crate::audio_converter::convert_to_mono_f32_16khz(&file_path) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to read or convert audio file: {}", e);
                app_handle
                    .emit(
                        "asr-error",
                        format!("音声ファイルの読み込みまたは変換に失敗しました: {}", e),
                    )
                    .unwrap();
                return;
            }
        };
        let sample_rate = REQUIRED_SAMPLE_RATE;

        let model_dir_path = match app_handle.path().resolve(
            "models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8",
            BaseDirectory::AppLocalData,
        ) {
            Ok(path) => path,
            Err(e) => {
                let err_msg = format!("モデルディレクトリの解決に失敗しました: {}", e);
                log::error!("{}", err_msg);
                app_handle.emit("asr-error", err_msg).unwrap();
                return;
            }
        };

        let cpus = num_cpus::get();
        // NOTE: CPU数が多いからといってスレッド数を増やしすぎると逆にパフォーマンスが落ちる。
        let num_threads = std::cmp::max(1, std::cmp::min(6, cpus / 2));
        let config = TransducerConfig {
            decoder: model_dir_path
                .join("decoder.int8.onnx")
                .to_str()
                .unwrap()
                .to_string(),
            encoder: model_dir_path
                .join("encoder.int8.onnx")
                .to_str()
                .unwrap()
                .to_string(),
            joiner: model_dir_path
                .join("joiner.int8.onnx")
                .to_str()
                .unwrap()
                .to_string(),
            tokens: model_dir_path
                .join("tokens.txt")
                .to_str()
                .unwrap()
                .to_string(),
            num_threads: num_threads as i32,
            sample_rate: REQUIRED_SAMPLE_RATE as i32,
            feature_dim: FEATURE_DIM,
            debug: true,
            model_type: "nemo_transducer".to_string(),
            ..Default::default()
        };

        let mut recognizer = match TransducerRecognizer::new(config) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to create recognizer: {}", e);
                app_handle
                    .emit("asr-error", format!("認識器の初期化に失敗しました: {}", e))
                    .unwrap();
                return;
            }
        };

        let total_duration_ms = (samples.len() as f64 / sample_rate as f64 * 1000.0) as u64;
        app_handle
            .emit("asr-started", StartedPayload { total_duration_ms })
            .unwrap();

        process_chunks(
            &samples,
            sample_rate,
            CHUNK_SECONDS,
            OVERLAP_SECONDS,
            &mut recognizer,
            &app_handle,
        );

        log::info!("Time taken for decode: {:?}", start_t.elapsed());
        app_handle.emit("asr-finished", ()).unwrap();
        log::info!("ASR process finished for: {}", file_path);
    });
}
