extern crate "dbus-rs" as dbus;

use std::os::args;
use dbus::{Connection, BusType, Message, MessageItem};

static DEST: &'static str = "org.kde.kdeconnect";
static PATH: &'static str = "/modules/kdeconnect";

fn get_ids(c: &Connection) -> Result<Vec<String>, &'static str> {
    let mut m = Message::new_method_call(DEST
                                         , PATH
                                         , &*[DEST, "daemon"].connect(".")
                                         , "devices").unwrap();
    m.append_items(&[MessageItem::Bool(true)]); // onlyReachable
    m = c.send_with_reply_and_block(m, 1000).unwrap();
    let items = m.get_items();
    match items[0] {
        MessageItem::Array(_, 0) => Err("No reachable devices."),
        MessageItem::Array(ref a, _) => Ok(a.iter().map(|&ref e| match *e {
            MessageItem::Str(ref s) => s.clone(),
            _ => unreachable!(),
        }).collect()),
        _ => unreachable!(),
    }
}

fn share_url(c: &Connection, id: &String, url: &String) {
    let mut m = Message::new_method_call(DEST
                                         , &*[PATH, "devices", &id[], "share"].connect("/")
                                         , &*[DEST, "device.share"].connect(".")
                                         , "shareUrl").unwrap();
    m.append_items(&[MessageItem::Str(url.clone())]);
    c.send_with_reply_and_block(m, 1000).unwrap();
}

fn main() {
    let urls = &args()[1..];
    if urls.is_empty() {
        return println!("Usage: {} url [url ...]", &args()[0])
    }

    let ref c = Connection::get_private(BusType::Session).unwrap();
    match get_ids(c) {
        Err(e) => return println!("{}", e),
        Ok(ids) =>
            for url in urls.iter() {
                for id in (*ids).iter() {
                    share_url(c, id, url);
                }
            }
    }
}
