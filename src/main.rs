use minigrepv2;
use std::env;





fn main() {
	let arguments: Vec<String> = env::args().collect();
	if arguments.len() < 3 {
		println!("write more arguments pls (filename, word)");
	} else {
		let filename = &arguments[1];
		let wordd = &arguments[2];
	let content = match minigrepv2::read(filename) {
		Ok(content) => content,
		Err(error) => {
			println!("{error}");
			return;
		},
	};
	minigrepv2::find_and_print(wordd, &content)
	}
 }
