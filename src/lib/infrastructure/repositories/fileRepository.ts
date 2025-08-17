import { BaseDirectory, exists, mkdir, writeFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { trace } from '@tauri-apps/plugin-log';

const MODEL_DIR_NAME = 'models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8';

async function ensureModelDirExists(): Promise<void> {
  if (!(await exists(MODEL_DIR_NAME, { baseDir: BaseDirectory.AppLocalData }))) {
    trace(`Directory does not exist, creating: ${MODEL_DIR_NAME}`);
    await mkdir(MODEL_DIR_NAME, { recursive: true, baseDir: BaseDirectory.AppLocalData });
  }
}

export const fileRepository = {
  async modelFileExists(fileName: string): Promise<boolean> {
    return await exists(`${MODEL_DIR_NAME}/${fileName}`, { baseDir: BaseDirectory.AppLocalData });
  },

  async saveModelFile(fileName: string, content: Uint8Array): Promise<void> {
    await ensureModelDirExists();
    const filePath = `${MODEL_DIR_NAME}/${fileName}`;
    await writeFile(filePath, content, { baseDir: BaseDirectory.AppLocalData });
    trace(`Saved binary file: ${filePath}`);
  },

  async saveModelTextFile(fileName: string, content: string): Promise<void> {
    await ensureModelDirExists();
    const filePath = `${MODEL_DIR_NAME}/${fileName}`;
    await writeTextFile(filePath, content, { baseDir: BaseDirectory.AppLocalData });
    trace(`Saved text file: ${filePath}`);
  },
};
