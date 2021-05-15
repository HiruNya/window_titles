use std::{error::Error, result::Result as StdResult};

pub use connection::Connection;

#[cfg_attr(target_os = "linux", path = "x11.rs")]
#[cfg_attr(target_os = "windows", path = "winapi.rs")]
#[cfg_attr(target_os = "macos", path = "apple.rs")]
mod connection;

#[cfg_attr(target_os = "windows", path = "winapi.rs")]
pub mod windows;

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub trait ConnectionTrait: Sized {
	fn new() -> Result<Self>;
	fn window_titles(&self) -> Result<Vec<String>>;
}
