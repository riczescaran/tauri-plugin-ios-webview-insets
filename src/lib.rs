use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_ios_webview_insets);

/// iOS: `WKWebView.scrollView.contentInsetAdjustmentBehavior = .never` so UIKit does not add
/// extra safe-area **content** insets (which leaves an unpainted band and exposes the window color).
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ios-webview-insets")
        .setup(|_app, api| {
            #[cfg(target_os = "ios")]
            {
                let _ = api.register_ios_plugin(init_plugin_ios_webview_insets)?;
            }
            #[cfg(not(target_os = "ios"))]
            let _ = api;
            Ok(())
        })
        .build()
}
