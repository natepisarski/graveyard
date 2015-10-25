/// Defines the characteristics of components of the Kouti project.
/// simply, each kouti component must implement a function called
/// pulse_react that returns a list of strings, which is the buffer
/// after acting. 
pub trait KoutiComponent {
	fn pulse_react(&mut self) -> String;
	fn history(&self) -> Vec<String>;
}