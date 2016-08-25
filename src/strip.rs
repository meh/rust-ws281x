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

use libc::c_int;
use ffi;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Strip {
	RGBW,
	RBGW,
	GRBW,
	GBRW,
	BRGW,
	BGRW,
	RGB,
	RBG,
	GRB,
	GBR,
	BRG,
	BGR,
}

pub const WS2812:  Strip = Strip::GRB;
pub const SK6812:  Strip = Strip::GRB;
pub const SK6812W: Strip = Strip::GRBW;

impl From<c_int> for Strip {
	fn from(value: c_int) -> Strip {
		match value {
			ffi::SK6812_STRIP_RGBW => Strip::RGBW,
			ffi::SK6812_STRIP_RBGW => Strip::RBGW,
			ffi::SK6812_STRIP_GRBW => Strip::GRBW,
			ffi::SK6812_STRIP_GBRW => Strip::GBRW,
			ffi::SK6812_STRIP_BRGW => Strip::BRGW,
			ffi::SK6812_STRIP_BGRW => Strip::BGRW,
			ffi::WS2811_STRIP_RGB  => Strip::RGB,
			ffi::WS2811_STRIP_RBG  => Strip::RBG,
			ffi::WS2811_STRIP_GRB  => Strip::GRB,
			ffi::WS2811_STRIP_GBR  => Strip::GBR,
			ffi::WS2811_STRIP_BRG  => Strip::BRG,
			ffi::WS2811_STRIP_BGR  => Strip::BGR,

			_ => unreachable!()
		}
	}
}

impl Into<c_int> for Strip {
	fn into(self) -> c_int {
		match self {
			Strip::RGBW => ffi::SK6812_STRIP_RGBW,
			Strip::RBGW => ffi::SK6812_STRIP_RBGW,
			Strip::GRBW => ffi::SK6812_STRIP_GRBW,
			Strip::GBRW => ffi::SK6812_STRIP_GBRW,
			Strip::BRGW => ffi::SK6812_STRIP_BRGW,
			Strip::BGRW => ffi::SK6812_STRIP_BGRW,
			Strip::RGB  => ffi::WS2811_STRIP_RGB,
			Strip::RBG  => ffi::WS2811_STRIP_RBG,
			Strip::GRB  => ffi::WS2811_STRIP_GRB,
			Strip::GBR  => ffi::WS2811_STRIP_GBR,
			Strip::BRG  => ffi::WS2811_STRIP_BRG,
			Strip::BGR  => ffi::WS2811_STRIP_BGR,
		}
	}
}
