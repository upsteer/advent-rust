fn double(arg: i32) -> i32 {
	return arg+arg;
}

pub fn run(){

	//Default types to i32
	let a = 1;

	//Default types to f64
	let b = 2.5;

	//For longer than i32, we need to explicitly define them
	let c:i64 = 12347897827398;

	println!("{:?}", (a, b, c));
	
	assert_eq!(double(12), 24);
	assert_eq!(double(13), 25);

	println!("This won't print beacause of assertion error");
}
