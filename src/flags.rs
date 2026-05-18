use clap::Parser;

#[derive(Parser, Clone)]
pub struct Flags {
    pub input_file: String,

    #[arg(short, long)]
    pub output_file: Option<String>,
}
