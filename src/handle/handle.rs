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

use {ffi, ChannelRef, ChannelMut};

/// Handle to a WS281X device.
pub struct Handle(ffi::ws2811_t);

use handle::handle_error::HandleCreationError;

impl Handle {
	pub unsafe fn new(mut value: ffi::ws2811_t) -> Result<Self, HandleCreationError> {
		let status = ffi::ws2811_init(&mut value);
		if status >= 0 {
			Ok(Handle(value))
		}
		else {
			Err(HandleCreationError{code: status})
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
