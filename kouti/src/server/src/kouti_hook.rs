use kouti_box::KoutiBox;
use kouti_component::KoutiComponent;

use citadel::access::strings::to_words;

/// KoutiHooks are bindings of keywords and a destination box. The relay
/// will use KoutiHooks to move any information where the keyword is the first 
/// item into the destination box.
pub struct KoutiHook<'a> {
	pub destination: &'a mut KoutiBox,
	pub hook_keyword: String
}

impl<'a> KoutiHook<'a> {

	/// Create a new KoutiHook with the KoutiBox and some input
	pub fn new(bound_box: Box<KoutiBox>, hook: String) -> KoutiHook<'a> {
		KoutiHook::<'a> {destination: bound_box.deref_mut(), hook_keyword: hook}
	}

	/// Test to see if the line (string) of output
	/// contains this hook_keywrod
	pub fn does_match_line(&self, line: String) -> bool {
		self.hook_keyword == to_words(line)[0]
	}

	/// Unconditionally feed a list of arguments (beginning with the hook_keyword) to the 
	/// destination box
	pub fn feed_arguments(&mut self, line: String){
		let arguments: Vec<String> = to_words(line)[1..].iter().map(|c|{c.clone()}).collect();
		self.destination.set_process_args(&arguments);
	}

	/// React to input. Test to see if it matches this hook, and if it does,
	/// set the process arguments of the destination KoutiBox and process the arguments.
	pub fn react(&mut self, line: String){
		if(self.does_match_line(line.clone())){ //TODO: unnecessary clones
			self.feed_arguments(line.clone());
		}

		self.destination.process_buffer();
	}
}

impl<'a> KoutiComponent for KoutiHook<'a> {
	/// Simply returns the buffer for the 
	/// captured KoutiBox 
	fn pulse_react(&mut self) -> String {
		self.destination.buffer[self.destination.buffer.len() - 1].clone()
	}

	fn history(&self) -> Vec<String> {
		self.destination.buffer.clone()
	}
}