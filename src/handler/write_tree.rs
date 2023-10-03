use std::path::Path;

use crate::{
    argparse::WriteTreeArgs,
    handler::util::hash_serializer,
    model::{Blob, Tree, Serializer},
};

use super::{util::write_serializer_to_objects_db, CommandHandler};

impl CommandHandler {
    pub fn write_tree(&self, _args: WriteTreeArgs) {
        let path = Path::new(".");
        let tree = Tree::try_from(path).expect("Failed to create tree object from local file");
        print!(
            "{}",
            Self::write_tree_helper(&tree)
        );
    }

    fn write_tree_helper(tree: &Tree) -> String {
        for entry in &tree.entries {
            let path = Path::new(&entry.filename);
            if let Ok(blob) = Blob::try_from(Path::new(&entry.filename)) {
                write_serializer_to_objects_db(blob.serializer())
                    .expect("Failed to write blob to db");
            } else if let Ok(tree) = Tree::try_from(path) {
                Self::write_tree_helper(&tree);
            }
        }
        write_serializer_to_objects_db(tree.serializer()).expect("Failed to write tree to db")
    }
}


/*
0923dfe9dccc2c67420122531f3d70761486cdcb
100755 blob b97aca77750351462b1baf6e6129e3d2ae79e9d0    git-starter-rust
100644 blob 3b18e512dba79e4c8300dd08aeb37f8e728b8dad    txt.xml
*/

/*
b3a5269f37bffd6d9aee48184fbd25b54f6a24fc
100755 blob b97aca77750351462b1baf6e6129e3d2ae79e9d0    git-starter-rust
100644 blob 3b18e512dba79e4c8300dd08aeb37f8e728b8dad    txt.xml
*/