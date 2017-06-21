extern crate gcc;

fn main() {
	gcc::compile_library("libws281x.a", &["c/mailbox.c", "c/ws2811.c", "c/pwm.c", "c/dma.c", "c/rpihw.c", "c/pcm.c"]);
}
