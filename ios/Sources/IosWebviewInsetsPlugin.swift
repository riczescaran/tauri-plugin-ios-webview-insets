import SwiftRs
import Tauri
import WebKit

public class IosWebviewInsetsPlugin: Plugin {
  public override func load(webview: WKWebView) {
    webview.scrollView.contentInsetAdjustmentBehavior = .never
  }
}

@_cdecl("init_plugin_ios_webview_insets")
public func initPlugin() -> Plugin {
  return IosWebviewInsetsPlugin()
}
