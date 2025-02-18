use clap::Parser;

enum CliSubcommand {
    Add,
    Update,
    Remove
}

struct Cli {
    command: CliSubcommand,
    package_path: std::path::PathBuf,
}

fn main() {
    println!("Hello {}", "world");
}