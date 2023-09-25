use std::path::Path;
use std::process::Command;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut args_list = Vec::new();

	if args.len() == 1 || args[1] == "-h" {
		usage();
		return;
	}
	else if args.len() > 5 {
		println!("Expected fewer arguments. Found {}",args.len());
		usage();
		return;
	}
	else if args.len() < 4 {
		println!("Expected more arguments. Found {}",args.len());
		usage();
		return;
	}

	if args[2].trim() != "g++" && args[2].trim() != "clang++" {
		println!("Unrecognized compiler : {}",args[2].trim());
		usage();
		return;
	}


	let bin_name = format!("BIN_NAME:={}",args[1].trim());
	args_list.push(bin_name);
	let src_path = format!("SRC_PATH= {}",args[3].trim());
	args_list.push(src_path);
	let comp = format!("CC= {}",args[2].trim());
	args_list.push(comp);


	let mut lib = String::from("LIBDIR=");
	if args.len() == 5 {
		lib.push_str(args[4].trim());
	}
	args_list.push(lib);


	let path = Path::new(args[3].trim());
	if !path.exists() {
		println!("Unable to locate source path.");
		return;
	}

	let mut i = 0;
	for arg in args_list.clone() {
		let mut tmp = arg.clone();
		let last = match tmp.pop() {
			Some(c) => c,
			None => continue
		};
		if last != '/' {
			tmp.push(last);
		}
		args_list[i] = tmp;
		i += 1;
	}

	match Command::new("make")
		.args(args_list.clone())
		.status() {
			Ok(stat) => stat,
				Err(err) => {
					println!("Unable to execute make command. Error: {}",err);
					return;
				}
		};
}

fn usage() {
	println!("Usage: <bin_name> <compiler> <src_path> [lib_str]");
	println!("Compilers: g++ or clang++");
}
