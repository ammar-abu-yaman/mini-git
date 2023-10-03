use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init,

    #[command(name = "cat-file")]
    CatFile(CatFileArgs),

    #[command(name = "hash-object")]
    HashObject(HashObjectArgs),

    #[command(name = "ls-tree")]
    LsTree(LsTreeArgs),

    #[command(name = "write-tree")]
    WriteTree(WriteTreeArgs),

    #[command(name = "commit-tree")]
    CommitTree(CommitTreeArgs),
}

#[derive(Debug, Args)]
pub struct CatFileArgs {
    #[arg(short)]
    pub pretty_print: Option<String>,
}

#[derive(Debug, Args)]
pub struct HashObjectArgs {
    #[arg(short)]
    pub write: bool,
    #[arg()]
    pub file: String,
}

#[derive(Debug, Args)]
pub struct LsTreeArgs {
    #[arg(long)]
    pub name_only: bool,
    #[arg()]
    pub tree_ish: String,
}

#[derive(Debug, Args)]
pub struct WriteTreeArgs;

#[derive(Debug, Args)]
pub struct CommitTreeArgs {
    #[arg()]
    pub tree_ish: String,
    #[arg(short)]
    pub parent: String,
    #[arg(short)]
    pub message: String,
}