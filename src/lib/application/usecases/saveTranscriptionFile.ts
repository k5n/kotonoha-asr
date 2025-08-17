import { fileRepository } from '$lib/infrastructure/repositories/fileRepository';

/**
 * 文字起こし結果をファイルとして保存するユースケース
 */
export async function saveTranscriptionFile(filePath: string, content: string): Promise<void> {
  await fileRepository.saveTranscriptionFile(filePath, content);
}
