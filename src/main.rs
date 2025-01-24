use std::{env, io::Write};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let status = match args.len() - 1 {
        0 => run_prompt(),
        1 => run_file(&args[1]),
        _ => 2,
    };
    if status > 0 {
        println!("Usage mlox [script]")
    };
}

fn run_file(file: &str) -> u8 {
    println!("Source file: {file}");
    let src = std::fs::read_to_string(file).expect("Cannot read: {file}");
    run(src);
    return 0;
}

fn run_prompt() -> u8 {
    println!("Prompt is running");
    let status = loop {
        let mut buf = String::new();
        print!(">");
        let _ = std::io::stdout().flush();
        let n = std::io::stdin()
            .read_line(&mut buf)
            .expect("Cannot read stdin.");
        match n {
            0 => break 0, // EOF
            1..usize::MAX => run(buf),
            _ => break 1,
        }
    };
    println!();
    return status;
}

fn run(src: String) {
    println!("src: {src}");
}
