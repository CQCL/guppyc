use clap::Parser;
use guppyc::cli::CliArgs;

fn main() {
    let args = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();

    match args.run() {
        Ok(_) => {}
        Err(e) => {
            log::error!("{:?}", e);
            std::process::exit(1);
        }
    }
}
