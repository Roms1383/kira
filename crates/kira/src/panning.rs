use std::ops::{
	Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{tween::Tweenable, Value};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
/// The stereo positioning of a sound.
///
/// Valid panning values range from `-1.0` to `1.0`. A value of `-1.0`
/// will cause a sound to only be output from the left speaker. A value
/// of `1.0` will cause a sound to only be output from the right speaker.
/// A value of `0.0` will cause a sound to be played at an equal volume
/// from both speakers.
pub struct Panning(pub f32);

impl Panning {
	/// Play the sound from the left speaker only.
	pub const LEFT: Self = Self(-1.0);
	/// Play the sound from both speakers at the same volume.
	pub const CENTER: Self = Self(0.0);
	/// Play the sound from the right speaker only.
	pub const RIGHT: Self = Self(1.0);
}

impl Default for Panning {
	fn default() -> Self {
		Self::CENTER
	}
}

impl Tweenable for Panning {
	fn interpolate(a: Self, b: Self, amount: f64) -> Self {
		Self(Tweenable::interpolate(a.0, b.0, amount))
	}
}

impl From<f32> for Panning {
	fn from(value: f32) -> Self {
		debug_assert!(
			value >= -1.0 && value <= 1.0,
			"panning must be between -1.0 and 1.0 (inclusive)"
		);
		Self(value)
	}
}

impl From<f32> for Value<Panning> {
	fn from(value: f32) -> Self {
		Self::Fixed(value.into())
	}
}

impl From<Panning> for Value<Panning> {
	fn from(value: Panning) -> Self {
		Self::Fixed(value)
	}
}

impl Add<Panning> for Panning {
	type Output = Panning;

	fn add(self, rhs: Panning) -> Self::Output {
		Self((self.0 + rhs.0).clamp(Self::LEFT.0, Self::RIGHT.0))
	}
}

impl AddAssign<Panning> for Panning {
	fn add_assign(&mut self, rhs: Panning) {
		*self = self.add(rhs);
	}
}

impl Sub<Panning> for Panning {
	type Output = Panning;

	fn sub(self, rhs: Panning) -> Self::Output {
		Self((self.0 - rhs.0).clamp(Self::LEFT.0, Self::RIGHT.0))
	}
}

impl SubAssign<Panning> for Panning {
	fn sub_assign(&mut self, rhs: Panning) {
		*self = self.sub(rhs);
	}
}

impl Mul<f32> for Panning {
	type Output = Panning;

	fn mul(self, rhs: f32) -> Self::Output {
		Self((self.0 * rhs).clamp(Self::LEFT.0, Self::RIGHT.0))
	}
}

impl MulAssign<f32> for Panning {
	fn mul_assign(&mut self, rhs: f32) {
		*self = self.mul(rhs);
	}
}

impl Div<f32> for Panning {
	type Output = Panning;

	fn div(self, rhs: f32) -> Self::Output {
		debug_assert!(!rhs.is_nan(), "cannot divide by NaN");
		debug_assert_ne!(rhs, 0.0, "cannot divide by zero");
		Self((self.0 / rhs).clamp(Self::LEFT.0, Self::RIGHT.0))
	}
}

impl DivAssign<f32> for Panning {
	fn div_assign(&mut self, rhs: f32) {
		*self = self.div(rhs);
	}
}

impl Neg for Panning {
	type Output = Panning;

	fn neg(self) -> Self::Output {
		Self(-self.0)
	}
}

impl Rem<f32> for Panning {
	type Output = Panning;

	fn rem(self, rhs: f32) -> Self::Output {
		debug_assert!(!rhs.is_nan(), "cannot get remainder of NaN");
		Self((self.0 % rhs).clamp(Self::LEFT.0, Self::RIGHT.0))
	}
}

impl RemAssign<f32> for Panning {
	fn rem_assign(&mut self, rhs: f32) {
		*self = self.rem(rhs);
	}
}
