use bytes::Viewer;

use clap::{Arg, App};

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_loop<T: Read>(viewer: &mut Viewer, data: &mut T) {
	let mut buffer: [u8; 1024] = [0; 1024];

	#[allow(while_true)]
	while true {
		let size = match data.read(&mut buffer) {
			Err(why) => panic!("read failed: {}", why),
			Ok(size) => size
		};

		if size == 0 {
			break;
		}

		viewer.draw(&buffer[0..size]);
	}
}

fn read_file(viewer: &mut Viewer, path: &Path) {
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file
	};

	read_loop(viewer, &mut file);
}

fn read_stdin(viewer: &mut Viewer) {
	let mut stdin = io::stdin();
	read_loop(viewer, &mut stdin);
}

fn main() {
	let mut app = App::new("bytes")
		.version("0.1.0")
		.author("David Westen <davidlwesten@protonmail.com")
		.arg(Arg::with_name("columns")
			.help("number of columns in output. minimum is 1")
			.default_value("10")
			.long("columns")
			.short("c"))
		.arg(Arg::with_name("width")
			.help("width of column. minimum is 4")
			.default_value("8")
			.long("width")
			.short("w"))
		.arg(Arg::with_name("FILE")
			.help("Input file")
			.required(false)
			.index(1));

	let matches = app.clone().get_matches();

	let columns = matches.value_of("columns").unwrap_or_default();
	let columns = columns.parse::<usize>().unwrap();
	if columns < 1 {
		if let Err(e) = app.print_long_help() {
			panic!("something terrible has happened: {}", e);
		}
		return;
	}

	let width = matches.value_of("width").unwrap_or_default();
	let width = width.parse::<usize>().unwrap();
	if width < 5 {
		if let Err(e) = app.print_long_help() {
			panic!("something terrible has happened: {}", e);
		}
		return;
	}

	let mut viewer = Viewer::new(columns, width);

	match matches.value_of("FILE") {
		Some(s) => read_file(&mut viewer, Path::new(s)),
		None => read_stdin(&mut viewer)
	};
}
