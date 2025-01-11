use std::ops::{
	Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{tween::Tweenable, Value};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The stereo positioning of a sound.
///
/// Valid panning values range from `-1.0` to `1.0`. A value of `-1.0`
/// will cause a sound to only be output from the left speaker. A value
/// of `1.0` will cause a sound to only be output from the right speaker.
/// A value of `0.0` will cause a sound to be played at an equal volume
/// from both speakers.
pub struct Panning(#[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize"))] f32);

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

impl Deref for Panning {
	type Target = f32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<f32> for Panning {
	fn from(value: f32) -> Self {
		debug_assert!(
			(-1.0..=1.0).contains(&value),
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
		self.saturating_add(rhs.0)
	}
}

impl AddAssign<Panning> for Panning {
	#[inline]
	fn add_assign(&mut self, rhs: Panning) {
		*self = self.add(rhs);
	}
}

impl Sub<Panning> for Panning {
	type Output = Panning;

	fn sub(self, rhs: Panning) -> Self::Output {
		self.saturating_sub(rhs.0)
	}
}

impl SubAssign<Panning> for Panning {
	#[inline]
	fn sub_assign(&mut self, rhs: Panning) {
		*self = self.sub(rhs);
	}
}

impl Mul<f32> for Panning {
	type Output = Panning;

	fn mul(self, rhs: f32) -> Self::Output {
		self.saturating_mul(rhs)
	}
}

impl MulAssign<f32> for Panning {
	#[inline]
	fn mul_assign(&mut self, rhs: f32) {
		*self = self.mul(rhs);
	}
}

impl Div<f32> for Panning {
	type Output = Panning;

	fn div(self, rhs: f32) -> Self::Output {
		self.saturating_div(rhs)
	}
}

impl DivAssign<f32> for Panning {
	#[inline]
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
		self.saturating_rem(rhs)
	}
}

impl RemAssign<f32> for Panning {
	#[inline]
	fn rem_assign(&mut self, rhs: f32) {
		*self = self.rem(rhs);
	}
}

#[cfg(feature = "serde")]
fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
	D: serde::de::Deserializer<'de>,
{
	use serde::Deserialize;
	let value = f32::deserialize(deserializer)?;
	if (-1.0..=1.0).contains(&value) {
		Ok(value)
	} else {
		Err(serde::de::Error::invalid_value(
			serde::de::Unexpected::Float(value.into()),
			&"panning must be between -1.0 and 1.0 (inclusive)",
		))
	}
}

#[inline]
const fn bounded(value: f32) -> Panning {
	match value {
		x if x < Panning::LEFT.0 => Panning::LEFT,
		x if x > Panning::RIGHT.0 => Panning::RIGHT,
		x => Panning(x),
	}
}

impl Panning {
	#[inline]
	const fn saturating_add(self, v: f32) -> Self {
		debug_assert!(!v.is_nan(), "cannot add by NaN");
		bounded(self.0 + v)
	}
	#[inline]
	const fn saturating_sub(self, v: f32) -> Self {
		debug_assert!(!v.is_nan(), "cannot substract by NaN");
		bounded(self.0 - v)
	}
	#[inline]
	const fn saturating_mul(self, v: f32) -> Self {
		debug_assert!(!v.is_nan(), "cannot multiply by NaN");
		bounded(self.0 * v)
	}
	#[inline]
	const fn saturating_div(self, v: f32) -> Self {
		debug_assert!(!v.is_nan(), "cannot divide by NaN");
		debug_assert!(v != 0.0, "cannot divide by zero");
		bounded(self.0 / v)
	}
	#[inline]
	const fn saturating_rem(self, v: f32) -> Self {
		debug_assert!(!v.is_nan(), "cannot get remainder by NaN");
		debug_assert!(v != 0.0, "cannot get remainder by zero");
		bounded(self.0 % v)
	}
}
