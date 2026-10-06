#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;
mod navigation;

use std::sync::atomic::{AtomicU64, Ordering};
use tauri::{
    webview::{NewWindowResponse, WebviewWindowBuilder},
    AppHandle, Manager, WebviewUrl, WindowEvent,
};
use tauri_plugin_window_state::StateFlags;

static POPUP_ID: AtomicU64 = AtomicU64::new(1);

fn open_external(url: &tauri::Url) {
    if navigation::may_open_in_browser(url) && open::that_detached(url.as_str()).is_err() {
        eprintln!("无法打开系统浏览器。");
    }
}

fn configure_webview<'a>(
    builder: WebviewWindowBuilder<'a, tauri::Wry, AppHandle>,
    app: &AppHandle,
) -> WebviewWindowBuilder<'a, tauri::Wry, AppHandle> {
    let popup_app = app.clone();
    builder
        .disable_drag_drop_handler()
        .enable_clipboard_access()
        .devtools(cfg!(debug_assertions))
        .on_navigation(|url| {
            if navigation::may_embed(url) {
                true
            } else {
                open_external(url);
                false
            }
        })
        .on_new_window(move |url, features| {
            if !navigation::may_embed(&url) {
                open_external(&url);
                return NewWindowResponse::Deny;
            }

            let label = format!("popup-{}", POPUP_ID.fetch_add(1, Ordering::Relaxed));
            let builder = WebviewWindowBuilder::new(
                &popup_app,
                label,
                WebviewUrl::External("about:blank".parse().unwrap()),
            )
            // Preserve the website's window.opener and authentication session.
            .window_features(features)
            .title("DeepSeek")
            .inner_size(900.0, 720.0);

            match configure_webview(builder, &popup_app).build() {
                Ok(window) => NewWindowResponse::Create { window },
                Err(_) => {
                    eprintln!("无法创建网页弹出窗口。");
                    NewWindowResponse::Deny
                }
            }
        })
        .on_document_title_changed(|window, title| {
            let title = if title.trim().is_empty() {
                "DeepSeek".into()
            } else {
                title
            };
            if window.set_title(&title).is_err() {
                eprintln!("无法更新窗口标题。");
            }
        })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::POSITION | StateFlags::SIZE | StateFlags::MAXIMIZED)
                .with_filter(|label| label == "main")
                .build(),
        )
        .manage(menu::ViewState::default())
        .menu(menu::create)
        .on_menu_event(|app, event| {
            if let Err(error) = menu::handle(app, event.id().as_ref()) {
                eprintln!("菜单操作失败：{error}");
            }
        })
        .setup(|app| {
            let handle = app.handle();
            let config = app
                .config()
                .app
                .windows
                .iter()
                .find(|window| window.label == "main")
                .ok_or("缺少主窗口配置")?;
            let builder = WebviewWindowBuilder::from_config(handle, config)?;
            configure_webview(builder, handle).build()?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if matches!(event, WindowEvent::Destroyed) {
                menu::forget_window(window.app_handle(), window.label());
                // Closing the main window releases all WebViews and processes.
                if window.label() == "main" {
                    window.app_handle().exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("DeepSeek Desktop 启动失败");
}
