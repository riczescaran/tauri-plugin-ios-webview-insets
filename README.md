# tauri-plugin-ios-webview-insets

[Tauri](https://tauri.app/) **v2** plugin for **iOS** that sets:

```swift
webview.scrollView.contentInsetAdjustmentBehavior = .never
```

Without this, UIKit can apply automatic **safe-area content insets** to the inner `UIScrollView` of `WKWebView`, which **shrinks the painted web surface** and exposes the window `backgroundColor` as a large band (often at the bottom, above the home indicator). This plugin does **not** replace CSS `env(safe-area-inset-*)` or crates like `tauri-plugin-safe-area-insets-css`; it fixes a **native scroll-view** behavior so your layout can paint edge-to-edge.

## Requirements

- Tauri **2.x**
- iOS builds: **macOS** with Xcode (Swift package is linked via `tauri-plugin` build + `swift-rs`)

## Install

**`src-tauri/Cargo.toml`**

```toml
[target.'cfg(target_os = "ios")'.dependencies]
tauri-plugin-ios-webview-insets = "0.1"
# or path / git:
# tauri-plugin-ios-webview-insets = { path = "../tauri-plugin-ios-webview-insets" }
# tauri-plugin-ios-webview-insets = { git = "https://github.com/riczescaran/tauri-plugin-ios-webview-insets" }
```

**`src-tauri/src/lib.rs`**

Register **before** other plugins that touch the webview or safe area on iOS, for example:

```rust
#[cfg(target_os = "ios")]
let builder = builder
    .plugin(tauri_plugin_ios_webview_insets::init())
    .plugin(tauri_plugin_barcode_scanner::init());
```

No extra **capabilities** entries are required (this crate exposes no commands).

**HTML:** use `viewport-fit=cover` on your viewport meta tag so `env(safe-area-inset-*)` works as expected.

## Development

From this repository root:

```bash
cargo check
cargo check --target aarch64-apple-ios   # on macOS, with iOS target installed
```

The first iOS-target build populates `../.tauri/tauri-api` next to `ios/` (generated; gitignored).

## License

Licensed under the [MIT license](LICENSE-MIT).
