//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use libc::{c_void, c_char, c_int};

pub const RPI_PWM_CHANNELS: usize = 2;

pub const RPI_PWM_CTL_MSEN2: u32 = 1 << 15;
pub const RPI_PWM_CTL_USEF2: u32 = 1 << 13;
pub const RPI_PWM_CTL_POLA2: u32 = 1 << 12;
pub const RPI_PWM_CTL_SBIT2: u32 = 1 << 11;
pub const RPI_PWM_CTL_RPTL2: u32 = 1 << 10;
pub const RPI_PWM_CTL_MODE2: u32 = 1 << 9;
pub const RPI_PWM_CTL_PWEN2: u32 = 1 << 8;
pub const RPI_PWM_CTL_MSEN1: u32 = 1 << 7;
pub const RPI_PWM_CTL_CLRF1: u32 = 1 << 6;
pub const RPI_PWM_CTL_USEF1: u32 = 1 << 5;
pub const RPI_PWM_CTL_POLA1: u32 = 1 << 4;
pub const RPI_PWM_CTL_SBIT1: u32 = 1 << 3;
pub const RPI_PWM_CTL_RPTL1: u32 = 1 << 2;
pub const RPI_PWM_CTL_MODE1: u32 = 1 << 1;
pub const RPI_PWM_CTL_PWEN1: u32 = 1 << 0;

pub const RPI_PWM_STA_STA4:  u32 = 1 << 12;
pub const RPI_PWM_STA_STA3:  u32 = 1 << 11;
pub const RPI_PWM_STA_STA2:  u32 = 1 << 10;
pub const RPI_PWM_STA_STA1:  u32 = 1 << 9;
pub const RPI_PWM_STA_BERR:  u32 = 1 << 8;
pub const RPI_PWM_STA_GAP04: u32 = 1 << 7;
pub const RPI_PWM_STA_GAP03: u32 = 1 << 6;
pub const RPI_PWM_STA_GAP02: u32 = 1 << 5;
pub const RPI_PWM_STA_GAP01: u32 = 1 << 4;
pub const RPI_PWM_STA_RERR1: u32 = 1 << 3;
pub const RPI_PWM_STA_WERR1: u32 = 1 << 2;
pub const RPI_PWM_STA_EMPT1: u32 = 1 << 1;
pub const RPI_PWM_STA_FULL1: u32 = 1 << 0;

pub const RPI_PWM_DMAC_ENAB: u32 = 1 << 31;

#[inline(always)]
pub unsafe fn RPI_PWM_DMAC_PANIC(val: u32) -> u32 {
	(val & 0xff) << 8
}

pub unsafe fn RPI_PWM_DMAC_DREQ(val: u32) -> u32 {
	val & 0xff
}

#[derive(Debug)]
#[repr(C)]
pub struct pwm_t {
	pub ctl: u32,
	pub sta: u32,
	pub dmac: u32,
	pub resvd_0x0c: u32,
	pub rng1: u32,
	pub dat1: u32,
	pub fif1: u32,
	pub resvd_0x1c: u32,
	pub rng2: u32,
	pub dat2: u32,
}

pub const PWM_OFFSET:      u32 = 0x0020c000;
pub const PWM_PERIPH_PHYS: u32 = 0x7e20c000;

#[derive(Debug)]
#[repr(C)]
pub struct pwm_pin_table_t {
	pinnum: c_int,
	altnum: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct pwm_pin_tables_t {
	count: c_int,
	pins: *const pwm_pin_table_t,
}

pub const RPI_HWVER_TYPE_UNKNOWN: u32 = 0;
pub const RPI_HWVER_TYPE_PI1:     u32 = 1;
pub const RPI_HWVER_TYPE_PI2:     u32 = 2;

#[derive(Debug)]
#[repr(C)]
pub struct rpi_hw_t {
	pub type_: u32,
	pub hwver: u32,
	pub periph_base: u32,
	pub videocore_base: u32,
	pub desc: *mut c_char,
}

// Can go as low as 400000
pub const WS2811_TARGET_FREQ: u32 = 800000;
pub const SK6812_SHIFT_WMASK: u32 = 0xf0000000;

// 4 color R, G, B and W ordering
pub const SK6812_STRIP_RGBW:  c_int = 0x18100800;
pub const SK6812_STRIP_RBGW:  c_int = 0x18100008;
pub const SK6812_STRIP_GRBW:  c_int = 0x18081000;
pub const SK6812_STRIP_GBRW:  c_int = 0x18080010;
pub const SK6812_STRIP_BRGW:  c_int = 0x18001008;
pub const SK6812_STRIP_BGRW:  c_int = 0x18000810;

// 3 color R, G and B ordering
pub const WS2811_STRIP_RGB: c_int = 0x00100800;
pub const WS2811_STRIP_RBG: c_int = 0x00100008;
pub const WS2811_STRIP_GRB: c_int = 0x00081000;
pub const WS2811_STRIP_GBR: c_int = 0x00080010;
pub const WS2811_STRIP_BRG: c_int = 0x00001008;
pub const WS2811_STRIP_BGR: c_int = 0x00000810;

// predefined fixed LED types
pub const WS2812_STRIP:  c_int = WS2811_STRIP_GRB;
pub const SK6812_STRIP:  c_int = WS2811_STRIP_GRB;
pub const SK6812W_STRIP: c_int = SK6812_STRIP_GRBW;

pub type ws2811_device_t = c_void;
pub type ws2811_led_t = u32;

#[derive(Debug)]
#[repr(C)]
pub struct ws2811_channel_t {
	pub gpionum: c_int,
	pub invert: c_int,
	pub count: c_int,
	pub strip_type: c_int,
	pub leds: *mut ws2811_led_t,
	pub brightness: u8,
	pub wshift: u8,
	pub gshift: u8,
	pub bshift: u8,
	pub gamma: *mut u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct ws2811_t {
	pub render_wait_time: u64,
	pub device: *mut ws2811_device_t,
	pub rpi_hw: *const rpi_hw_t,
	pub freq: u32,
	pub dmanum: c_int,
	pub channel: [ws2811_channel_t; RPI_PWM_CHANNELS],
}

extern "C" {
	pub fn rpi_hw_detect() -> *const rpi_hw_t;
	pub fn pwm_pin_alt(chan: c_int, pinnum: c_int) -> c_int;

	pub fn ws2811_init(ws2811: *mut ws2811_t) -> c_int;
	pub fn ws2811_fini(ws2811: *mut ws2811_t);
	pub fn ws2811_render(ws2811: *mut ws2811_t) -> c_int;
	pub fn ws2811_wait(ws2811: *mut ws2811_t) -> c_int;
}
