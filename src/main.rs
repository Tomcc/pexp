use anyhow::anyhow;
use anyhow::Result;
use argh::{self, FromArgs};
use notify::RecursiveMode;
use notify::Watcher;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

#[derive(FromArgs, PartialEq, Debug)]
/// Set an environment variable
#[argh(subcommand, name = "set")]
struct SetArgs {
    /// name of the variable
    #[argh(positional)]
    name: String,

    /// value of the variable
    #[argh(positional)]
    value: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Unset an environment variable
#[argh(subcommand, name = "unset")]
struct UnsetArgs {
    /// name of the variable
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Start the watch daemon against ~/.syncenvrc
#[argh(subcommand, name = "watch")]
struct WatchArgs {
    /// target PID that will receive the change signal
    #[argh(positional)]
    pid: nix::pty::SessionId,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Set(SetArgs),
    Unset(UnsetArgs),
    Watch(WatchArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Set persistent environment variables across sessions.
struct Args {
    #[argh(subcommand)]
    command: Subcommands,
}

const RC_FILE: &str = ".syncenvrc";

fn get_rc_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or(anyhow!("Could not find home directory"))?;
    let path = home.join(RC_FILE);

    Ok(path)
}

fn load_existing() -> Result<String> {
    let path = get_rc_path()?;

    // if no file is existing, create a new one
    if !path.exists() {
        let mut file = std::fs::File::create(&path)?;
        writeln!(file, "# Warning: this file is managed by syncenv #")?;
    }

    let contents = std::fs::read_to_string(path)?;

    Ok(contents)
}

fn load_lines_except(name: &str) -> Result<Vec<String>> {
    let db_str = load_existing()?;

    // split into lines, delete the target line if it exists
    let starts_with = format!("export {}=", name);
    let lines: Vec<String> = db_str
        .lines()
        .filter(|line| !line.starts_with(&starts_with))
        .map(str::to_string)
        .collect();

    Ok(lines)
}

fn dump_lines(lines: impl IntoIterator<Item = String>) -> Result<()> {
    let path = get_rc_path()?;
    let mut file = std::fs::File::create(&path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = argh::from_env::<Args>();

    match args.command {
        Subcommands::Set(args) => {
            // split into lines, delete the target line if it exists
            let mut lines = load_lines_except(&args.name)?;

            // now append the new version to the end and sort alphabetically
            let target_line = format!("export {}={}", args.name, args.value);
            lines.push(target_line);
            lines.sort();

            // and dump them back out
            dump_lines(lines)?;
        }
        Subcommands::Unset(args) => {
            // split into lines, delete the target line if it exists
            let lines = load_lines_except(&args.name)?;

            // and dump them back out
            dump_lines(lines)?;
        }
        Subcommands::Watch(args) => {
            use nix::sys::signal::*;

            let target_pid = nix::unistd::Pid::from_raw(args.pid);

            // watch the file and send a signal to the target process if anything changes
            let mut watcher = notify::recommended_watcher(move |res| match res {
                Ok(_) => {
                    // trigger the SIGUSR2 signal
                    kill(target_pid, SIGUSR2).expect("Failed to send SIGUSR2 signal");
                }
                Err(e) => {
                    println!("Syncenv error: Watching failed. Cause: {:?}", e);
                    exit(1)
                }
            })?;

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            let path = get_rc_path()?;
            watcher.watch(&path, RecursiveMode::NonRecursive)?;

            // wait forever
            loop {
                std::thread::park();
            }
        }
    }

    Ok(())
}
