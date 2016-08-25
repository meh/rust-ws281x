use ffi;
use super::{ChannelRef, ChannelMut};

#[derive(Debug)]
pub struct Channel(ffi::ws2811_channel_t);

impl Channel {
	pub unsafe fn new(value: ffi::ws2811_channel_t) -> Result<Self, ()> {
		Ok(Channel(value))
	}

	pub unsafe fn into_inner(self) -> ffi::ws2811_channel_t {
		self.0
	}

	pub fn as_ref(&self) -> ChannelRef {
		unsafe {
			ChannelRef::wrap(&self.0)
		}
	}

	pub fn as_mut(&mut self) -> ChannelMut {
		unsafe {
			ChannelMut::wrap(&mut self.0)
		}
	}
}
