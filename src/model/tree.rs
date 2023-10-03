use std::{
    fmt::Display,
    io::{self, BufRead, Read},
    path::Path,
};

use bytes::Buf;

use crate::handler::util::{
    binary_to_hex_string, get_filemode, hash_serializer, hex_string_to_binary,
};

use super::{Blob, Serializer};

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeObjectEntry>,
}

impl Tree {
    pub fn name_only_display(&self) -> NameOnly<'_> {
        NameOnly(self)
    }

    pub fn serializer(&self) -> TreeSerializer<'_> {
        TreeSerializer(self)
    }
}

impl TryFrom<&[u8]> for Tree {
    type Error = std::io::Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = buf.reader();
        let mut buf = vec![];
        reader.read_until(b'\0', &mut buf)?;
        let mut entries = vec![];
        buf.clear();
        while let Ok(n) = reader.read_until(b' ', &mut buf) {
            if n == 0 {
                break;
            }
            let mode = std::str::from_utf8(buf.split_last().unwrap().1)
                .unwrap()
                .to_string();
            buf.clear();
            reader.read_until(b'\0', &mut buf).unwrap();
            let filename = std::str::from_utf8(buf.split_last().unwrap().1)
                .unwrap()
                .to_string();
            buf.resize(20, b'\0');
            reader.read_exact(&mut buf).unwrap();
            let hash = binary_to_hex_string(&buf);
            buf.clear();
            entries.push(TreeObjectEntry {
                mode,
                filename,
                hash,
            });
        }
        Ok(Tree { entries })
    }
}

impl TryFrom<&Path> for Tree {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::try_from_path_helper(path, 0)
    }
}

impl Tree {
    fn try_from_path_helper(path: &Path, level: u32) -> Result<Self, std::io::Error> {
        if !path.exists() || !path.is_dir() {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let mut entries = vec![];
        for entry in std::fs::read_dir(path)? {
            let path = {
                let p = entry?.path();
                match p.strip_prefix("./") {
                    Ok(path) => path.to_owned(),
                    Err(_) => p,
                } 
            };
        
            if path.starts_with(".git") && level == 0 {
                continue;
            }
            let filename = path.to_string_lossy().to_string();
            let mode = get_filemode(&path)?;
            let hash = if path.is_dir() {
                let tree = Self::try_from_path_helper(path.as_path(), level + 1)?;
                hash_serializer(tree.serializer())?
            } else {
                let blob = Blob::try_from(path.as_path())?;
                hash_serializer(blob.serializer())?
            };
            entries.push(TreeObjectEntry {
                mode,
                filename,
                hash,
            })
        }
        Ok(Tree { entries })
    }
}

#[derive(Debug)]
pub struct TreeObjectEntry {
    pub mode: String,
    pub filename: String,
    pub hash: String,
}

pub struct NameOnly<'a>(&'a Tree);

impl<'a> Display for NameOnly<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tree_obj = self.0;
        let mut sorted_filenames: Vec<_> = tree_obj
            .entries
            .iter()
            .map(|entry| &entry.filename[..])
            .collect();
        sorted_filenames.sort_unstable();
        for filename in sorted_filenames {
            writeln!(f, "{filename}")?;
        }
        Ok(())
    }
}

pub struct TreeSerializer<'a>(&'a Tree);

impl Serializer for TreeSerializer<'_> {
    fn serialize<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let tree = self.0;
        let mut body = vec![];
        let mut sorted_entries: Vec<_> = tree.entries.iter().collect();
        sorted_entries.sort_by_key(|entry| &entry.filename[..]);

        
        for TreeObjectEntry{filename, mode, hash} in sorted_entries {
            body.extend_from_slice(mode.as_bytes());
            body.push(b' ');
            body.extend_from_slice(filename.as_bytes());
            body.push(0);
            body.append(&mut hex_string_to_binary(hash));
        }
        w.write("tree ".as_bytes()).unwrap();
        w.write(body.len().to_string().as_bytes()).unwrap();
        w.write(&[0]).unwrap();
        w.write(&body).unwrap();

        Ok(())

    }
}

