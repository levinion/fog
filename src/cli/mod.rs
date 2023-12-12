#[derive(clap::Parser)]
#[command(name = "fog")]
#[command(author = "levinion <levinnion@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "A simple language", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Run {
        file: Option<String>,
        #[arg(short, long)]
        debug: bool,
    },
    New {
        name: String,
    },
}
