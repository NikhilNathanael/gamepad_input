use super::winapi::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XInputGamepad {
	id : DWORD,
	inner: XINPUT_GAMEPAD,
}

impl XInputGamepad {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			inner: unsafe{std::mem::zeroed()}
		}
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn update(&mut self) -> bool {
		let mut state: XINPUT_STATE = unsafe{std::mem::zeroed()};
		if unsafe{XInputGetState(self.id, &mut state as _) == ERROR_SUCCESS} {
			self.inner = state.Gamepad;
			return true;
		}
		false
	}

	pub fn set_vibration(&self, left: u16, right: u16) -> bool {
		let mut vibration = XINPUT_VIBRATION{
			wLeftMotorSpeed: left,
			wRightMotorSpeed: right,
		};
		let result = unsafe{XInputSetState(self.id(), &mut vibration as *mut _)};
		if result == ERROR_SUCCESS {
			true
		} else {
			false
		}
	}

	// pub fn l_stick(&self) -> 
}
