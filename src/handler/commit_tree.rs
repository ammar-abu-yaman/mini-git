use super::{CommandHandler, util::write_serializer_to_objects_db};
use crate::{argparse::CommitTreeArgs, model::commit::Commit};

impl CommandHandler {
    pub fn commit_tree(&self, args: CommitTreeArgs) {
        let CommitTreeArgs{tree_ish, parent, message} = args;
        let commit = Commit {parent_hash: parent, tree_hash: tree_ish, message };
        let commit_hash = write_serializer_to_objects_db(commit.serializer()).unwrap();
        print!("{commit_hash}")
    }
}
