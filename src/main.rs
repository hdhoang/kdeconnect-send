extern crate "dbus-rs" as dbus;

use std::os::args;
use dbus::{Connection, BusType, Message, MessageItem};

static DEST: &'static str = "org.kde.kdeconnect";
static PATH: &'static str = "/modules/kdeconnect";

fn get_ids(c: &Connection) -> Result<Vec<String>, &'static str> {
    let mut m = Message::new_method_call(DEST
                                         , PATH
                                         , (DEST.to_string() + ".daemon").as_slice()
                                         , "devices").unwrap();
    m.append_items(&[MessageItem::Bool(true)]); // onlyReachable
    m = c.send_with_reply_and_block(m, 1000).unwrap();
    let items = m.get_items();
    match items[0] {
        MessageItem::Array(_, 0) => Err("No reachable devices"),
        MessageItem::Array(ref a, _) => Ok(a.clone().into_iter().map(|e| match e {
            MessageItem::Str(s) => s,
            _ => unreachable!(),
        }).collect()),
        _ => unreachable!(),
    }
}

fn share_url(c: &Connection, id: &String, url: &String) {
    let mut m = Message::new_method_call(DEST
                                         , (PATH.to_string() + "/devices/" + id.as_slice() + "/share").as_slice()
                                         , (DEST.to_string() + ".device.share").as_slice()
                                         , "shareUrl").unwrap();
    m.append_items(&[MessageItem::Str(url.clone())]);
    c.send_with_reply_and_block(m, 1000).unwrap();
}

fn main() {
    let mut urls = args();
    urls.remove(0);

    let ref c = Connection::get_private(BusType::Session).unwrap();
    let ids = get_ids(c).unwrap();
    for url in urls.iter() {
        for id in ids.iter() {
            share_url(c, id, url);
        }
    }
}
