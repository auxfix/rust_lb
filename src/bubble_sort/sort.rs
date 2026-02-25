use std::io;


fn main() {
	println!("Please enter length of the array, you want to sort: ");	

	let mut user_input = String::new();
	io::stdin()
	    .read_line(&mut user_input)
	    .expect("Could nor read your data, i mean for real");

	let n:i32  = user_input.trim().parse().expect("Input not an integer");

	println! ("We willl be sorting array of size:  {}", n);


	for _i in 0..n {
		println!("Enter {} element: ", _i);
	}
}
