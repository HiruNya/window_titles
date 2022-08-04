use xcb::{Connection as XConnection, x::{self, Atom}};
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
		
		let client_list = connection.send_request(&x::InternAtom {
			only_if_exists: false,
			name: "_NET_CLIENT_LIST".as_bytes(),
		});
		let client_list = connection.wait_for_reply(client_list)?.atom();
		
		let string = connection.send_request(&x::InternAtom {
			only_if_exists: false,
			name: "UTF8_STRING".as_bytes(),
		});
		let string = connection.wait_for_reply(string)?.atom();
		
		let window_name = connection.send_request(&x::InternAtom {
			only_if_exists: false,
			name: "_NET_WM_NAME".as_bytes(),
		});
		let window_name = connection.wait_for_reply(window_name)?.atom();
		
		Ok(Self { connection, client_list, string, window_name })
	}
	fn window_titles(&self) -> Result<Vec<String>> {
		let titles = self.connection.get_setup().roots()
			.map(|screen| screen.root())
		    .map(|window| self.connection.send_request(&x::GetProperty {
		        delete: false,
		        window,
		        property: self.client_list,
		        r#type: x::ATOM_NONE,
		        long_offset: 0,
		        long_length: 1024})
		     )
		    .filter_map(|cookie| self.connection.wait_for_reply(cookie).ok())
		    .flat_map(|reply| reply.value().to_vec().into_iter())
		    .filter_map(|window| {
		        let c = self.connection.send_request(&x::GetProperty {
		            delete: false,
		            window,
		            property: self.window_name.to_owned(),
		            r#type: self.string.to_owned(),
		            long_offset: 0,
		            long_length: 1024});
		        self.connection.wait_for_reply(c).ok()
		    })
		    .filter_map(|value| String::from_utf8(value.value().to_vec()).ok())
		    .collect();
		Ok(titles)
	}
}
