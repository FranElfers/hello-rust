/*
A tuple is a general way of grouping together some number
of other values with a variety of types into one compound type.
*/
fn main() {
	let tup: (i32, f64, u8) = (500, 3.5, 1);

	let (x,y,z) = tup;

	let a = tup.0;

	println!("X: {} Y: {} Z: {}", x,y,z);
	println!("A: {}", a);
}