use kouti_box::KoutiBox;
use kouti_hook::KoutiHook;
use kouti_feed::KoutiFeed;

use kouti_component::KoutiComponent;

use citadel::access::strings::to_words;

/// KoutiRelay is the collection and interaction of all active Boxes and Hooks.
/// it can create new Boxes and Hooks, as well as delete active ones
pub struct KoutiRelay<'a, 'b> {
	pub active_feeds: Vec<KoutiFeed<'a, 'b>>,
	pub active_hooks: Vec<KoutiHook<'a>>,
}

impl<'a, 'b> KoutiRelay<'a, 'b> {
	/// Add a hook to the list of active hooks
	pub fn create_hook(&'a mut self, hook: KoutiHook<'a>) {
		self.active_hooks.push(hook);
	}

	/// Add a feed to the list of active feeds
	pub fn create_feed(&'a mut self, feed: KoutiFeed<'a, 'b>) {
		self.active_feeds.push(feed);
	}

	/// Builds a hook from a String.
	/// The syntax of the string should be "keyword command"
	pub fn build_hook(arg: String) -> KoutiHook<'a> {
		let words = to_words(arg);
		KoutiHook::new(Box::new(KoutiBox::new(words[0])), words[1].clone())
	}

	pub fn build_feed() {
		// Stub
	}
	
	/// Makes all of the active boxes "pulse", which means they will 
	/// process their buffers and return them for processing by the other boxes. 
	pub fn pulse(&mut self) {
		let mut all_output: Vec<String> = vec![];

		// Collect all the output from the feeds (also feeds)
		for ref mut feed in self.active_feeds.as_mut() {
			all_output.push(feed.pulse_react());
		}
		// Collect all the info from the current hooks
		for ref mut hook in self.active_hooks.as_mut() {
			all_output.push(hook.pulse_react());
		}

		// See if any hooks can do something with the output 
		for ref output_line in all_output.clone() {
			for ref mut hook in self.active_hooks.as_mut() {
				hook.react(output_line.clone());
			}
		}
	}
}