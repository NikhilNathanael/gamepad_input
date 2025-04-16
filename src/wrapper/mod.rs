use super::winapi::*;

pub mod buttons;
use buttons::GamePadButton;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GamepadID {
	Id0 = 0,
	Id1 = 1,
	Id2 = 2,
	Id3 = 3,
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

pub struct GamepadMap {
	prev: [Option<XInputGamepad>; 4],
	current: [Option<XInputGamepad>; 4],
}

impl GamepadMap {
	pub fn new() -> Self {
		Self {
			prev: [None; 4],
			current: [None; 4],
		}
	}

	pub fn current(&self, id: GamepadID) -> Option<&XInputGamepad> {
		self.current[id as u32 as usize].as_ref()
	}

	pub fn prev(&self, id: GamepadID) -> Option<&XInputGamepad> {
		self.prev[id as u32 as usize].as_ref()
	}

	pub fn update(&mut self) {
		self.prev = self.current;
		self.current[0] = XInputGamepad::get_state(GamepadID::Id0);
		self.current[1] = XInputGamepad::get_state(GamepadID::Id1);
		self.current[2] = XInputGamepad::get_state(GamepadID::Id2);
		self.current[3] = XInputGamepad::get_state(GamepadID::Id3);
	}
}
