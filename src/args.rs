use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    /// Number of times to greet
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}
