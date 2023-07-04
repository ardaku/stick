use std::env;

mod sdb;

fn print_help() {
    eprintln!("Tasks:");
    eprintln!();
    eprintln!("--help          Print this help text");
    eprintln!("sdb             Generate stick & gcdb bytecode databases");
}

fn print_unknown(x: &str) {
    eprintln!("cargo xtask {} is an invalid command.", x);
    eprintln!();
    eprintln!("Run `cargo xtask` for help page.");
}

fn sdb() {
    sdb::main()
}

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("sdb") => sdb(),
        None | Some("--help") => print_help(),
        Some(x) => print_unknown(x),
    }
}
