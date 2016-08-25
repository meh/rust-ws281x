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

#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;
pub mod ffi;

pub type Led = ffi::ws2811_led_t;

pub mod strip;
pub use strip::Strip;

pub mod handle;
pub use handle::Handle;

pub mod channel;
pub use channel::{Channel, ChannelRef, ChannelMut};
