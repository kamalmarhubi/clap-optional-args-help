extern crate clap;

use clap::{App, Arg};
use clap::AppSettings::TrailingVarArg;

fn main() {
    let _matches = App::new("clap-optional-args-help")
                      .setting(TrailingVarArg)
                      .arg(Arg::with_name("COMMAND").help("Command to run").index(1))
                      .arg(Arg::with_name("ARG")
                               .help("Arguments for COMMAND")
                               .multiple(true)
                               .requires("COMMAND")
                               .index(2))
                      .get_matches();
}
