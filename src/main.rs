use clap::Parser;
use std::{
    fs::{self, metadata},
    path::PathBuf,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "The command to run on each directory")]
    command: String,

    #[clap(short = 'p', long, help = "Do not transform \\n into newline")]
    plain: bool,

    #[clap(short, long, help = "Recurse into inner directories")]
    recurse: bool,

    #[clap(
        short,
        long,
        default_value = "",
        help = "Only filter directories containing this directory"
    )]
    contains_dir: String,

    #[clap(short, long, default_value = "", help = "Filter following directories")]
    filter: String,
    /*
    // Unimplemented options
        #[clap(short, long, help = "Be more verbose")]
        verbose: bool,

        #[clap(
            short,
            long,
            help = "Depth of recursion. Negative values are counted from bottom"
        )]
        depth: Option<i8>,

        #[clap(short, long, default_value = "", help = "Ignore following directories")]
        ignore: String,
    */
}

fn main() {
    let args = Args::parse();

    let command = if args.plain {
        args.command.clone()
    } else {
        format_command(&args.command)
    };

    process_directory(".", &command, &args);
}

fn process_directory(dir: &str, command: &str, args: &Args) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path_name = path.unwrap().path();
        let metadata = metadata(&path_name).unwrap();
        if metadata.is_dir() {
            if filter_dir(&path_name, args) {
                println!("pushd \"{}\"", path_name.display());
                println!("  {}", command);
                println!("popd\n");
            }

            if args.recurse {
                process_directory(path_name.to_str().unwrap(), command, args);
            }
        }
    }
}

/// Format the command to be executed
fn format_command(raw_command: &String) -> String {
    raw_command.replace("\\n", "\n  ")
}

// Filter directory by name
fn filter_dir(path_name: &PathBuf, args: &Args) -> bool {
    if !args.contains_dir.is_empty() && !path_name.join(&args.contains_dir).exists() {
        return false;
    }
    if !args.filter.is_empty() && !path_name.to_str().unwrap().contains(&args.filter) {
        return false;
    }
    true
}
