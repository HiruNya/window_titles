use xcb::{Connection as XConnection, xproto::{Atom, ATOM_WINDOW, get_property, intern_atom, Window}};

use crate::{ConnectionTrait, Result};

pub struct Connection {
	connection: XConnection,
	client_list: Atom,
	string: Atom,
	window_name: Atom,
}

impl ConnectionTrait for Connection {
	fn new() -> Result<Self> {
		let connection = XConnection::connect(None)?.0;
		let client_list = intern_atom(&connection, false, "_NET_CLIENT_LIST").get_reply()?.atom();
		let string = intern_atom(&connection, false, "UTF8_STRING").get_reply()?.atom();
		let window_name = intern_atom(&connection, false, "_NET_WM_NAME").get_reply()?.atom();
		Ok(Self { connection, client_list, string, window_name })
	}
	fn window_titles(&self) -> Result<Vec<String>> {
		let titles = self.connection.get_setup().roots()
			.map(|screen| screen.root())
			.map(|window| get_property(&self.connection, false, window, self.client_list, ATOM_WINDOW, 0, 1024))
			.filter_map(|cookie| cookie.get_reply().ok())
			.flat_map(|reply| reply.value().to_vec().into_iter())
			.filter_map(|window: Window| get_property(&self.connection, false, window, self.window_name, self.string, 0, 1024).get_reply().ok())
			.filter_map(|value| String::from_utf8(value.value().to_vec()).ok())
			.collect();
		Ok(titles)
	}
}
