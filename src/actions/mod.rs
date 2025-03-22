use std::collections::HashMap;
use std::process::exit;

use crate::ui::user::{display_next, display_raw};

pub mod applaunch;
pub mod commandlaunch;
pub mod util;
pub mod websearch;

pub fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app_launcher" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                applaunch::applaunch(exec);
                exit(0);
            }
            "web_launcher" => {
                let query = attrs.get("keyword").map_or("", |s| s.as_str());
                let engine = attrs.get("engine").map_or("", |s| s.as_str());
                let _  = websearch::websearch(engine, query);
                exit(0);
            }
            "command" => {
                let exec = attrs.get("exec").map_or("", |s| s.as_str());
                let keyword = attrs.get("keyword").map_or("", |s| s.as_str());
                let _ = commandlaunch::command_launch(exec, keyword);
                exit(0)
            }
            "copy" => {
                if let Some(result) = attrs.get("result") {
                    let _ = util::copy_to_clipboard(result.as_str());
                }
            },
            "next" => {
                let next_content = attrs.get("next_content").map_or("No next_content provided...", |s| s);
                display_next(next_content);
            },
            "display_raw" => {
               if let Some(next_content) = attrs.get("next_content"){
                    display_raw(next_content, false);
                }
            }
            _ => {
                if let Some(out) = attrs.get("text_content"){
                    print!("{}", out);
                }
                exit(0)

            }
        }
    }
}
