extern crate cc;

fn main() {
	cc::Build::new()
            .file("c/mailbox.c")
            .file("c/ws2811.c")
            .file("c/pwm.c")
            .file("c/pcm.c")
            .file("c/dma.c")
            .file("c/rpihw.c")
            .compile("libws281x.a");
}
