extern crate citadel;

mod kouti_box;
use kouti_box::KoutiBox;

mod kouti_feed;
use kouti_feed::KoutiFeed;

mod kouti_hook;
use kouti_hook::KoutiHook;

mod kouti_relay;
use kouti_relay::KoutiRelay;

mod kouti_component;

fn main() {
	let mut my_box = KoutiBox::new("ls");

	{
		let mut my_hook = KoutiHook::new(&mut my_box, String::from("FIRE!"));
		my_hook.react(String::from("FIRE! /home/"));
	}	

	my_box.set_process_args(&vec![String::from("-R")]);
	my_box.process_buffer();
	println!("Latest information in buffer: {}", my_box.buffer[1]);
}
