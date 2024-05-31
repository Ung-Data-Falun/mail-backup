use std::hash::{DefaultHasher, Hash, Hasher};

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
pub fn get_plugin() -> Plugin {
    Plugin {
        message_handler: Some(Box::into_raw(Box::new(message_handler))),
    }
}

fn message_handler(message: String) {
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
