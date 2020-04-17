
#[cfg_attr(target_os = "linux", path = "x11.rs")]
#[cfg_attr(target_os = "windows", path = "winapi.rs")]
mod connection;
pub use connection::Connection;
