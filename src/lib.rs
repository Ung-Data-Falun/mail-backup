use std::{ffi::CStr, hash::{DefaultHasher, Hash, Hasher}, os::raw::c_char};

use chrono::Utc;
use mlpa::Plugin;

macro_rules! try_return {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_e) => return,
        }
    };
}

#[no_mangle]
pub extern "C" fn get_plugin() -> Plugin {
    use mlpa::Optional::Some;
    Plugin {
        on_start: Some(on_start),
        message_handler: Some(message_handler),
    }
}

extern "C" fn on_start() { 
    println!("Initialized mail-backup");
}

extern "C" fn message_handler(message: *const c_char) {
    let message = unsafe { CStr::from_ptr(message) };
    let message = String::from_utf8_lossy(message.to_bytes()).to_string();
    match std::fs::read_dir("archive") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                try_return!(std::fs::create_dir("archive"));
            }
            _ => return,
        },
    }
    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    let hash = hasher.finish();

    let unix_time = Utc::now().timestamp();

    let filename = format!("archive/{unix_time:x}-{hash:x}.txt");

    try_return!(std::fs::write(filename, message));
}
