use std::path::PathBuf;

use crate::argparse::{Cli, Commands};

pub mod util;

pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;
pub mod commit_tree;

pub struct CommandHandler {
    object_path: PathBuf,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            object_path: PathBuf::new().join(".git").join("objects"),
        }
    }
}

impl CommandHandler {
    pub fn handle(&self, cmd: Cli) {
        match cmd.command {
            Commands::Init => self.init(),
            Commands::CatFile(args) => self.cat_file(args),
            Commands::HashObject(args) => self.hash_object(args),
            Commands::LsTree(args) => self.ls_tree(args),
            Commands::WriteTree(args) => self.write_tree(args),
            Commands::CommitTree(args) => self.commit_tree(args),
        }
    }
}
