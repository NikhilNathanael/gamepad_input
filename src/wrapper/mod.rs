use super::winapi::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct GamePadButton (u16);
pub const DPADUP		: GamePadButton	= GamePadButton(0x0001);
pub const DPADDOWN		: GamePadButton	= GamePadButton(0x0002);
pub const DPADLEFT		: GamePadButton	= GamePadButton(0x0004);
pub const DPADRIGHT		: GamePadButton	= GamePadButton(0x0008);
pub const START			: GamePadButton	= GamePadButton(0x0010);
pub const BACK			: GamePadButton	= GamePadButton(0x0020);
pub const LEFTTHUMB		: GamePadButton	= GamePadButton(0x0040);
pub const RIGHTTHUMB	: GamePadButton	= GamePadButton(0x0080);
pub const LEFTSHOULDER	: GamePadButton	= GamePadButton(0x0100);
pub const RIGHTSHOULDER	: GamePadButton	= GamePadButton(0x0200);
pub const ABUTTON		: GamePadButton	= GamePadButton(0x1000);
pub const BBUTTON		: GamePadButton	= GamePadButton(0x2000);
pub const XBUTTON		: GamePadButton	= GamePadButton(0x4000);
pub const YBUTTON		: GamePadButton	= GamePadButton(0x8000);

impl GamePadButton {
	pub fn is_pressed(self, button: GamePadButton) -> bool {
		button.0 & self.0 == button.0
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XInputGamepad {
	id : DWORD,
	buttons: GamePadButton,
	left_trigger: u8,
	right_trigger: u8,
	left_thumb: [i16;2],
	right_thumb: [i16;2],
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
			self.buttons = GamePadButton(gamepad.wButtons);
			self.left_trigger = gamepad.bLeftTrigger;
			self.right_trigger = gamepad.bRightTrigger;
			self.left_thumb = [gamepad.sThumbLX, gamepad.sThumbLY];
			self.right_thumb = [gamepad.sThumbRX, gamepad.sThumbRY];
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
}
