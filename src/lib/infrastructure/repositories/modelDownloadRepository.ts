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
};
