use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter, Read};
use std::os::unix::prelude::MetadataExt;
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

use crate::model::Serializer;
use crate::model::Tree;
use crate::model::TreeObjectEntry;

const BUF_SIZE: usize = 256;

pub fn get_objects_db_path() -> PathBuf {
    Path::new(".git").join("objects")
}

pub fn binary_to_hex_string(slice: &[u8]) -> String {
    hex::encode(slice)
}

pub fn hex_string_to_binary(slice: &str) -> Vec<u8> {
    hex::decode(slice).expect("failed to decode")
}

pub fn read_file_to_end<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn hash_object(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hash: &[u8] = hash.as_slice();
    binary_to_hex_string(hash)
}

pub fn hash_serializer(s: impl Serializer) -> Result<String, std::io::Error> {
    let mut buf = vec![];
    s.serialize(&mut buf)?;
    Ok(hash_object(&buf))
}

pub fn write_serializer_to_objects_db(s: impl Serializer) -> Result<String, std::io::Error> {
    let mut content = vec![];
    s.serialize(&mut content)?;
    let hash = hash_object(&content);
    content.clear();

    zip_serializer(s, &mut content)?;

    let parent_path = get_objects_db_path().join(&hash[..2]);

    if ! parent_path.exists() {
        fs::create_dir(&parent_path)?;
    }
    let file_path = parent_path.join(&hash[2..]);
    let file = fs::File::create(file_path.as_path()).expect(&format!(
        "fatal: could not create object with path '{}'",
        file_path.as_path().to_str().unwrap()
    ));
    let mut writer = BufWriter::new(file); 
    writer.write_all(&content[..])?;
    Ok(hash)
}

pub fn get_filemode(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
    if path.is_dir() {
        return Ok(String::from("040000"));
    } else if path.is_symlink() {
        return Ok(String::from("120000"));
    } else if path.is_file() {
        return if is_executable(path) {
            Ok(String::from("100755"))
        } else {
            Ok(String::from("100644"))
        };
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }
}

pub fn is_executable(path: impl AsRef<Path>) -> bool {
    match fs::metadata(path) {
        Ok(metadata) if metadata.mode() & 0o111 != 0 => true,
        _ => false,
    }
}

pub fn find_object_from_hash(hash: &str) -> PathBuf {
    if hash.len() < 4 {
        panic!("Error: Not a valid object name");
    }
    let object_dir = Path::new(".git").join("objects").join(&hash[..2]);

    if !object_dir.exists() || !object_dir.is_dir() {
        panic!("Error: Not a valid object name");
    }

    let matchings: Vec<PathBuf> = object_dir
        .as_path()
        .read_dir()
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap().path())
        .filter(|p| {
            p.to_str()
                .unwrap()
                .starts_with(object_dir.join(&hash[2..]).to_str().unwrap())
        })
        .collect();
    if matchings.len() != 1 {
        panic!("Error: Not a valid object name");
    }
    matchings[0].to_owned()
}

pub fn unzip_object<P: AsRef<Path>>(object_path: P) -> Result<Vec<u8>, std::io::Error> {
    let file = fs::File::open(object_path)?;
    let mut zlib_decoder = ZlibDecoder::new(BufReader::new(file));
    let mut buf = vec![];
    zlib_decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn parse_tree_object<R: BufRead + Debug>(mut reader: R) -> Result<Tree, io::Error> {
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

pub fn zip_object<R: Read, W: Write>(mut content: R, output: W) -> Result<(), io::Error> {
    let mut encoder = ZlibEncoder::new(output, flate2::Compression::default());
    let mut buf = [0u8; BUF_SIZE];
    while let Ok(n) = content.read(&mut buf) {
        if n == 0 {
            break;
        }
        encoder.write(&buf[..n])?;
    }
    Ok(())
}

pub fn zip_serializer<S: Serializer, W: Write>(serializer: S, output: W) -> Result<(), io::Error> {
    let mut encoder = ZlibEncoder::new(output, flate2::Compression::default());
    serializer.serialize(&mut encoder)?; 
    Ok(())
}

pub fn produce_blob(buf: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut content = Vec::with_capacity(20 + buf.len());
    write!(content, "blob {}\0", buf.len())?;
    content.extend(&buf[..]);
    Ok(content)
}
