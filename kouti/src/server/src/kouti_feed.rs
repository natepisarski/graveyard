use kouti_box::KoutiBox;
use kouti_component::KoutiComponent;

use citadel::access::strings::to_words;

/// KoutiFeeds are direction connections between
/// two programs. Output from one program (source)
/// will be taken and put into another (destination).
/// 
/// this is the Kouti equivelant of source | destination in
/// unix shell. 
///
/// By taking references to KoutiBoxes, you can have two feeds
/// feed each other. 
pub struct KoutiFeed<'a, 'b> {
	pub source: &'a mut KoutiBox,
	pub destination: &'b mut KoutiBox,
	pub all: bool,
}

impl<'a, 'b> KoutiFeed<'a, 'b> {

	/// Make a new KoutiFeed with the specified input and output boxes
	pub fn new(source_box: &'a  mut KoutiBox, destination_box: &'b mut KoutiBox, all: bool) -> KoutiFeed<'a, 'b> {
		KoutiFeed {source : source_box, destination: destination_box, all: all}
	}

	/// Feed all of the source process's buffer into
	/// the destination buffer's arguments, and then run it.
	/// This will place the output of the command into destination's buffer.
	pub fn feed_all(&mut self){
		self.destination.set_process_args(&self.source.buffer);
		self.destination.process_buffer();
	}

	/// Feed the last output from the source's buffer
	/// into the destination as words
	pub fn feed_last(&mut self){
		let last_output = self.source.buffer[self.source.buffer.len() - 1].clone();

		self.destination.set_process_args(&to_words(last_output));
		self.destination.process_buffer();
	}
}

impl<'a, 'b> KoutiComponent for KoutiFeed<'a, 'b> {
	/// Feed the last item, and then return the destination output. 
	fn pulse_react(&mut self) -> String {
		self.source.pulse_react();

		if(self.all) {
			self.feed_all();
		} else {
			self.feed_last();
		}

		self.destination.buffer[self.destination.buffer.len() - 1].clone()
	}

	fn history(&self) -> Vec<String> {
		self.destination.buffer.clone()
	}
}