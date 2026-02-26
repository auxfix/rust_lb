use std::io;


fn main() {
	println!("Please enter array you whant us to sort, split characters by comma: ");	

	let mut user_input = String::new();
	io::stdin()
	    .read_line(&mut user_input)
	    .expect("Could nor read your data, i mean for real");

	let array_parts  = user_input.split(",");

	println! ("Here is your array:",);

    array_parts.sort();

	for _i in array_parts {
		print!("{} ", _i);
	}
}
