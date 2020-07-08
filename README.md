# window_titles

`window_titles` is a small crossplatform utility crate with the only job of getting the titles of windows.

It supports:

- **Linux / x11**:
Using [`xcb`] to query the x11 server. (Safe)
- **Windows**:
Using [`winapi`]. (Possibly Unsafe)
- **MacOS**:
Using the `osascript` command. (Safe)

Usage is simple:

1. Import both `Connection` and `ConnectionTrait`.

```rs
use window_titles::{Connection, ConnectionTrait};
```

2. Initiate the connection (Only Linux can return an error for this method).

```rs
let connection = Connection::new()?;
```

3. Get the window titles.

```rs
let titles: Vec<String> = connection.window_titles()?;
```

[`xcb`]: https://github.com/rtbo/rust-xcb
[`winapi`]: https://github.com/retep998/winapi-rs
