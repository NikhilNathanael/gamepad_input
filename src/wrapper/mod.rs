use super::winapi::*;

pub mod buttons;
use buttons::GamePadButton;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XInputGamepad {
	pub id : DWORD,
	pub buttons: GamePadButton,
	pub left_trigger: u8,
	pub right_trigger: u8,
	pub left_thumb: [i16;2],
	pub right_thumb: [i16;2],
}

impl XInputGamepad {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			buttons: Default::default(),
			left_trigger: Default::default(),
			right_trigger: Default::default(),
			left_thumb: Default::default(),
			right_thumb: Default::default(),
		}
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn update(&mut self) -> bool {
		let mut state: XINPUT_STATE = unsafe{std::mem::zeroed()};
		if unsafe{XInputGetState(self.id, &mut state as _) == ERROR_SUCCESS} {
			let gamepad = state.Gamepad;
			self.buttons = GamePadButton::from_u16(gamepad.wButtons);
			self.left_trigger = gamepad.bLeftTrigger;
			self.right_trigger = gamepad.bRightTrigger;
			self.left_thumb = [gamepad.sThumbLX, gamepad.sThumbLY];
			self.right_thumb = [gamepad.sThumbRX, gamepad.sThumbRY];
			true
		} else {
			false
		}
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
}
