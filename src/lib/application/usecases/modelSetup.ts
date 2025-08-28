import { fileRepository } from '$lib/infrastructure/repositories/fileRepository';
import { modelDownloadRepository } from '$lib/infrastructure/repositories/modelDownloadRepository';
import { error, trace } from '@tauri-apps/plugin-log';

const FILES_TO_DOWNLOAD = [
  { name: 'decoder.int8.onnx', isBinary: true },
  { name: 'encoder.int8.onnx', isBinary: true },
  { name: 'joiner.int8.onnx', isBinary: true },
  { name: 'tokens.txt', isBinary: false },
];

/**
 * アプリケーションが必要とするモデルファイルがローカルに存在するかを確認し、
 * 存在しない場合はHugging Faceからダウンロードする。
 * @param onProgress 進捗を報告するコールバック (0-100)
 */
export async function ensureModelIsReady(onProgress: (progress: number) => void): Promise<void> {
  try {
    const checkResults = await Promise.all(
      FILES_TO_DOWNLOAD.map((file) => fileRepository.modelFileExists(file.name))
    );
    const allFilesExist = checkResults.every((r) => r);

    if (allFilesExist) {
      trace('All model files already exist.');
      onProgress(100);
      return;
    }
    trace('Some model files are missing. Starting download...');

    let completedFiles = 0;
    const totalFiles = FILES_TO_DOWNLOAD.length;

    for (const file of FILES_TO_DOWNLOAD) {
      if (file.isBinary) {
        // 大きなバイナリファイルはストリーミングダウンロード
        await modelDownloadRepository.downloadFileStream(file.name, (fileProgress) => {
          const totalProgress = Math.round(
            ((completedFiles + fileProgress / 100) / totalFiles) * 100
          );
          onProgress(totalProgress);
        });
      } else {
        // 小さなテキストファイルは従来通り
        const content = await modelDownloadRepository.downloadFile(file.name);
        const textContent = new TextDecoder().decode(content);
        await fileRepository.saveModelTextFile(file.name, textContent);
      }

      completedFiles++;
      onProgress(Math.round((completedFiles / totalFiles) * 100));
    }

    trace('Model download completed.');
  } catch (e) {
    error(`Failed to setup model: ${e}`);
    throw e; // エラーを呼び出し元に伝える
  }
}
