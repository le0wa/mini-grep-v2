use std::fs;



pub fn read(drct: &str) -> Result<String, Box<dyn std::error::Error>> {
	let output = fs::read_to_string(drct)?;
	Ok(output)
}


pub fn find_and_print(word: &str, content: &str) {
	let content2 = content.lines();
	for line in content2 {
		if line.contains(word) {
			println!("{line}");
		}
	}
}



