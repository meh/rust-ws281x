use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HandleCreationError {
	pub code: i32
}

impl Error for HandleCreationError {
	fn description(&self) -> &str {
		match self.code {
			-1 => "Generic failure",
			-2 => "Out of memory",
			-3 => "Hardware revision is not supported",
			-4 => "Memory lock failed",
			-5 => "mmap() failed",
			-6 => "Unable to map registers into userspace",
			-7 => "Unable to initialize GPIO",
			-8 => "Unable to initialize PWM",
			-9 => "Failed to create mailbox device",
			-10 => "DMA error",
			-11 => "Selected GPIO not possible",
			-12 => "Unable to initialize PCM",
			-13 => "Unable to initialize SPI",
			-14 => "SPI transfer error",
			_ => "unrecognised error"
		}
	}
}

impl fmt::Display for HandleCreationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Handle creation failed, error code: {}. {}", self.code, self.description())
	}
}
