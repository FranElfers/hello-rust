
fn arrays() {
	/*
	Every element of an array must have the same type
	Arrays in rust are different in some other languages,
	because they have a fixed length. Once declared, they
	cannot grow or shrink in size
	*/
	let months:String[] = ["January","February","March","April","May","June","July","August","September","October","November","December"];

	println!("first element: {}", months[months.len() - 1]);
}