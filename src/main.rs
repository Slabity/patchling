#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

extern crate fern;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

mod result;
use result::*;

fn run(matches: &ArgMatches) -> Result<()> {
    debug!("{:#?}", matches);
    Err("Test".into())
}

fn init_logger(file: &str, level: &str) -> Result<()> {
    use fern::FormatCallback as Callback;
    use log::LogRecord as Record;
    use std::fmt::Arguments as Args;

    let format = | callback: Callback, message: &Args, record: &Record | {
        callback.finish(format_args!("[{}] {}", record.level(), message))
    };

    let verbosity = match level {
        "OFF" => log::LogLevelFilter::Off,
        "ERROR" => log::LogLevelFilter::Error,
        "WARN" => log::LogLevelFilter::Warn,
        "INFO" => log::LogLevelFilter::Info,
        "DEBUG" => log::LogLevelFilter::Debug,
        "TRACE" => log::LogLevelFilter::Trace,
        &_ => {
            log::LogLevelFilter::Off
        }
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

fn print_error_backtrace(err: Error) {
    for e in err.iter() {
        error!("caused by: {}", &e);
    }

    if let Some(backtrace) = err.backtrace() {
        error!("[BACKTRACE]: {:?}", backtrace)
    }

    print_stderr_backtrace(err);
}

fn print_stderr_backtrace(err: Error) {
    for e in err.iter() {
        eprintln!("[ERROR] caused by: {}", &e);
    }

    if let Some(backtrace) = err.backtrace() {
        eprintln!("[BACKTRACE]: {:?}", backtrace);
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();

    let log_file = matches.value_of("log-file").unwrap_or("<stdout>");
    let log_level = matches.value_of("log-level").unwrap_or("INFO");

    if let Err(e) = init_logger(log_file, log_level) {
        print_stderr_backtrace(e);
    }

    if let Err(e) = run(&matches) {
        print_error_backtrace(e);
    }
}
