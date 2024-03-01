pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	println!("Executing code in sequential.");

	// Execution: Sequential
	for Entry in Entry {
		if let Some(Last) = Entry.last() {
			if *Last == Pattern {
				let Directory = &Entry[0..Entry.len() - 1].join(&Separator.to_string());

				let mut Out = match cfg!(target_os = "windows") {
					true => Command::new("cmd")
						.args(["/C", &Command])
						.current_dir(Directory)
						.stdout(Stdio::piped())
						.spawn()
						.expect("Failed to execute process."),
					false => Command::new("sh")
						.arg("-c")
						.current_dir(Directory)
						.arg(Command.clone())
						.stdout(Stdio::piped())
						.spawn()
						.expect("Failed to execute process."),
				}
				.stdout
				.expect("Failed to get stdout handle");

				let mut Output = String::new();

				loop {
					let mut Buffer = [0; 512];
					let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");

					if Byte == 0 {
						break;
					}

					Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
				}

				println!("{}", Output);
			}
		}
	}
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use std::{
	io::Read,
	process::{Command, Stdio},
};