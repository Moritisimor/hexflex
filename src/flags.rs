use clap::Parser;

#[derive(Parser, Clone)]
pub struct Flags {
    #[arg(short, long)]
    pub output_file: Option<String>,

    #[arg(short, long)]
    pub input_file: String,
}
