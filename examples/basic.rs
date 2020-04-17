use window_titles::Connection;

fn main() {
	let connection = Connection::new().unwrap();
	connection.window_titles().into_iter()
		.for_each(|title| println!("{}", title))
}
