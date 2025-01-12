use std::{error::Error, io::stdin};

use kira::{
	sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
	Panning,
};

fn main() -> Result<(), Box<dyn Error>> {
	let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
	let sound_data = StaticSoundData::from_file("crates/examples/assets/score.ogg")?;
	let pannings = vec![
        // invalid values
		Panning(f32::NEG_INFINITY),
		Panning(-10.0),
        // valid values
		Panning::LEFT,
        Panning(-0.5),
        Panning(-0.1),
		Panning::CENTER,
        Panning(0.1),
        Panning(0.5),
		Panning::RIGHT,
        // invalid values
		Panning(10.0),
		Panning(f32::INFINITY),
		Panning(f32::NAN),
	];
	let mut idx = 0;

	println!("Press enter to play a sound");
	loop {
		wait_for_enter_press()?;
		println!("current panning: {:?}", pannings[idx]);
		manager.play(sound_data.panning(pannings[idx]).clone())?;

		idx = (idx + 1) % pannings.len();
	}
}

fn wait_for_enter_press() -> Result<(), Box<dyn Error>> {
	stdin().read_line(&mut "".into())?;
	Ok(())
}
