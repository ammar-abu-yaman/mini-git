use std::path::Path;

use super::CommandHandler;
use crate::argparse::HashObjectArgs;
use crate::handler::util::write_serializer_to_objects_db;
use crate::model::Blob;

impl CommandHandler {
    pub fn hash_object(&self, args: HashObjectArgs) {
        if !args.write {
            panic!("-w flag should be included");
        }

        let path = Path::new(&args.file);
        let blob = Blob::try_from(path).expect("Failed to parse blob object");
        let hash =
            write_serializer_to_objects_db(blob.serializer()).expect("Failed to write blob to db");
        print!("{hash}");
    }
}
