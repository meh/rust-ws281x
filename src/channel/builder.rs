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

use std::{mem, ptr};
use libc::c_int;
use {ffi, Strip};
use super::Channel;

pub struct Builder(ffi::ws2811_channel_t);

impl Builder {
	pub fn new() -> Self {
		unsafe {
			Builder(mem::zeroed())
		}
	}

	pub fn pin(&mut self, value: i32) -> &mut Self {
		self.0.gpionum = value as c_int;
		self
	}

	pub fn count(&mut self, value: usize) -> &mut Self {
		self.0.count = value as c_int;
		self
	}

	pub fn invert(&mut self, value: bool) -> &mut Self {
		self.0.invert = if value { 1 } else { 0 };
		self
	}

	pub fn brightness(&mut self, value: u8) -> &mut Self {
		self.0.brightness = value;
		self
	}

	pub fn strip(&mut self, value: Strip) -> &mut Self {
		self.0.strip_type = value.into();
		self
	}

	pub fn build(&mut self) -> Result<Channel, ()> {
		unsafe {
			Channel::new(ptr::read(&self.0))
		}
	}
}
