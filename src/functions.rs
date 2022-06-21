/*
A function is a block of organized, reusable code that is
used to perform a signle related action. Functions provide
better modularity for your application and a high degree of
code reusing. */
/*
Rust code uses snake_case as the conventional style
for function and variable names. */

fn main() {
	println!("hello world");
	another_function(sum(1, 4));
}

fn another_function(x:i32) {
	println!("Number: {}", x);
}

fn sum(x:i32, y:i32) -> i32 {
	x + y
}