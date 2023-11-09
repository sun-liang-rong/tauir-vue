// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use uuid::Uuid;
#[derive(serde::Serialize)]
struct FileEntry {
    key: String,
    title: String,
    children: Vec<FileEntry>,
    path: String,
    is_dir: bool
}
use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, Menu, MenuEntry, MenuItem, Submenu,
    SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowMenuEvent,
};
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
//在系统里创建一个文件夹的方法
#[tauri::command]
async fn my_read_file(path: String) -> Result<(), String> {
    match std::fs::create_dir(&path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

//在系统里查看一个文件夹是不是存在
#[tauri::command]
fn check_directory_exists(path: String) -> bool {
    let directory_path = std::path::Path::new(&path);
    directory_path.is_dir()
}
//查看某一个文件夹下面是否还有文件夹
#[tauri::command]
fn check_subdirectories_exist(path: String) -> bool {
    let directory = match std::fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(_) => return false,
    };
    for entry in directory {
        if let Ok(entry) = entry {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}
//查看某个文件夹下面的所有文件 及文件夹
#[tauri::command]
fn get_directory_contents(path: String) -> Vec<FileEntry> {
    let directory = match std::fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };

    let mut entries = Vec::new();

    for entry_result in directory {
        if let Ok(entry) = entry_result {
            let entry_path = entry.path();
            let name = entry_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let children = get_directory_contents(entry_path.to_string_lossy().into_owned());
                entries.push(FileEntry {
                    title: name,
                    path: entry_path.to_string_lossy().into_owned(),
                    children,
                    key: Uuid::new_v4().to_string(),
                    is_dir: true
                });
            } else {
                entries.push(FileEntry {
                    title: name,
                    path: entry_path.to_string_lossy().into_owned(),
                    children: Vec::new(),
                    key: Uuid::new_v4().to_string(),
                    is_dir: false
                });
            }
        }
    }

    entries
}
// 创建文件的方法
#[tauri::command]
fn create_file(path: String) -> Result<(), String> {
    let file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    // 可选：根据需要对新文件进行进一步的处理

    Ok(())
}
// 主函数
fn main() {
    tauri::Builder::default()
        // .menu(window_menu())
        .system_tray(window_system_tray())
        .on_window_event(window_event)
        // .on_menu_event(window_on_menu)
        .on_system_tray_event(system_tray_event)
        .invoke_handler(tauri::generate_handler![
            greet,
            hello,
            close_splashscreen,
            my_read_file,
            check_directory_exists,
            check_subdirectories_exist,
            get_directory_contents,
            create_file
        ])
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
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        },
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
// fn window_menu() -> Menu {
//     // Menu::with_items([MenuEntry::Submenu(
//     //     Submenu::new(
//     //         title: "工具",
//     //         Menu::with_items([
//     //             MenuItem::CloseWindow.into(),
//     //             CustomMenuItem::new(id: "hello", title: "Hello").into(),
//     //         ]),
//     //     )
//     // )])
//     let quit = CustomMenuItem::new("js".to_string(), "JS");
//     let close = CustomMenuItem::new("baidu".to_string(), "百度");
//     let submenu = Submenu::new("工具", Menu::new().add_item(quit).add_item(close));
//     let menu = Menu::new()
//     .add_submenu(submenu);
//         menu
// }
// menu菜单的点击事件
// fn window_on_menu(event: WindowMenuEvent) {
//     match event.menu_item_id() {
//         "js" => {
//           println!("JS");
//           let url = tauri::WindowBuilder::new(
//             &event.window().app_handle(),
//             "local",
//             tauri::WindowUrl::App("/login".into())
//           ).build();
//         }
//         "baidu" => {
//             println!("百度");
//         }
//         _ => {}
//       }
// }
// 系统窗口关闭事件
fn window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let window = event.window().clone();
            tauri::api::dialog::confirm(
                Some(&event.window()),
                "提示",
                "你确定要关闭tauri吗?",
                move |answer| {
                    if answer {
                        let _ = window.close();
                    }
                },
            );
        }
        _ => {}
    }
}
