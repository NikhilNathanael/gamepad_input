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
		while let Some(gamepad) = XInputGamepad::get_state(GamepadID::Id0) {
			eprintln!("{:?}", gamepad.buttons);
		}
	}

	#[test]
	fn gamepad_button_debug_test () {
		eprintln!("{:?}", GamePadButton::from_u16(!0));
	}
}
