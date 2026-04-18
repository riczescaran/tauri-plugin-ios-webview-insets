import SwiftRs
import Tauri
import WebKit

class IosWebviewInsetsPlugin: Plugin {
  override func load(webview: WKWebView) {
    webview.scrollView.contentInsetAdjustmentBehavior = .never
  }
}

@_cdecl("init_plugin_ios_webview_insets")
func initPlugin() -> Plugin {
  return IosWebviewInsetsPlugin()
}
