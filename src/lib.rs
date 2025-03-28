pub mod winapi;
pub mod wrapper;
pub use wrapper::*;

#[cfg(test)]
mod test {
	use windows_error_handling::*;
	use super::wrapper::*;
	use crate::wrapper::buttons::*;
	#[test]
	fn test () {
		let mut gamepad = XInputGamepad::new(0);
		while gamepad.update() {
			eprintln!("{:?}", gamepad.buttons);
		}
	}

	#[test]
	fn gamepad_button_debug_test () {
		eprintln!("{:?}", GamePadButton::from_u16(!0));
	}
}
