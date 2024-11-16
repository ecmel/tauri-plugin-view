import { invoke } from "@tauri-apps/api/core";

export async function view(path: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:view|view", {
    payload: {
      path,
    },
  }).then((r) => (r.value ? r.value : null));
}
