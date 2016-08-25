use std::ops::Deref;
use std::slice;
use libc::c_int;
use {ffi, Led, Strip};
use super::ChannelRef;

#[derive(Debug)]
pub struct Ref<'a>(*mut ffi::ws2811_channel_t, ChannelRef<'a>);

impl<'a> Ref<'a> {
	pub unsafe fn wrap(ptr: *mut ffi::ws2811_channel_t) -> Self {
		Ref(ptr, ChannelRef::wrap(ptr as *const _))
	}

	pub fn set_pin(&mut self, value: i32) {
		unsafe {
			(*self.0).gpionum = value as c_int;
		}
	}

	pub fn set_invert(&mut self, value: bool) {
		unsafe {
			(*self.0).invert = if value { 1 } else { 0 };
		}
	}

	pub fn set_brightness(&mut self, value: i32) {
		unsafe {
			(*self.0).brightness = value as c_int;
		}
	}

	pub fn set_strip(&mut self, value: Strip) {
		unsafe {
			(*self.0).strip_type = value.into();
		}
	}

	pub fn leds_mut(&mut self) -> &mut [Led] {
		unsafe {
			slice::from_raw_parts_mut((*self.0).leds, (*self.0).count as usize)
		}
	}
}

impl<'a> Deref for Ref<'a> {
	type Target = ChannelRef<'a>;

	fn deref(&self) -> &Self::Target {
		&self.1
	}
}
