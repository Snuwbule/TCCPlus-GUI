
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// who would run this on windows or linux lol
use std::process::{Command, ExitStatus};
use std::io::Result;
use std::fs;

#[tauri::command]
fn send(action: &str, service: &str, bundle_id: &str) -> String {
    if action.is_empty() || service.is_empty() || bundle_id.is_empty() {
        return format!("Arguments are empty! \n The app will break if you break it.");
    }
    println!("{}ing {} for {}", action, service, bundle_id);
    let result = run_the_thing("/Applications/tccplus", &[action, service, bundle_id]);
    match result {
        Ok(msg) => return format!("{}", msg),
        Err(error) => return format!("{}", error)
    };
}

#[tauri::command]
fn find_bundle_id(payload: &str) -> String {
    let dir = payload.to_string() + "/contents/Info.plist";
    let file = fs::read_to_string(dir).expect("file not found"); 
    return format!("{}", file);
}

pub fn run_the_thing(tcc: &str, args:&[&str]) -> Result<ExitStatus> {
    Command::new(tcc).args(args).spawn()?.wait()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send,find_bundle_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
