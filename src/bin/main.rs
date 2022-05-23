use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use filler::ThreadPool;
use filler::arena::Arena;
use regex::Regex;
use json;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	let (status_line, contents) = dispatch(buffer);
	
	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		status_line,
		contents.len(),
		contents
	);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn dispatch(buffer: [u8; 1024]) -> (&'static str, String) {
	let get = b"GET / HTTP/1.1\r\n";
	let getjs = b"GET /js/index.js HTTP/1.1\r\n";
	let getcss = b"GET /filler.css HTTP/1.1\r\n";
	let players = b"GET /players HTTP/1.1\r\n";
    let run = Regex::new(r"^GET /run\?p1=\w+.filler&p2=\w+.filler HTTP/1.1").unwrap();
	let (status_line, filename);

	if buffer.starts_with(get) {
		(status_line, filename) = ("HTTP/1.1 200 OK", "public/index.html");
	}
	else if buffer.starts_with(getcss) {
		(status_line, filename) = ("HTTP/1.1 200 OK", "public/filler.css");
	}
	else if buffer.starts_with(getjs) {
		(status_line, filename) = ("HTTP/1.1 200 OK", "public/js/index.js");
	}
    else if buffer.starts_with(players) {
		return ("HTTP/1.1 200 OK", get_players_json())
    }
    else if run.is_match(std::str::from_utf8(&buffer).unwrap()) {
        let path = "./assets/players/";
        let re = Regex::new(r"^GET /run\?p1=(\w+.filler)&p2=(\w+.filler) HTTP/1.1").unwrap();
        let caps = re.captures(std::str::from_utf8(&buffer).unwrap()).unwrap();
        let p1 = format!("{}{}", path, caps.get(1).map_or("", |m| m.as_str()));
        let p2 = format!("{}{}", path, caps.get(2).map_or("", |m| m.as_str()));
		let contents = Arena::run("./assets/filler_vm",
			    &mut ["-f", "assets/map02",
				"-p1", p1.as_str(),
				"-p2", p2.as_str()]);
		return ("HTTP/1.1 200 OK", contents.get_replay().to_string())
	}
	else {
		(status_line, filename) = ("HTTP/1.1 404 NOT FOUND", "public/404.html");
	}

	let contents = fs::read_to_string(filename).unwrap();
	(status_line, contents)
}

fn get_players_json() -> String {
    let paths = fs::read_dir("./assets/players/").unwrap();
    let names = paths.map(|entry| {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let file_name_str = file_name.to_str().unwrap();
        let file_name_string = String::from(file_name_str);
        file_name_string
    }).collect::<Vec<String>>();
    json::stringify(names)
}
