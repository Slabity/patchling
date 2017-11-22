#[macro_use] extern crate clap;
extern crate fern;
#[macro_use] extern crate log;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate conrod;

use self::clap::ArgMatches;
use self::failure::Error;

mod gui;

fn run(matches: &ArgMatches) -> Result<(), Error> {
    use std::thread;

    let gui = thread::spawn(|| {
        gui::run().unwrap()
    });

    gui.join();

    Ok(())
}

fn init_logger(file: &str, level: &str) -> Result<(), Error> {
    use self::fern::FormatCallback as Callback;
    use self::log::LogRecord as Record;
    use std::fmt::Arguments as Args;

    let format = | callback: Callback, message: &Args, record: &Record | {
        callback.finish(format_args!("[{}] {}", record.level(), message))
    };

    let verbosity = match level.to_uppercase().as_ref() {
        "OFF"   => log::LogLevelFilter::Off,
        "ERROR" => log::LogLevelFilter::Error,
        "WARN"  => log::LogLevelFilter::Warn,
        "INFO"  => log::LogLevelFilter::Info,
        "DEBUG" => log::LogLevelFilter::Debug,
        "TRACE" => log::LogLevelFilter::Trace,
        &_      => log::LogLevelFilter::Off
    };

    let dispatch = match file {
        "<stdout>" => {
            fern::Dispatch::new()
                .format(format)
                .level(verbosity)
                .chain(std::io::stdout())
        },
        name => {
            fern::Dispatch::new()
                .format(format)
                .level(verbosity)
                .chain(std::fs::File::create(name)?)
        }
    };

    dispatch.apply()?;

    info!("Initialized log");

    Ok(())
}

fn log_error(err: Error) {
    error!("{}", err);
    error!("[BACKTRACE]: {}", err.backtrace());
    eprint_error(err);
}

fn eprint_error(err: Error) {
    eprintln!("{}", err);
    eprintln!("[BACKTRACE]: {}", err.backtrace());
}

fn main() {
    use self::clap::App;

    let yaml = load_yaml!("cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();

    let log_file = matches.value_of("log-file").unwrap_or("<stdout>");
    let log_level = matches.value_of("log-level").unwrap_or("INFO");

    if let Err(e) = init_logger(log_file, log_level) {
        eprint_error(e);
    }

    if let Err(e) = run(&matches) {
        log_error(e);
    }
}
