pub mod blob;
pub mod tree;
pub mod commit;

use std::io::{self, Write};

pub use blob::Blob;
pub use tree::{Tree, TreeObjectEntry};

pub trait Serializer {
    fn serialize<W: Write>(&self, w: W) -> Result<(), io::Error>;
}