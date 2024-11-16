# Tauri Mobile View Plugin

A mobile plugin for [Tauri](https://tauri.app/) for viewing and sharing files.

## Installation

```sh
npm install tauri-plugin-view-api
```

**`src-tauri/Cargo.toml`**

```ini
[dependencies]
tauri-plugin-view = "0.0.5"
```

**`src-tauri/src/lib.rs`**

```rust
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_view::init())
    .run(tauri::generate_context!())
    .expect("...");
}
```

**`src-tauri/capabilities/default.json`**

```json
"permissions": [
  "core:default",
  "view:allow-view"
]
```

### Android

Application file paths should be replaced with `../`

**`src-tauri/gen/android/app/src/main/res/xml/file_paths.xml`**

```xml
<?xml version="1.0" encoding="utf-8"?>
<paths xmlns:android="http://schemas.android.com/apk/res/android">
  <external-path name="my_images" path="../" />
  <cache-path name="my_cache_images" path="../" />
</paths>
```

## Usage

```ts
import { view } from "tauri-plugin-view-api";
import { appDataDir, resolve } from "@tauri-apps/api/path";

const path = await resolve(await appDataDir(), "sample.pdf");

await view(path);
```

**Note:** On desktop, `view` function is noop and just returns the given path.

## Contributing

iOS code is in [`ios/Sources/ViewPlugin.swift`](https://github.com/ecmel/tauri-plugin-view/blob/main/ios/Sources/ViewPlugin.swift), Android code is in [`android/src/main/java/ViewPlugin.kt`](https://github.com/ecmel/tauri-plugin-view/blob/main/android/src/main/java/ViewPlugin.kt) and the desktop code is in [`src/desktop.rs`](https://github.com/ecmel/tauri-plugin-view/blob/main/src/desktop.rs)

Pull requests are welcomed.
