use clap_derive::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CommandArg {
    #[arg(long)]
    pub package: String,
}
