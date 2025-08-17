/**
 * ミリ秒を HH:MM:SS.ms 形式の文字列にフォーマットします。
 * @param ms - フォーマットするミリ秒
 * @returns フォーマットされた文字列
 */
export function formatTime(ms: number): string {
  if (ms < 0) ms = 0;
  const date = new Date(ms);
  const hours = String(date.getUTCHours()).padStart(2, '0');
  const minutes = String(date.getUTCMinutes()).padStart(2, '0');
  const seconds = String(date.getUTCSeconds()).padStart(2, '0');
  const milliseconds = String(date.getUTCMilliseconds()).padStart(3, '0');
  return `${hours}:${minutes}:${seconds}.${milliseconds}`;
}
