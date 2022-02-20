use std::io::Read;
use std::process::{Child, Command, Stdio};

pub(crate) fn spawn_process(program: &String, input: &String) -> Child {
	match Command::new(program)
		.arg(input)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn() {
		Err(why) => panic!("Process {} error: {}", program,  why),
		Ok(process) => process,
	}
}

pub(crate) fn process_output(process: &mut Child, name: &String) -> String {
	let mut output = String::new();
	match process.stdout.take().unwrap().read_to_string(&mut output) {
		Err(why) => panic!("couldn't read [{}]{} stdout: {}", process.id(), name, why),
		Ok(_) => print!("[{}]{} responded with:\n{}", process.id(), name, output),
	}
	output
}
