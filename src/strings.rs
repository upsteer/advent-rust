pub fn run(){

	// While writing code, if we know how long the string will be exactly,
	// like: a mongoid will be exactly 16 chars long, then `str` is the way to go

	/*
		In the case of a string literal, we know the contents at compile time, 
		so the text is hardcoded directly into the final executable.
	*/
	let string1 = "romit"; // This is fix length immutable string

	println!("{}", string1);


	// But in case we don't know how long or big the data is going to be prior to
	// compiling, then we use `String`, like user input or incoming log message
	// and pass it like so: String::from(log_message)
	let string2 = String::from("khanal"); // This is heap length also immutable;

	println!("{}", string2);

	// Transfering and taking Ownership
	// Since String is stored in heap; passing it to another function will transfer it's ownership
	pass_reference(string1);

	pass_reference(&string2);

	// This is valid, since we didn't transfer owenership to pass_reference because of the `&`
	println!("string1: {} and string2: {}", string1, string2);

	take_reference(&string2);
	
	take_reference(&string1.to_owned());

	println!("string1: {} and string2: {}", string1, string2);

	// take_ownership(string2);
	
	take_ownership(string2);
	
	take_ownership(string1.to_owned());

	// This does not work because string2 is transfered because of heap
	// But works for string1 because it string1 is not transfered but a temp `string1.to_owned() 
	// is passed to the function
	// println!("string1: {} and string2: {}", string1, string2);
}

fn pass_reference(firstname: &str){
	// Focus on the `&`; This function takes a reference of the strings, so they are read only
	println!("{}", firstname);
}

fn take_reference(firstname: &String){
	// Focus on the `&`; This function takes a reference of the strings, so they are read only
	println!("{}", firstname);
}

fn take_ownership(firstname: String){
	println!("taken owenership of: {}", firstname);
}