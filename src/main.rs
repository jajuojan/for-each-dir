use clap::Parser;
use std::fs;
use std::fs::metadata;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "The command to run on each directory")]
    command: String,

    #[clap(short, long, help = "Be more verbose")]
    verbose: bool,

    #[clap(short = 'p', long, help = "Do not transform \\n into newline")]
    plain: bool,

    #[clap(short, long, help = "Recurse into inner directories")]
    recurse: bool,

    #[clap(
        short,
        long,
        help = "Depth of recursion. Negative values are counted from bottom"
    )]
    depth: Option<u8>,

    #[clap(short, long, default_value = "", help = "Filter following directories")]
    filter: String,

    #[clap(short, long, default_value = "", help = "Ignore following directories")]
    ignore: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(".").unwrap();
    let command = format_command(&args.command);

    for path in paths {
        let path_name = path.unwrap().path();
        let metadata = metadata(&path_name).unwrap();
        if metadata.is_dir() {
            println!("pushd \"{}\"", path_name.display());
            println!("  {}", command);
            println!("popd\n");
        }
    }
}

/// Format the command to be executed
fn format_command(raw_command: &String) -> String {
    raw_command.replace("\\n", "\n  ")
}
