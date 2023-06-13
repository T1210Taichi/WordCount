// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, Manager};

mod tokenizer;

#[tauri::command]
fn get_noun(text: String) -> Vec::<String>{
    tokenizer::get_noun(text)
}
#[tauri::command]
fn get_count_noun(text: String) -> usize{
    tokenizer::get_noun(text).len()
}
#[tauri::command]
fn get_top_noun(text: String) -> String{
    let vec = tokenizer::get_top_noun(text);
    let mut string:String = vec.join(", ");
    return string;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            SystemTray::new()
                .with_id("main")
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("menu1", "Menu1"))
                        .add_item(CustomMenuItem::new("quit", "Quit"))
                )
                .on_event(move |event| {
                    if let SystemTrayEvent::MenuItemClick {id, ..} = event{
                        if id == "quit"{
                            let tray_handle = handle.tray_handle_by_id("main").unwrap();
                            //タスクトレイを削除
                            tray_handle.destroy().unwrap();
                            //アプリを終了
                            handle.exit(0);
                        }
                        if id == "menu1"{
                                                        
                        }
                    }
                })
                .build(app)
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_noun, get_count_noun, get_top_noun])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
