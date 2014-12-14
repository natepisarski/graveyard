/* libcap - Data structures and functions for use in main */

use std;
use std::io::{BufferedWriter, File, fs, FilePermission, IoResult, process};
use std::io::process::{Process};

pub struct CaptureContext {
    pub process: Process,
    pub commands: Vec<String>,
    pub filename: Path
}

impl CaptureContext {

    pub fn write_context(&self) -> IoResult<()> {
        let mut writer = BufferedWriter::new(File::create(&self.filename));

        for curline in self.commands.iter() {
            writer.write_str(curline.as_slice());
            writer.flush();
        }

        return fs::chmod(&self.filename, std::io::USER_EXEC);
    }

    pub fn start(&mut self) {

        let mut local_reader = std::io::stdin();
        
        loop {
            let shell_output = self.process.stdout.as_mut().unwrap().read_to_string().unwrap();           

            let input = local_reader.read_line().ok().expect("Failed to read line");

            if(input.clone() == String::from_str("quitcapture".as_slice())) {
                break;
            }
            
            self.commands.push(input.clone());

            self.process.stdin.as_mut().unwrap().write_str(input.clone().as_slice());
        }
    }
}
