use std::slice;
use std::marker::PhantomData;
use {ffi, Led, Strip};

#[derive(Debug)]
pub struct Ref<'a>(*const ffi::ws2811_channel_t, PhantomData<&'a ()>);

impl<'a> Ref<'a> {
	pub unsafe fn wrap(ptr: *const ffi::ws2811_channel_t) -> Self {
		Ref(ptr, PhantomData)
	}

	pub fn pin(&self) -> i32 {
		unsafe {
			(*self.0).gpionum as i32
		}
	}

	pub fn invert(&mut self) -> bool {
		unsafe {
			if (*self.0).invert == 0 { false } else { true }
		}
	}

	pub fn brightness(&mut self) -> i32 {
		unsafe {
			(*self.0).brightness as i32
		}
	}

	pub fn strip(&mut self) -> Strip {
		unsafe {
			Strip::from((*self.0).strip_type)
		}
	}

	pub fn leds(&mut self) -> &[Led] {
		unsafe {
			slice::from_raw_parts((*self.0).leds, (*self.0).count as usize)
		}
	}
}
