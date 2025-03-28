#![allow(dead_code)]
mod xinput {
	#![allow(non_camel_case_types, non_snake_case)]
	use super::basic_types::*;
	#[repr(C)]
	#[derive(Debug, Clone, Copy)]
	pub struct XINPUT_GAMEPAD {
		pub wButtons: WORD,
		pub bLeftTrigger: BYTE,
		pub bRightTrigger: BYTE,
		pub sThumbLX: SHORT,
		pub sThumbLY: SHORT,
		pub sThumbRX: SHORT,
		pub sThumbRY: SHORT,
	}
	pub type PXINPUT_GAMEPAD = *mut XINPUT_GAMEPAD;

	#[repr(C)]
	#[derive(Debug, Clone, Copy)]
	pub struct XINPUT_STATE {
		pub dwPackerNumber: DWORD,
		pub Gamepad: XINPUT_GAMEPAD,
	}
	pub type PXINPUT_STATE = *mut XINPUT_STATE;

	#[repr(C)]
	#[derive(Debug, Clone, Copy, Default)]
	pub struct XINPUT_VIBRATION {
		pub wLeftMotorSpeed: WORD,
		pub wRightMotorSpeed: WORD,
	}
	pub type PXINPUT_VIBRATION = *mut XINPUT_VIBRATION;

	#[link(name = "Xinput", kind = "static")]
	extern "C" {
		// get input state of particular controller 
		// return value on success is ERROR_SUCCESS
		// return value if controller is not connected is ERROR_DEVICE_NOT_CONNECTED
		// function does not use SetLastError to set error
		pub fn XInputGetState (dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD;

		// set vibration state of particular controller
		// return value on success is ERROR_SUCCESS
		// return value if controller is not connected is ERROR_DEVICE_NOT_CONNECTED
		// function does not use SetLastError to set error
		pub fn XInputSetState (dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION) -> DWORD;
	}
}
pub use xinput::*;
pub use basic_types::*;
pub use error_constants::*;

mod basic_types {
	use std::ffi::c_int;

	pub type WORD = u16;
	pub type BYTE = u8;
	pub type SHORT = i16;
	pub type DWORD = u32;
	pub type BOOL = c_int;
}

mod error_constants {
	use super::basic_types::*;
	pub const ERROR_SUCCESS: DWORD = 0x0;
}

