use super::CommandHandler;

use std::fs;
use std::path::Path;

impl CommandHandler {
    pub fn init(&self) {
        let root = Path::new(".git");
        fs::create_dir(root).unwrap();
        fs::create_dir(&self.object_path).unwrap();
        fs::create_dir(root.join("refs")).unwrap();
        fs::write(root.join("HEAD"), "ref: refs/heads/master\n").unwrap();
        println!("Initialized git directory")
    }
}
