// typer.rs -- Stick
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

extern crate stick;
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::{ fmt::{ Display, Formatter, Error }, io::{ Write, stdout, stdin } };
use stick::{ Input::*, Button };

/// The set of characters determined by the move stick.
#[derive(PartialEq)]
enum Set {
	None,
	Abcd,
	Efgh,
	Ijkl,
	Mnop,
	Qrst,
	Uvwx,
	YzAccents,
	Accents,
}

impl Display for Set {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&Set::None => write!(f, "(A)Autocomplete, (B)Newline, (X)Space, (Y)Backspace"),
			&Set::Abcd => write!(f, "(A)A, (B)B, (X)C, (Y)D"),
			&Set::Efgh => write!(f, "(A)E, (B)F, (X)G, (Y)H"),
			&Set::Ijkl => write!(f, "(A)I, (B)J, (X)K, (Y)L"),
			&Set::Mnop => write!(f, "(A)M, (B)N, (X)O, (Y)P"),
			&Set::Qrst => write!(f, "(A)Q, (B)R, (X)S, (Y)T"),
			&Set::Uvwx => write!(f, "(A)U, (B)V, (X)W, (Y)X"),
			&Set::YzAccents => write!(f, "(A)Y, (B)Z, (X)[Ñ], (Y)[Á]"),
			&Set::Accents => write!(f, "(A)Å, (B)Ā, (X)À, (Y)Ä"),
		}
	}
}

fn setupdate(set: &mut Set, newset: Set) -> bool {
	let mut update = false;
	if *set != newset {
		update = true;
	}
	*set = newset;
	update
}

fn main() {
	let mut cm = stick::ControllerManager::new(vec![]);
	let mut set = Set::None;
	let mut update = false;
	let mut string = String::new();

	// Get the standard input stream.
	let stdin = stdin();
	// Get the standard output stream and go to raw mode.
	let mut stdout = stdout().into_raw_mode().unwrap();

	write!(stdout, "{}{}{}{}",
		// Clear screen.
		termion::clear::All,
		// Move cursor to first row, first column.
		termion::cursor::Goto(1, 1), set, termion::cursor::Hide
	).unwrap();

	// Update terminal screen.
	stdout.flush().unwrap();

	// Main loop
	'a: loop {
		while let Some((_, i)) = cm.update() {
			match i {
				Move(x, y) => {
					if y < -0.5 {
						if x > 0.5 {
							update = setupdate(
								&mut set,
								Set::Efgh);
						} else if x < -0.5 {
							update = setupdate(
								&mut set,
								Set::Accents);
						} else {
							update = setupdate(
								&mut set,
								Set::Abcd);
						}
					} else if y > 0.5 {
						if x > 0.5 {
							update = setupdate(
								&mut set,
								Set::Mnop);
						} else if x < -0.5 {
							update = setupdate(
								&mut set,
								Set::Uvwx);
						} else {
							update = setupdate(
								&mut set,
								Set::Qrst);
						}
					} else if x < -0.5 {
						update = setupdate(&mut set,
							Set::YzAccents);
					} else if x > 0.5 {
						update = setupdate(&mut set,
							Set::Ijkl);
					} else {
						update = setupdate(&mut set,
							Set::None);
					}
				}
				Camera(x, y) => {
				}
				Press(b) => {
					match b {
						Button::Menu =>  {
							break 'a;
						}
						Button::Accept => {
							update = true;
							match set {
								Set::None => {}, // Autocomplete
								Set::Abcd => string.push('A'),
								Set::Efgh => string.push('E'),
								Set::Ijkl => string.push('I'),
								Set::Mnop => string.push('M'),
								Set::Qrst => string.push('Q'),
								Set::Uvwx => string.push('U'),
								Set::YzAccents => string.push('Y'),
								Set::Accents => {}, // Å modifier
							}
						},
						Button::Cancel => {
							update = true;
							match set {
								Set::None => string.push('\n'),
								Set::Abcd => string.push('B'),
								Set::Efgh => string.push('F'),
								Set::Ijkl => string.push('J'),
								Set::Mnop => string.push('N'),
								Set::Qrst => string.push('R'),
								Set::Uvwx => string.push('V'),
								Set::YzAccents => string.push('Z'),
								Set::Accents => {}, // modifier
							}
						},
						Button::Execute => {
							update = true;
							match set {
								Set::None => string.push(' '),
								Set::Abcd => string.push('C'),
								Set::Efgh => string.push('G'),
								Set::Ijkl => string.push('K'),
								Set::Mnop => string.push('O'),
								Set::Qrst => string.push('S'),
								Set::Uvwx => string.push('W'),
								Set::YzAccents => {}, // modifier
								Set::Accents => {}, // modifier
							}
						},
						Button::Action => {
							update = true;
							match set {
								Set::None => { string.pop(); },
								Set::Abcd => string.push('D'),
								Set::Efgh => string.push('H'),
								Set::Ijkl => string.push('L'),
								Set::Mnop => string.push('P'),
								Set::Qrst => string.push('T'),
								Set::Uvwx => string.push('X'),
								Set::YzAccents => {}, // modifier
								Set::Accents => {}, // modifier
							}
						},
						_ => {}
					}
				}
				Release(b) => {
				}
				_ => {}
			}
		}

		if update {
			// Write first line.
			write!(stdout, "{}{}{}", termion::cursor::Goto(1, 1),
				termion::clear::CurrentLine, set).unwrap();

			// Write text
			write!(stdout, "{}{}{}", termion::cursor::Goto(1, 2),
				termion::clear::CurrentLine, string).unwrap();

			// Update terminal screen.
			stdout.flush().unwrap();

			update = false;
		}
	}

	// Unhide Cursor
	write!(stdout, "{}", termion::cursor::Show).unwrap();
}
