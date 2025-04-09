use super::winapi::*;

pub mod buttons;
use buttons::GamePadButton;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GamepadID {
	Id_0 = 0,
	Id_1 = 1,
	Id_2 = 2,
	Id_3 = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XInputGamepad {
	pub id : GamepadID,
	pub buttons: GamePadButton,
	pub left_trigger: f32,
	pub right_trigger: f32,
	pub left_thumb: [f32;2],
	pub right_thumb: [f32;2],
}

pub struct GamepadDiff {
	pub buttons: GamePadButton,
	pub left_trigger: f32,
	pub right_trigger: f32,
	pub left_thumb: [f32;2],
	pub right_thumb: [f32;2],
}

impl XInputGamepad {
	pub fn get_state(id: GamepadID) -> Option<Self> {
		let mut state: XINPUT_STATE = unsafe{std::mem::zeroed()};
		if unsafe{XInputGetState(id as u32, &mut state as _) == ERROR_SUCCESS} {
			let gamepad = state.Gamepad;
			Some(Self {
				id,
				buttons: GamePadButton::from_u16(gamepad.wButtons),
				left_trigger: gamepad.bLeftTrigger as f32 / u8::MAX as f32,
				right_trigger: gamepad.bRightTrigger as f32 / u8::MAX as f32,
				left_thumb: [gamepad.sThumbLX as f32 / i16::MAX as f32, gamepad.sThumbLY as f32 / i16::MAX as f32],
				right_thumb: [gamepad.sThumbRX as f32 / i16::MAX as f32, gamepad.sThumbRY as f32 / i16::MAX as f32],
			})
		} else {
			None
		}
	}

	pub fn set_vibration(id: u32, left: u16, right: u16) -> bool {
		let mut vibration = XINPUT_VIBRATION{
			wLeftMotorSpeed: left,
			wRightMotorSpeed: right,
		};
		let result = unsafe{XInputSetState(id, &mut vibration as *mut _)};
		if result == ERROR_SUCCESS {
			true
		} else {
			false
		}
	}

	// NOT TESTED
	pub fn diff(&self, other: &Self) -> GamepadDiff {
		GamepadDiff {
			buttons: self.buttons ^ other.buttons,
			left_trigger: self.left_trigger - other.left_trigger,
			right_trigger: self.right_trigger - other.right_trigger,
			left_thumb: [
				self.left_thumb[0] - other.left_thumb[0], 
				self.left_thumb[1] - other.left_thumb[1]
			],
			right_thumb: [
				self.right_thumb[0] - other.right_thumb[0], 
				self.right_thumb[1] - other.right_thumb[1]
			],
		}
	}
}
