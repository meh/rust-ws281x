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

use libc::{c_void, c_char, c_int, uint32_t, uint8_t, uint64_t};

pub const RPI_PWM_CHANNELS: usize = 2;

pub const RPI_PWM_CTL_MSEN2: uint32_t = 1 << 15;
pub const RPI_PWM_CTL_USEF2: uint32_t = 1 << 13;
pub const RPI_PWM_CTL_POLA2: uint32_t = 1 << 12;
pub const RPI_PWM_CTL_SBIT2: uint32_t = 1 << 11;
pub const RPI_PWM_CTL_RPTL2: uint32_t = 1 << 10;
pub const RPI_PWM_CTL_MODE2: uint32_t = 1 << 9;
pub const RPI_PWM_CTL_PWEN2: uint32_t = 1 << 8;
pub const RPI_PWM_CTL_MSEN1: uint32_t = 1 << 7;
pub const RPI_PWM_CTL_CLRF1: uint32_t = 1 << 6;
pub const RPI_PWM_CTL_USEF1: uint32_t = 1 << 5;
pub const RPI_PWM_CTL_POLA1: uint32_t = 1 << 4;
pub const RPI_PWM_CTL_SBIT1: uint32_t = 1 << 3;
pub const RPI_PWM_CTL_RPTL1: uint32_t = 1 << 2;
pub const RPI_PWM_CTL_MODE1: uint32_t = 1 << 1;
pub const RPI_PWM_CTL_PWEN1: uint32_t = 1 << 0;

pub const RPI_PWM_STA_STA4:  uint32_t = 1 << 12;
pub const RPI_PWM_STA_STA3:  uint32_t = 1 << 11;
pub const RPI_PWM_STA_STA2:  uint32_t = 1 << 10;
pub const RPI_PWM_STA_STA1:  uint32_t = 1 << 9;
pub const RPI_PWM_STA_BERR:  uint32_t = 1 << 8;
pub const RPI_PWM_STA_GAP04: uint32_t = 1 << 7;
pub const RPI_PWM_STA_GAP03: uint32_t = 1 << 6;
pub const RPI_PWM_STA_GAP02: uint32_t = 1 << 5;
pub const RPI_PWM_STA_GAP01: uint32_t = 1 << 4;
pub const RPI_PWM_STA_RERR1: uint32_t = 1 << 3;
pub const RPI_PWM_STA_WERR1: uint32_t = 1 << 2;
pub const RPI_PWM_STA_EMPT1: uint32_t = 1 << 1;
pub const RPI_PWM_STA_FULL1: uint32_t = 1 << 0;

pub const RPI_PWM_DMAC_ENAB: uint32_t = 1 << 31;

#[inline(always)]
pub unsafe fn RPI_PWM_DMAC_PANIC(val: uint32_t) -> uint32_t {
	(val & 0xff) << 8
}

pub unsafe fn RPI_PWM_DMAC_DREQ(val: uint32_t) -> uint32_t {
	val & 0xff
}

#[derive(Debug)]
#[repr(C)]
pub struct pwm_t {
	pub ctl: uint32_t,
	pub sta: uint32_t,
	pub dmac: uint32_t,
	pub resvd_0x0c: uint32_t,
	pub rng1: uint32_t,
	pub dat1: uint32_t,
	pub fif1: uint32_t,
	pub resvd_0x1c: uint32_t,
	pub rng2: uint32_t,
	pub dat2: uint32_t,
}

pub const PWM_OFFSET:      uint32_t = 0x0020c000;
pub const PWM_PERIPH_PHYS: uint32_t = 0x7e20c000;

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

pub const RPI_HWVER_TYPE_UNKNOWN: uint32_t = 0;
pub const RPI_HWVER_TYPE_PI1:     uint32_t = 1;
pub const RPI_HWVER_TYPE_PI2:     uint32_t = 2;

#[derive(Debug)]
#[repr(C)]
pub struct rpi_hw_t {
	pub type_: uint32_t,
	pub hwver: uint32_t,
	pub periph_base: uint32_t,
	pub videocore_base: uint32_t,
	pub desc: *mut c_char,
}

// Can go as low as 400000
pub const WS2811_TARGET_FREQ: uint32_t = 800000;
pub const SK6812_SHIFT_WMASK: uint32_t = 0xf0000000;

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
pub type ws2811_led_t = uint32_t;

#[derive(Debug)]
#[repr(C)]
pub struct ws2811_channel_t {
	pub gpionum: c_int,
	pub invert: c_int,
	pub count: c_int,
	pub strip_type: c_int,
	pub leds: *mut ws2811_led_t,
	pub brightness: uint8_t,
	pub wshift: uint8_t,
	pub rshift: uint8_t,
	pub gshift: uint8_t,
	pub bshift: uint8_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct ws2811_t {
	pub render_wait_until: uint64_t,
	pub device: *mut ws2811_device_t,
	pub rpi_hw: *const rpi_hw_t,
	pub freq: uint32_t,
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
