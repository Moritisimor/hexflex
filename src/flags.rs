use clap::Parser;

#[derive(Parser, Clone)]
pub struct Flags {
    #[arg(short, long)]
    pub save: Option<String>,

    #[arg(short='f', long)]
    pub input_file: String
}
