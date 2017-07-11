use std::io;
use std::io::Write;
use std::env;
use std::process;

use prelude::*;

use clap::{App, Arg, AppSettings, ArgMatches};

const ABOUT: &'static str = "
Eris calls to Zeus.";

fn execute(args: Vec<String>) -> Result<()> {
    let app = App::new("eris")
        .about(ABOUT)
        .max_term_width(100)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage);

    let matches = app.get_matches_from_safe(args)?;
    unreachable!();
}

fn run() -> Result<()> {
    execute(env::args().collect())
}

/// Helper that renders an error to stderr.
pub fn print_error(err: &Error) {
    use std::error::Error;

    if let &ErrorKind::Clap(ref clap_err) = err.kind() {
        clap_err.exit();
    }

    writeln!(&mut io::stderr(), "error: {}", err).ok();
    let mut cause = err.cause();
    while let Some(the_cause) = cause {
        writeln!(&mut io::stderr(), "  caused by: {}", the_cause).ok();
        cause = the_cause.cause();
    }

    if env::var("RUST_BACKTRACE") == Ok("1".into()) {
        writeln!(&mut io::stderr(), "").ok();
        writeln!(&mut io::stderr(), "{:?}", err.backtrace()).ok();
    }
}

pub fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(err) => {
            print_error(&err);
            process::exit(1);
        }
    }
}
