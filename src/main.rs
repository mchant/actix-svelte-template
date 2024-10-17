mod args;
mod web;
use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    tokio::select! {
        Err(err) = web::run(args.host, args.port) => {
            println!("web exited. {}", err);
        }
    };
    Ok(())
}
