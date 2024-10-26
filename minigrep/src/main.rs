use std::env;
use std::error::Error;

use minigrep::run;
use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(c) => c,
        Err(error) => return Err(format!("problem parsing arguments: {error}").into()),
    };

    if let Err(error) = run(config) {
        return Err(error.into());
    }

    Ok(())
}
