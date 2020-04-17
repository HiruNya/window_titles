pub use connection::Connection;

#[cfg_attr(target_os = "linux", path = "x11.rs")]
#[cfg_attr(target_os = "windows", path = "winapi.rs")]
#[cfg_attr(target_os = "macos", path = "apple.rs")]
mod connection;
