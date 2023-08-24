
mod errors;
mod cli;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            errors::exit_with_retcode(Ok(()))
        },
        Err(e) => {
            log::debug!("Error while parsing args:\n\t{}", e);
            errors::exit_with_retcode(Err(e))
        }
    };
}
