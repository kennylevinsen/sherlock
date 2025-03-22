use std::fs::File;
use std::os::linux::fs::MetadataExt;
use std::io::{self, Read};

use super::Loader;

impl Loader {
    pub fn load_pipe_args() -> String {
        if let Ok(metadata) = File::open("/dev/stdin").and_then(|f| f.metadata()){
            // 0o020000 - Character device (e.g. TTY)
            // 0o170000 - octal mask to extract all file types
            if metadata.st_mode() & 0o170000 == 0o020000 {
                return String::new();
            }
        }
        let stdin = io::stdin();
        let mut buf = String::new();
        let _ = stdin.lock().read_to_string(&mut buf);
        return buf
    }
}
