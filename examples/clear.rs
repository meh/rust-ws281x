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

	handler.channel_mut(0).set_brightness(0);
	handler.render().unwrap();
	handler.wait().unwrap();
}
