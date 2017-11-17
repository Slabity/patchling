#[macro_use]
extern crate clap;
use clap::App;

extern crate fern;
extern crate chrono;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

mod result;
use result::*;

fn run() -> Result<()> {
    Ok(())
}

fn init_logger(file: &str, level: &str) -> Result<()> {

    let format = | callback: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::LogRecord | {
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

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let log_file = matches.value_of("log-file").unwrap_or("<stdout>");
    let log_level = matches.value_of("log-level").unwrap_or("INFO");

    if let Err(ref e) = init_logger(log_file, log_level) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let msg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(msg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(msg);
        }
    }

    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let msg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(msg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(msg);
        }
    }
}
