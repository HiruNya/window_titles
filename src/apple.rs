use std::{error::Error, fmt, process::Command};

use crate::{ConnectionTrait, Result};

const PREFIX: &str = r#"tell application "System Events""#;
const SUFFIX: &str = r#"to get the title of every window of every process"#;
const PERMISSION_ERROR: &str = "osascript is not allowed assistive access";

pub struct Connection;
impl ConnectionTrait for Connection {
	fn new() -> Result<Self> { Ok(Self) }
	fn window_titles(&self) -> Result<Vec<String>> {
		let arguments = &["-ss", "-e", &format!("{} {}", PREFIX, SUFFIX)];
		let command = Command::new("osascript").args(arguments).output()
			.expect("failed to execute AppleScript command");

		let error = String::from_utf8_lossy(&command.stderr);
		match error.contains(PERMISSION_ERROR) {
			true => Err(WindowTitleError::NoAccessibilityPermission.into()),
			false => Ok(split(&String::from_utf8_lossy(&command.stdout))),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum WindowTitleError {
	NoAccessibilityPermission,
}
impl fmt::Display for WindowTitleError {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result<> {
		match self {
			WindowTitleError::NoAccessibilityPermission => write!(fmt, "Permission to use the accessibility API has not been granted")
		}
	}
}
impl Error for WindowTitleError {}

fn split(mut string: &str) -> Vec<String> {
	let mut titles = Vec::new();
	while let Some(start) = string.find('"') {
		let end = string[start + 1..].find('"').unwrap();
		titles.push(string[start + 1..][..end].to_string());
		string = &string[start + 1..][end + 1..];
	}
	titles
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split() {
		let string = r#"{{}, {"0"}, {"1", "2"}}"#;
		assert_eq!(split(string), &["0", "1", "2"]);
	}
}
