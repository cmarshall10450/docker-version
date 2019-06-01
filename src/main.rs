#[macro_use]
extern crate clap;
extern crate semver;

mod error;
mod command;

use error::DockerVersionError;
use clap::App;

fn main() -> Result<(), DockerVersionError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml).get_matches();

    match matches.subcommand() {
        ("bump", Some(m)) => command::bump(m).unwrap(),
        (&_, _) => ()
    }

    Ok(())
}