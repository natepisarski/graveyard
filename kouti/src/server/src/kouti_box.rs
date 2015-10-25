use std::process::{Child, Command};
use std::io::Read;
use std::default::Default;

use kouti_component::KoutiComponent;

// KoutiBox is a wrapper for a process, providing a buffer
// that the process's standard output is redirected to. The
// Relay should read and empty this buffer.
pub struct KoutiBox {
	pub process: (String, Vec<String>),
	pub buffer:  Vec<String>
}

impl KoutiBox {

	/// Create a new KoutiBox from a command.
	/// Default is no arguments
	pub fn new(cmd: &str) -> KoutiBox{
		KoutiBox {
			process: (String::from(cmd), vec![]),
			buffer: vec![],
		}
	}

	/// Get the name of the current process
	pub fn name(&self) -> String {
		match(self.process){
			(ref name, _) => name.clone(),
		}
	}

	/// Set the process arguments for the running command
	pub fn set_process_args(&mut self, args: &Vec<String>){
		match self.process.clone() {
			(name, current_args) => self.process = (name, args.clone()),
		}
	}

	/// Move output from the process to the KoutiBox buffer.
	/// If the process returns anything that is 
	pub fn process_buffer(&mut self) {
		let attempted_output = match self.process.clone() {
			(prg_name, args) => Command::new(prg_name).args(&args).output(),
		};

		match attempted_output {
			// Output returned properly? Convert it to a String or do nothing if it cant.
			Ok(op) => match String::from_utf8(op.stdout) {
				Ok(str_output) => self.buffer.push(str_output.clone()),
				Err(_) => (),
			},
			// Something went wrong with the output? Push a blank string.
			_ => self.buffer.push("".to_string()),
		}
	}
}

impl KoutiComponent for KoutiBox {
	fn pulse_react(&mut self) -> String {
		return self.buffer[self.buffer.len() - 1].clone()
	}

	fn history(&self) -> Vec<String> {
		self.buffer.clone()
	}
}

impl Sized for KoutiBox {
	
}