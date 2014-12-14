use std::io::process::{Command};
use libcap::{CaptureContext};

mod libcap;



fn main() {
    let shell_name = match std::os::getenv("SHELL"){
        Some(shellname) => shellname,
        None => String::from_str("/bin/sh".as_slice()),
    };

    let mut cap_ctx = CaptureContext {process: (Command::new(shell_name).spawn().unwrap()), commands:  vec![], filename: Path::new(&std::os::args()[1])};
    
    cap_ctx.start();    
    cap_ctx.write_context();
}
