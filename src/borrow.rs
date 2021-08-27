pub fn run(){
	let mut s = String::from("Hello Romit!");

	let word = first_word(&s); //Gives ownership to first_word
	let first = word.to_owned();
	s.clear();

	println!("here is the first word: {}", first);
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate(){
		if item == b' '{
			return &s[0..i]; // Returned pointer
		}
	}
	&s[..]
}