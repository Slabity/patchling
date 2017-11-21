use std;
use log;
use conrod;

#[derive(Debug, Fail)]
enum 

error_chain! {
    foreign_links {
        Std(std::io::Error);
        Log(log::SetLoggerError);
        Display(conrod::glium::backend::glutin::DisplayCreationError);
    }
}
