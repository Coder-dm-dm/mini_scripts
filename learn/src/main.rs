use std::io;

fn main() {
	loop{
		let data_in = input("Celsius to Kelvin: input a number".to_string()).to_string();
		//.trim() is extremly imporant to allow parse()
		let data_process: f32 = data_in.trim().parse().expect("Failed to parse integer");

		let data_out: f32 = {
			data_process + 273.15
		};

		println!("{data_out} K");
	}



}


fn input(x: String) -> String {
// u must use (.to_string()) to make sure input works properly./n
	println!("{}", x);

	let mut ret = String::new();
	io::stdin()
		.read_line(&mut ret)
		.expect("Failed to read line");

	return ret;


}