// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, MenuEntry, GlobalWindowEvent, WindowMenuEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTray, AppHandle, SystemTrayEvent};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    let str = String::from(name);
    str
}
#[tauri::command]
fn hello(helloMsg: String) -> String {
    let mut str = String::from("");
    if helloMsg.len() > 0 {
        str = String::from("你好, 已经设置过值了");
    } else {
        str = String::from("你好, tauri");
    }
    str
}
// 关闭loading页面的 桥接方法
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}
// 主函数
fn main() {
    tauri::Builder::default()
        .menu(window_menu())
        .system_tray(window_system_tray())
        .on_window_event(window_event)
        .on_menu_event(window_on_menu)
        .on_system_tray_event(system_tray_event)
        .invoke_handler(tauri::generate_handler![greet, hello, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// 系统托盘图标点击事件
fn system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
          position: _,
          size: _,
          ..
        } => {
          println!("system tray received a left click");
          let window = app.get_window("main").unwrap();
              window.show().unwrap();
        }
        SystemTrayEvent::RightClick {
          position: _,
          size: _,
          ..
        } => {
          println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
          position: _,
          size: _,
          ..
        } => {
          println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
          match id.as_str() {
            "quit" => {
              std::process::exit(0);
            }
            "hide" => {
              let window = app.get_window("main").unwrap();
              window.show().unwrap();
            }
            _ => {}
          }
        }
        _ => {}
      }
}
// 设置系统图盘
fn window_system_tray() -> SystemTray {
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new();
        // .add_item(quit)
        // .add_native_item(SystemTrayMenuItem::Separator);
        // .add_item(hide);
    // tray_menu
    let tray: SystemTray = SystemTray::new().with_menu(tray_menu);
    tray
}
// 设置menu菜单
fn window_menu() -> Menu {
    // Menu::with_items([MenuEntry::Submenu(
    //     Submenu::new(
    //         title: "工具",
    //         Menu::with_items([
    //             MenuItem::CloseWindow.into(),
    //             CustomMenuItem::new(id: "hello", title: "Hello").into(),
    //         ]),
    //     )
    // )])
    let quit = CustomMenuItem::new("js".to_string(), "JS");
    let close = CustomMenuItem::new("baidu".to_string(), "百度");
    let submenu = Submenu::new("工具", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
    .add_submenu(submenu);
        menu
}
// menu菜单的点击事件
fn window_on_menu(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "js" => {
          println!("JS");
          let url = tauri::WindowBuilder::new(
            &event.window().app_handle(),
            "local",
            tauri::WindowUrl::App("/login".into())
          ).build();
        }
        "baidu" => {
            println!("百度");
        }
        _ => {}
      }
}
// 系统窗口关闭事件
fn window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let window = event.window().clone();
            tauri::api::dialog::confirm(Some(&event.window()),"提示","你确定要关闭tauri吗?",move|answer| {
                    if answer {
                        let _ = window.close();
                    }
                },
            );
        }
        _ => {}
    }
}