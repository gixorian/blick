use bytesize::ByteSize;
use colored::Colorize;
use std::{env, error::Error, fs, os::unix::fs::MetadataExt, path::Path};

use clap::Parser;

/// List files and directories in the current directory
#[derive(Parser)]
pub struct Cli {
    /// Display version information and exit
    #[arg(short = 'v', long = "version")]
    pub version: bool,

    /// List files only
    #[arg(short = 'f', long = "files-only")]
    pub files: bool,

    /// List directories only
    #[arg(short = 'd', long = "dirs-only")]
    pub directories: bool,

    /// Display absolute paths
    #[arg(short = 'a', long = "absolute-paths")]
    pub absolute_paths: bool,
}

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    if args.version {
        print!(env!("CARGO_PKG_NAME"));
        println!("-v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let cur_dir = env::current_dir().map_err(|e| Box::new(e))?;

    println!("\n{}", cur_dir.display().to_string().green().bold());
    list_all()?;

    Ok(())
}

fn list_all() -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir("./").map_err(|e| Box::new(e))?;

    for entry in entries {
        let path = entry.unwrap().path();
        let str = path.display().to_string();
        let trimmed = str.chars().skip(2).collect::<String>();

        let metadata = path.metadata().expect("Failed to read metadata");
        let size = ByteSize::b(metadata.size())
            .display()
            .iec_short()
            .to_string();

        let perms = get_rwx_string(&path).unwrap();
        let (user, group) = get_owner_and_group(&path)?;

        if path.is_dir() {
            println!(
                "{:>7}\t=> {}\t <= {} | u: {:<6} | g: {:<6}",
                size,
                str.purple().bold(),
                perms,
                user,
                group
            );
        } else if trimmed.starts_with('.') {
            println!(
                "{:>7}\t-> {}\t <- {} | u: {:<6} | g: {:<6}",
                size,
                str.yellow(),
                perms,
                user,
                group
            );
        } else {
            println!(
                "{:>7}\t-> {}\t <- {} | u: {:<6} | g: {:<6}",
                size,
                str.white(),
                perms,
                user,
                group
            );
        }
    }

    Ok(())
}

fn get_rwx_string(path: &Path) -> std::io::Result<String> {
    let metadata = fs::metadata(path)?;
    let mode = metadata.mode();

    let user = [
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
    ];

    let group = [
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
    ];

    let others = [
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    ];

    Ok(format!(
        "{}{}{}",
        user.iter().collect::<String>(),
        group.iter().collect::<String>(),
        others.iter().collect::<String>()
    ))
}

fn get_owner_and_group(path: &Path) -> std::io::Result<(String, String)> {
    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    let gid = metadata.gid();

    let user = users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());

    let group = users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string());

    Ok((user, group))
}
