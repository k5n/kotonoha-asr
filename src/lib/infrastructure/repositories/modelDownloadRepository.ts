import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { fetch } from '@tauri-apps/plugin-http';
import { trace } from '@tauri-apps/plugin-log';

const MODEL_BASE_URL =
  'https://huggingface.co/csukuangfj/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/resolve/main';

export const modelDownloadRepository = {
  async downloadFile(fileName: string): Promise<Uint8Array> {
    const url = `${MODEL_BASE_URL}/${fileName}`;
    trace(`Downloading ${url}`);

    const response = await fetch(url, {
      method: 'GET',
    });

    if (!response.ok) {
      throw new Error(`Failed to download ${fileName}: ${response.status}`);
    }

    const content = await response.arrayBuffer();
    return new Uint8Array(content);
  },

  async downloadFileStream(
    fileName: string,
    onProgress?: (progress: number) => void
  ): Promise<void> {
    const url = `${MODEL_BASE_URL}/${fileName}`;
    const filePath = `models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/${fileName}`;

    trace(`Streaming download ${url} to ${filePath}`);

    // 進捗イベントリスナーを設定
    let unlisten: (() => void) | null = null;
    if (onProgress) {
      unlisten = await listen('download_progress', (event: any) => {
        const payload = event.payload;
        if (payload.fileName === fileName) {
          onProgress(payload.progress);
        }
      });
    }

    try {
      await invoke('download_model_file_stream', {
        url,
        filePath,
      });
    } finally {
      if (unlisten) {
        unlisten();
      }
    }
  },
};
