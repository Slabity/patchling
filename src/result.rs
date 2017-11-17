use std;
use log;

error_chain! {
    foreign_links {
        Std(std::io::Error);
        Log(log::SetLoggerError);
    }
}
