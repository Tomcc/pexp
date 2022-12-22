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
/// Start the watch daemon against ~/.syncenvrc
#[argh(subcommand, name = "watch")]
struct WatchArgs {
    // empty
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Set(SetArgs),
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

fn main() -> Result<()> {
    let args = argh::from_env::<Args>();

    match args.command {
        Subcommands::Set(args) => {
            let db_str = load_existing()?;

            // split into lines, filtering comments
            let mut lines: Vec<String> = db_str.lines().map(str::to_string).collect();

            // Find the line that matches args.name, given the "export VAR=..." format, and edit it
            let starts_with = format!("export {}=", args.name);
            let target_line = format!("export {}=\"{}\"", args.name, args.value);

            let mut found = false;
            for line in &mut lines.iter_mut() {
                if line.starts_with(&starts_with) {
                    *line = target_line.clone();
                    found = true;
                    break;
                }
            }

            // if not found, append and sort alphabetically
            if !found {
                lines.push(target_line);
                lines.sort();
            }

            // dump the lines out, overwriting the file
            let path = get_rc_path()?;
            let mut file = std::fs::File::create(&path)?;
            for line in lines {
                writeln!(file, "{}", line)?;
            }
        }
        Subcommands::Watch(_watch_args) => {
            // watch the file and immediately exit if anything happens.
            // The shell script will source the file again.
            let mut watcher = notify::recommended_watcher(|res| match res {
                Ok(_) => exit(0),
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
