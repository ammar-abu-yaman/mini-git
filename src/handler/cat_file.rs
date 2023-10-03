use std::io::{stdout, Write};

use super::CommandHandler;
use crate::argparse::CatFileArgs;
use crate::handler::util::{find_object_from_hash, unzip_object};

impl CommandHandler {
    pub fn cat_file(&self, args: CatFileArgs) {
        if let None = args.pretty_print {
            return;
        }
        let hash = args.pretty_print.unwrap();
        let path = find_object_from_hash(&hash);
        let buf = unzip_object(&path).expect(&format!("Fatal Error: Can't read '{path:?}'"));
        let index = buf.iter().position(|el| el == &b'\0').unwrap();
        stdout().lock().write_all(&buf[index + 1..]).expect("Couldn't write to stdout");
    }
}
