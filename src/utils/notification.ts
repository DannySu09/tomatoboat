import { requestPermission, sendNotification, isPermissionGranted } from '@tauri-apps/api/notification';

export async function notify(title: string, content: string) {
  if (!isPermissionGranted) {
    await requestPermission();
  }

  return sendNotification({
    title,
    body: content
  });
}