pub mod winapi;
pub mod wrapper;
pub use wrapper::*;

#[cfg(test)]
mod test {
	use windows_error_handling::*;
	use super::wrapper::*;
	#[test]
	fn test () {
		let mut gamepad = XInputGamepad::new(0);
		while gamepad.update() {
			eprintln!("{:?}", gamepad.buttons);
		}
	}
}
