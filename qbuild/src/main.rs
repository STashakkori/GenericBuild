use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
  let args: Vec<String> = env::args().collect();
	let mut args_list = Vec::new();
	
	if args.len() < 3 || args.len() > 5 {
			println!("=============================================================");
			println!("  Usage: qbuild  Out_bin_name  Compiler  SrcPath  [LIB_DIR] ");
			println!("                               | gcc              optional");
			println!("                               | clang");
			println!("=============================================================");
			return; 
	}
	
	let mut binname = "BIN_NAME=".to_string();
	binname.push_str(&args[1].trim().to_string());
	args_list.push(binname);
	
	match args[2].trim() {
		"gcc" => args_list.push("CC=gcc".to_string()),
		"clang" => args_list.push("CC=clang".to_string()),
		_ => {
			println!("Invalid compiler option!");
			return;
		}
	};  
	
	let path_check = Path::new(&args[3]);  
	if !path_check.exists() { 
		println!("Path does not exist.");
		return;
	}
	
	let mut srcpath = "SRC_PATH=".to_string();
	srcpath.push_str(&args[3].trim().to_string());
	args_list.push(srcpath);
	
	if args.len() == 5 {
		
		let path_check = Path::new(&args[4]);  
		if !path_check.exists() { 
			println!("Path does not exist.");
			return;
		}
		
		let mut libpath = "LIBDIR=".to_string();
		libpath.push_str(&args[4].trim().to_string());
		args_list.push(libpath);
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
	
	let outputty = match Command::new("make").args(args_list.clone()).output() {
    Ok(out) => out,
    Err(e) => { 
			println!("error executing make: {}", e);
			return;
		}
  };

	let stdout_str = String::from_utf8_lossy(&outputty.stdout);
  let stderr_str = String::from_utf8_lossy(&outputty.stderr);
  
	if stderr_str == "" {
		print!("{}", stdout_str);
	}
	else {
		print!("{}", stderr_str);
	}
}
