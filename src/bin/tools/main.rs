mod readme;
mod scaffold;
mod util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "tools", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    Scaffold {
        #[clap(long)]
        url: String,
    },
    VerifyReadme,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Scaffold { url } => scaffold::scaffold_new_challenge(&url),
        Command::VerifyReadme => {
            println!("Verifying README has all corresponding challenge entries.");
            let res = readme::verify();
            if res.is_ok() {
                println!("No README issues detected!")
            }
            res
        }
    }
}
