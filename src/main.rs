extern crate gpgme;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::process::exit;

use gpgme::{Context, Protocol};

const Key: &'static str = "your gpg key here";

fn main() {
	let args : Vec<String> = env::args().collect();
	
	if args[1] == "help" || args[1] == "h" || args[1] == "info" {
		program_info();
		exit(1);
	}

	if args.len() < 3 {
		exit(1);
	}
	
	if args[1] == "read" || args[1] == "r" {
		let output = decrypt_file(&args[2]);
		println!("{}", output);
	}	
	else if args[1] == "new" || args[1] == "n"{
		let un = prompt_info(String::from("Username:"));
		let pw = prompt_info(String::from("Password:"));
		temp_file(un, pw);
		encrypt_file(&args[2]);
	}
	else {
		program_info();
		exit(1);
	}
}

fn prompt_info(x: String) -> String {
	let mut info = String::new();

	println!("{}", x);
	io::stdin().read_line(&mut info)
		.expect("failed to read input");

	info
}


fn temp_file(username: String, password: String) {
	let path = String::from("/tmp/pwman00");
	let mut file = File::create(path).
		expect("error creating temp");

	file.write_all(username.as_bytes());
	file.write_all(password.as_bytes());
}

fn encrypt_file(file: &String) {
	let mut ctx = Context::from_protocol(Protocol::OpenPgp).unwrap();
	ctx.set_armor(true);
	let mut recipients: Vec<String> = Vec::new();
	recipients.push(String::from(Key));
	let mut input = File::open("/tmp/pwman00").unwrap();
	let mut output = Vec::new();

	let keys = if !recipients.is_empty() {
        ctx.find_keys(recipients)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|k| k.can_encrypt())
            .collect()
    } else {
        Vec::new()
    };

	ctx.encrypt(&keys, &mut input, &mut output)
		.expect("encrypting failed");
	
	let mut f = File::create(&file)
		.expect("error creating file");

	f.write_all(&output);
	fs::remove_file("/tmp/pwman00");
}


fn decrypt_file(file: &String) -> String {
	let mut ctx = Context::from_protocol(Protocol::OpenPgp).unwrap();
	let mut input = File::open(&file).unwrap();
	let mut output = Vec::new();
        
	ctx.decrypt(&mut input, &mut output)
		.expect("decrypt failed");

	println!("[decrypted]: {}\n", file);
	let output_string = String::from_utf8(output).unwrap();
	output_string
}

fn program_info() {
	println!("a password-manager");
	println!("apwman [command] FILE");
	println!("commands::");
	println!("h : help, program info");
	println!("r : read, decrypt and print file content");
	println!("n : new, create and encrypt password file");
}
