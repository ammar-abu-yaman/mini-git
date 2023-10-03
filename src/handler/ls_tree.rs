use crate::{argparse::LsTreeArgs, model::Tree};

use super::{
    util::{find_object_from_hash, unzip_object},
    CommandHandler,
};

impl CommandHandler {
    pub fn ls_tree(&self, args: LsTreeArgs) {
        if !args.name_only {
            panic!("should include the option '--name-only'")
        }

        let tree_path = find_object_from_hash(&args.tree_ish);
        let content = unzip_object(&tree_path).expect(&format!(
            "Fatal Error: Unable to read object '{tree_path:?}'"
        ));
        let tree = Tree::try_from(&content[..]).expect("Failed to parse tree object");
        print!("{}", tree.name_only_display());
    }
}
