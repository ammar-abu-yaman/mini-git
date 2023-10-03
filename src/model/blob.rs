use std::{fmt::Display, path::Path};

use crate::handler::util::read_file_to_end;

use super::Serializer;

pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn pretty_display(&self) -> Pretty<'_> {
        Pretty(self)
    }

    pub fn serializer(&self) -> BlobSerializer<'_> {
        BlobSerializer(self)
    }
}

pub struct Pretty<'a>(&'a Blob);

impl<'a> Display for Pretty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = String::from_utf8_lossy(&self.0.content);
        write!(f, "{}", content)
    }
}

impl TryFrom<&[u8]> for Blob {
    type Error = std::io::Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let index = buf
            .iter()
            .position(|el| el == &b'\0')
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        Ok(Blob {
            content: Vec::from(&buf[index + 1..]),
        })
    }
}

impl TryFrom<&Path> for Blob {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_dir() {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let content = read_file_to_end(path)?;
        Ok(Blob { content })
    }
}

pub struct BlobSerializer<'a>(&'a Blob);

impl Serializer for BlobSerializer<'_> {
    fn serialize<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let blob = self.0;
        write!(w, "blob {}\0", blob.content.len())?;
        w.write(&self.0.content)?;
        Ok(())
    }
}
