use std::fs;



pub fn read(drct: &str) -> Result<String, Box<dyn std::error::Error>> {
	let output = fs::read_to_string(drct)?;
	Ok(output)
}


pub fn find<'a>(word: &'a str, content: &'a str) -> Vec<&'a str> {
	let content2 = content.lines();
	let mut vector2 = Vec::new();
	for line in content2 {
		if line.contains(word) {
			vector2.push(line);
		}
	}
	vector2
}



