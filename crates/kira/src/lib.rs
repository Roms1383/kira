/*!
# Kira

Kira is a backend-agnostic library to create expressive audio
for games. Besides the common sound playback features, it provides
[tweens](tween) for smoothly adjusting properties of sounds, a
flexible [mixer](track) for applying effects to audio, and a
[clock] system for precisely timing audio events.

## Related crates

You will most likely want to use Kira with some of these other
crates:

- [`kira-cpal`](https://crates.io/crates/kira-cpal) - backend for
Windows, Mac, and Linux targets
- [`kira-loaders`](https://crates.io/crates/kira-loaders) - adds
support for loading audio files

*/

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]
#![warn(clippy::todo)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

pub mod clock;
mod clock_speed;
pub mod dsp;
mod error;
mod loop_behavior;
pub mod manager;
mod playback_rate;
pub mod sound;
mod start_time;
pub mod track;
pub mod tween;
mod volume;

pub use clock_speed::*;
pub use error::*;
pub use loop_behavior::*;
pub use playback_rate::*;
pub use start_time::*;
pub use volume::*;
