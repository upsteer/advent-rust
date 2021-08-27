pub fn name_and_age(){
	let name = "Romit";

	let mut age = 25;
	
	age+=1;
	
	println!("{} of {}", name, age);

	let (my_name, my_age) = ("Romit", 25);

	println!("{0} is my name, {1} is my age", my_name, my_age);
}