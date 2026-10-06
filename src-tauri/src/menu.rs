use std::{collections::HashMap, sync::Mutex};
use tauri::{
    menu::{AboutMetadata, CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Manager, Wry,
};

use crate::navigation::{is_official, CHAT_URL};

#[derive(Default)]
pub struct ViewState {
    zoom: Mutex<HashMap<String, f64>>,
}

pub fn create(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let about = PredefinedMenuItem::about(
        app,
        Some("关于 DeepSeek Desktop"),
        Some(AboutMetadata {
            name: Some("DeepSeek Desktop".into()),
            version: Some(app.package_info().version.to_string()),
            comments: Some("独立制作的轻量桌面封装，直接访问 DeepSeek 官方网页。".into()),
            ..Default::default()
        }),
    )?;
    let application = Submenu::new(app, "DeepSeek Desktop", true)?;
    application.append_items(&[&about, &PredefinedMenuItem::separator(app)?])?;
    #[cfg(target_os = "macos")]
    application.append_items(&[
        &PredefinedMenuItem::hide(app, Some("隐藏 DeepSeek Desktop"))?,
        &PredefinedMenuItem::hide_others(app, Some("隐藏其他"))?,
        &PredefinedMenuItem::show_all(app, Some("显示全部"))?,
        &PredefinedMenuItem::separator(app)?,
    ])?;
    application.append(&MenuItem::with_id(
        app,
        "quit",
        "退出 DeepSeek Desktop",
        true,
        Some("CmdOrCtrl+Q"),
    )?)?;
    let chat = Submenu::with_items(
        app,
        "对话",
        true,
        &[
            &MenuItem::with_id(app, "new_chat", "新对话", true, Some("CmdOrCtrl+N"))?,
            &MenuItem::with_id(app, "reload", "重新加载", true, Some("CmdOrCtrl+R"))?,
            &MenuItem::with_id(
                app,
                "open_browser",
                "在浏览器中打开",
                true,
                Some("CmdOrCtrl+Shift+B"),
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "close", "关闭窗口", true, Some("CmdOrCtrl+W"))?,
        ],
    )?;
    let edit = Submenu::new(app, "编辑", true)?;
    #[cfg(target_os = "macos")]
    edit.append_items(&[
        &PredefinedMenuItem::undo(app, Some("撤销"))?,
        &PredefinedMenuItem::redo(app, Some("重做"))?,
        &PredefinedMenuItem::separator(app)?,
    ])?;
    #[cfg(not(target_os = "macos"))]
    edit.append_items(&[
        &MenuItem::with_id(app, "undo", "撤销", true, Some("CmdOrCtrl+Z"))?,
        &MenuItem::with_id(app, "redo", "重做", true, Some("CmdOrCtrl+Shift+Z"))?,
        &PredefinedMenuItem::separator(app)?,
    ])?;
    edit.append_items(&[
        &PredefinedMenuItem::cut(app, Some("剪切"))?,
        &PredefinedMenuItem::copy(app, Some("复制"))?,
        &PredefinedMenuItem::paste(app, Some("粘贴"))?,
        &PredefinedMenuItem::select_all(app, Some("全选"))?,
    ])?;
    let view = Submenu::with_items(
        app,
        "显示",
        true,
        &[
            &MenuItem::with_id(app, "back", "后退", true, Some("CmdOrCtrl+["))?,
            &MenuItem::with_id(app, "forward", "前进", true, Some("CmdOrCtrl+]"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "zoom_in", "放大", true, Some("CmdOrCtrl+="))?,
            &MenuItem::with_id(app, "zoom_out", "缩小", true, Some("CmdOrCtrl+-"))?,
            &MenuItem::with_id(app, "zoom_reset", "实际大小", true, Some("CmdOrCtrl+0"))?,
            &PredefinedMenuItem::separator(app)?,
            &CheckMenuItem::with_id(
                app,
                "always_on_top",
                "主窗口置顶",
                true,
                false,
                None::<&str>,
            )?,
            &MenuItem::with_id(
                app,
                "fullscreen",
                "切换全屏",
                true,
                Some("CmdOrCtrl+Shift+F"),
            )?,
        ],
    )?;
    let window = Submenu::with_items(
        app,
        "窗口",
        true,
        &[
            &MenuItem::with_id(app, "minimize", "最小化", true, Some("CmdOrCtrl+M"))?,
            &MenuItem::with_id(app, "maximize", "缩放", true, None::<&str>)?,
        ],
    )?;
    let help = Submenu::with_items(
        app,
        "帮助",
        true,
        &[
            &MenuItem::with_id(app, "website", "DeepSeek 官网", true, None::<&str>)?,
            &MenuItem::with_id(app, "status", "服务状态", true, None::<&str>)?,
        ],
    )?;
    Menu::with_items(app, &[&application, &chat, &edit, &view, &window, &help])
}

pub fn handle(app: &AppHandle, id: &str) -> tauri::Result<()> {
    match id {
        "quit" => {
            app.exit(0);
            return Ok(());
        }
        "website" => {
            crate::open_external(&"https://www.deepseek.com/".parse().unwrap());
            return Ok(());
        }
        "status" => {
            crate::open_external(&"https://status.deepseek.com/".parse().unwrap());
            return Ok(());
        }
        _ => {}
    }

    let Some(main) = app.get_webview_window("main") else {
        return Ok(());
    };
    let window = app
        .webview_windows()
        .into_values()
        .find(|window| window.is_focused().unwrap_or(false))
        .unwrap_or_else(|| main.clone());

    match id {
        "new_chat" => {
            main.navigate(CHAT_URL.parse().unwrap())?;
            main.set_focus()?;
        }
        "reload" => window.reload()?,
        "close" => window.close()?,
        "minimize" => window.minimize()?,
        "maximize" => {
            if window.is_maximized()? {
                window.unmaximize()?;
            } else {
                window.maximize()?;
            }
        }
        #[cfg(not(target_os = "macos"))]
        "undo" => window.eval("document.execCommand('undo')")?,
        #[cfg(not(target_os = "macos"))]
        "redo" => window.eval("document.execCommand('redo')")?,
        "open_browser" => {
            let url = window.url()?;
            if is_official(&url) {
                crate::open_external(&url);
            } else {
                crate::open_external(&CHAT_URL.parse().unwrap());
            }
        }
        "back" => window.eval("window.history.back()")?,
        "forward" => window.eval("window.history.forward()")?,
        "zoom_in" | "zoom_out" | "zoom_reset" => {
            let state = app.state::<ViewState>();
            let mut levels = state.zoom.lock().unwrap_or_else(|error| error.into_inner());
            let level = levels.entry(window.label().into()).or_insert(1.0);
            let next: f64 = match id {
                "zoom_in" => *level + 0.1,
                "zoom_out" => *level - 0.1,
                _ => 1.0,
            };
            let next = ((next * 10.0).round() / 10.0).clamp(0.5, 2.0);
            window.set_zoom(next)?;
            *level = next;
        }
        "always_on_top" => {
            if let Some(item) = app
                .menu()
                .and_then(|menu| menu.get("always_on_top"))
                .and_then(|item| item.as_check_menuitem().cloned())
            {
                main.set_always_on_top(item.is_checked()?)?;
            }
        }
        "fullscreen" => window.set_fullscreen(!window.is_fullscreen()?)?,
        _ => {}
    }
    Ok(())
}

pub fn forget_window(app: &AppHandle, label: &str) {
    app.state::<ViewState>()
        .zoom
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .remove(label);
}
