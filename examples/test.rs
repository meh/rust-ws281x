use std::thread;
use std::time::Duration;

extern crate ws281x;

fn main() {
	let mut handler = ws281x::handle::new()
		.dma(5)
		.channel(0, ws281x::channel::new()
			.pin(18)
			.count(8 * 8)
			.brightness(55)
			.build().unwrap())
		.build().unwrap();

	let mut check = 0;

	loop {
		for (i, led) in handler.channel_mut(0).leds_mut().iter_mut().enumerate() {
			if i % 2 == check {
				*led = 0
			}
			else {
				*led = 0xffffff;
			}
		}

		handler.render().unwrap();
		handler.wait().unwrap();

		thread::sleep(Duration::from_millis(500));
		check = if check == 0 { 1 } else { 0 };
	}
}
