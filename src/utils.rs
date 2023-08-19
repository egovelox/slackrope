use std::time::Duration;
use weechat_relay_rs::messages::WString;

pub fn sleep(seconds: u64) {
    std::thread::sleep(Duration::from_secs(seconds))
}

pub fn to_utf8_lossy(ws: &WString) -> Option<std::borrow::Cow<'_, str>> {
    ws.bytes().as_ref().map(|k| String::from_utf8_lossy(k))
}

pub fn clean_string(ws: &WString) -> String {
    if let Some(s) = to_utf8_lossy(ws) {
        s.to_string()
    } else {
        "".to_string()
    }
}

pub fn match_string(ws: &WString, search_term: &str) -> bool {
    if let Some(s) = to_utf8_lossy(ws) {
        match s.as_ref() == search_term {
            true => return true,
            false => return false,
        }
    };
    false
}
