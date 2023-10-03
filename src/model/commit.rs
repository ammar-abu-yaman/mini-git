use std::io;
use std::io::Write;
use chrono::offset::Local;
use chrono::DateTime;

use super::Serializer;

pub struct Commit {
    pub parent_hash: String,
    pub tree_hash: String,
    pub message: String,
}

impl Commit {
    pub fn serializer(&self) -> CommitSerializer<'_> {
        CommitSerializer(self)
    }
}

pub struct CommitSerializer<'a>(&'a Commit);

impl Serializer for CommitSerializer<'_> {
    fn serialize<W: io::Write>(&self, mut w: W) -> Result<(), io::Error> {
        let timestamp: DateTime<Local> = Local::now();
    
        let Commit {parent_hash, tree_hash, message,} = self.0;
        let mut body = vec![];
        write!(body, "tree {tree_hash}\n")?;
        write!(body, "parent {parent_hash}\n")?;
        write!(body, "author Mahatma Gandhi <mahatma.gandhi@gmail.com> {}\n", timestamp.format("%s %z"))?;
        write!(body, "committer Mahatma Gandhi <mahatma.gandhi@gmail.com> {}\n", timestamp.format("%s %z"))?;
        write!(body, "\n")?;
        write!(body, "{message}")?;
        
        write!(w, "commit {}\0", body.len())?;
        w.write(&body)?;
        Ok(())
    }
}
// df8b2dba16060f7e3ef5194eeccb4f7819452d52
/*
tree ba337d5cf7bd0ace4070a25b890d4ace942e1f7e
parent 64c91216bbcfd2ee920755816b093a6ce557b0e9
author Ammar Abu Yaman <ammar.abu.yaman@gmail.com> 1694264894 +0300
committer Ammar Abu Yaman <ammar.abu.yaman@gmail.com> 1694264894 +0300

Added support for ls-tree --name-only command
*/