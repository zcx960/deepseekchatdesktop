# DeepSeek Desktop

[简体中文](README.md) · **English**

A DeepSeek desktop client focused on a small footprint. Built with **Tauri 2, Rust, and the system WebView**, it loads the [official DeepSeek chat website](https://chat.deepseek.com/) directly, preserves the official interface, and requires no API key.

**Measured on macOS Apple Silicon: 2.37 MiB for the app bundle and 1.80 MiB for the DMG installer.**

This is an independent community wrapper, not an official DeepSeek desktop release.

## Why it is lightweight

| Design | Implementation |
| --- | --- |
| Reuse the system browser engine | WKWebView on macOS, WebView2 on Windows, and WebKitGTK on Linux |
| Keep the runtime small | No bundled Chromium or Node.js; Node.js is used only for development and packaging |
| Keep the frontend small | Load the official website directly, with no local frontend framework, frontend build, or development server |
| Keep background work small | No local AI service, background polling, or system tray; closing the main window exits the app |
| Keep native features small | Use system menus, shortcuts, and window management without building a separate interface |
| Optimize release size | Enable size optimization, LTO, symbol stripping, and removal of unused Tauri commands |

The size measurements come from a local **macOS arm64 Release build on October 6, 2026**. They exclude the system WebView, website caches, and development dependencies. Other platforms and future versions may have different sizes.

**Installation size is not runtime memory usage.** The WebView renders the website, so memory usage still depends on the official page, conversation length, and attachments. This project does not yet include a complete memory benchmark.

## Features

- Sign in with a DeepSeek website account and use its chat, history, and attachment interfaces.
- Native menus for new chats, reload, back/forward, zoom, fullscreen, and keeping the main window on top.
- Single-instance operation: launching again focuses the existing window.
- Restore the main window's position, size, and maximized state; the WebView persists sign-in data.
- Open external references in the system browser while keeping official pages and selected sign-in domains inside the app.
- Native copy/paste, file selection, and website sign-in popups.

DeepSeek provides and updates the website features. The app's sign-in state is separate from your system browser.

## Run for development

Install **Node.js 20+**, **stable Rust**, and the [Tauri build prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform.

```sh
git clone https://github.com/zcx960/deepseekchatdesktop.git
cd deepseekchatdesktop
npm ci
npm run dev
```

Sign in on the official page after launch. No API key or local model setup is required.

On macOS, the scripts prefer separately installed Command Line Tools by setting `DEVELOPER_DIR` only for child processes. They do not change the system's `xcode-select` setting, and they preserve an existing `DEVELOPER_DIR`.

## Check and package

```sh
npm run check
npm run build
```

`npm run check` runs Rust formatting checks, tests, and Clippy with warnings treated as errors. To build only the macOS `.app`:

```sh
npm run build:app
```

Build outputs are placed in `src-tauri/target/release/bundle/`.

| Platform | Browser engine | Available outputs | Validation |
| --- | --- | --- | --- |
| macOS Apple Silicon | System WKWebView | `.app`, `.dmg` | Built and launched locally |
| Windows | System WebView2 | Installers | Not tested on a Windows machine |
| Linux | WebKitGTK | AppImage, deb, rpm | Not tested on a Linux machine |

Install the prerequisites and build on the corresponding platform. Windows uses a WebView2 download bootstrapper rather than including the full browser runtime in the installer. Local macOS builds have no Apple Developer signature or notarization.

## Keyboard shortcuts

Use `⌘` on macOS and `Ctrl` on Windows / Linux.

| Action | Shortcut |
| --- | --- |
| New chat | `⌘/Ctrl + N` |
| Reload | `⌘/Ctrl + R` |
| Open the current page in the system browser | `⌘/Ctrl + Shift + B` |
| Back / forward | `⌘/Ctrl + [` / `⌘/Ctrl + ]` |
| Zoom in / out | `⌘/Ctrl + =` / `⌘/Ctrl + -` |
| Reset zoom | `⌘/Ctrl + 0` |
| Toggle fullscreen | `⌘/Ctrl + Shift + F` |
| Close window / quit app | `⌘/Ctrl + W` / `⌘/Ctrl + Q` |
| Copy / paste / select all | Standard system shortcuts |

“New chat” reloads the official chat homepage. The “View” menu includes the main window's always-on-top toggle. Closing the main window exits the entire app; closing a popup closes only that popup.

## Website and local permissions

The remote website receives no Tauri capabilities. The app registers no custom Tauri commands and exposes no global Tauri API. The wrapper does not separately collect or relay accounts, conversations, or files; the DeepSeek website handles them.

The app allows DeepSeek HTTPS pages and explicitly listed Apple, Google, and WeChat sign-in domains inside its WebViews. Other HTTP(S) links open in the system browser. Navigation to schemes such as `file:` and `javascript:` is rejected. All popups use the same navigation rules.

An internet connection is required. If loading fails, reload the page, open it in the system browser from the chat menu, or check the official service status from the help menu. Third-party sign-in providers may restrict embedded browsers; complete authorization flows still need testing.

## Completed validation

On macOS Apple Silicon, October 6, 2026:

- Rust formatting checks, three URL-boundary tests, and strict Clippy checks passed.
- The Release `.app` and DMG were built successfully, and DMG integrity verification passed.
- The development entry point compiled and launched.
- The packaged app displayed both the official sign-in page and the signed-in chat interface.
- A second launch exited normally while the original process remained running.

Message sending, attachment uploads, complete Apple / Google authorization flows, and native interactions on Windows / Linux have not been fully validated.

## Project layout

```text
scripts/                   Development, build, and validation commands
src-tauri/src/main.rs       WebViews, popups, single instance, and app lifecycle
src-tauri/src/menu.rs       Native menus and keyboard shortcuts
src-tauri/src/navigation.rs URL boundaries and tests
src-tauri/tauri.conf.json   Window, permission, and packaging settings
src-tauri/icons/            App icons and source information
```

## License

The project's original code is open source under the [MIT License](LICENSE).

The DeepSeek name and whale artwork belong to their respective rights holders and are not original code licensed under this project's MIT license. See the [icon source notes](src-tauri/icons/README.md).
