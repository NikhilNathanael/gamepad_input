#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct GamePadButton (u16);

impl GamePadButton {
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
}

impl std::ops::BitOr<GamePadButton> for GamePadButton {
	type Output = Self;
	fn bitor(self, other: GamePadButton) -> Self::Output {
		GamePadButton(self.0 | other.0)
	}
}

impl std::ops::BitXor<GamePadButton> for GamePadButton {
	type Output = Self;
	fn bitxor(self, other: GamePadButton) -> Self::Output {
		GamePadButton(self.0 ^ other.0)
	}
}

impl GamePadButton {
	pub fn from_u16(data: u16) -> Self {
		Self(data)
	}

	pub fn into_u16(self) -> u16 {
		self.0
	}

	pub fn is_pressed(self, button: GamePadButton) -> bool {
		button.0 & self.0 == button.0
	}
}

impl std::fmt::Debug for GamePadButton {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.write_str(&BUTTONS.iter().filter(|x| self.is_pressed(x.0)).map(|x| x.1).collect::<Vec<_>>().join(" | "))
	}
}

const BUTTONS: [(GamePadButton, &str); 14] = [
	(GamePadButton::DPADUP, "GamePadButton::DPADUP"), 
	(GamePadButton::DPADDOWN, "GamePadButton::DPADDOWN"),
	(GamePadButton::DPADLEFT, "GamePadButton::DPADLEFT"),
	(GamePadButton::DPADRIGHT, "GamePadButton::DPADRIGHT"),
	(GamePadButton::START, "GamePadButton::START"),
	(GamePadButton::BACK, "GamePadButton::BACK"),
	(GamePadButton::LEFTTHUMB, "GamePadButton::LEFTTHUMB"),
	(GamePadButton::RIGHTTHUMB, "GamePadButton::RIGHTTHUMB"),
	(GamePadButton::LEFTSHOULDER, "GamePadButton::LEFTSHOULDER"),
	(GamePadButton::RIGHTSHOULDER, "GamePadButton::RIGHTSHOULDER"),
	(GamePadButton::ABUTTON, "GamePadButton::ABUTTON"),
	(GamePadButton::BBUTTON, "GamePadButton::BBUTTON"),
	(GamePadButton::XBUTTON, "GamePadButton::XBUTTON"),
	(GamePadButton::YBUTTON, "GamePadButton::YBUTTON"),
];
