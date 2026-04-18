# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-19

### Added

- Initial release: Tauri 2 iOS plugin setting `WKWebView.scrollView.contentInsetAdjustmentBehavior = .never`.
- Rust `init()` with `register_ios_plugin` / `ios_plugin_binding!` so the Swift `Plugin` loads correctly.
