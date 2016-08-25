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
use libc::{c_int, uint32_t};

use {ffi, Channel, ChannelRef, ChannelMut};

pub struct Handle(ffi::ws2811_t);

impl Handle {
	fn new(mut value: ffi::ws2811_t) -> Result<Self, ()> {
		unsafe {
			if ffi::ws2811_init(&mut value) >= 0 {
				Ok(Handle(value))
			}
			else {
				Err(())
			}
		}
	}

	pub fn render(&mut self) -> Result<(), ()> {
		unsafe {
			if ffi::ws2811_render(&mut self.0) >= 0 {
				Ok(())
			}
			else {
				Err(())
			}
		}
	}

	pub fn wait(&mut self) -> Result<(), ()> {
		unsafe {
			if ffi::ws2811_wait(&mut self.0) >= 0 {
				Ok(())
			}
			else {
				Err(())
			}
		}
	}

	pub fn channel(&self, index: usize) -> ChannelRef {
		unsafe {
			ChannelRef::wrap(&self.0.channel[index])
		}
	}

	pub fn channel_mut(&mut self, index: usize) -> ChannelMut {
		unsafe {
			ChannelMut::wrap(&mut self.0.channel[index])
		}
	}
}

impl Drop for Handle {
	fn drop(&mut self) {
		unsafe {
			ffi::ws2811_fini(&mut self.0)
		}
	}
}

pub struct Builder(ffi::ws2811_t);

impl Builder {
	pub fn new() -> Self {
		unsafe {
			let mut handle: ffi::ws2811_t = mem::zeroed();
			handle.freq = ffi::WS2811_TARGET_FREQ;

			Builder(handle)
		}
	}

	pub fn frequency(&mut self, value: u32) -> &mut Self {
		self.0.freq = value as uint32_t;
		self
	}

	pub fn dma(&mut self, value: i32) -> &mut Self {
		self.0.dmanum = value as c_int;
		self
	}

	pub fn channel(&mut self, index: usize, value: Channel) -> &mut Self {
		unsafe {
			self.0.channel[index] = value.into_inner();
		}

		self
	}

	pub fn build(&mut self) -> Result<Handle, ()> {
		unsafe {
			Handle::new(ptr::read(&self.0))
		}
	}
}

pub fn new() -> Builder {
	Builder::new()
}
