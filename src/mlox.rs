use crate::{error::ScannerError, parser::Parser, scanner::Scanner};
use std::io::Write;

pub struct Mlox {
    args: Vec<String>,
    error_handler: ScannerError,
}

impl Mlox {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            error_handler: ScannerError::default(),
        }
    }

    pub fn interpreter(&mut self) {
        let status = match self.args.len() - 1 {
            0 => self.run_prompt(),
            1 => self.run_file(),

            _ => 2,
        };

        if status > 0 {
            println!("Usage mlox [script]")
        };
    }

    fn run_file(&mut self) -> u8 {
        println!("Source file: {}", self.args[1]);
        let src = std::fs::read_to_string(&self.args[1]).expect("Cannot read: {file}");
        self.run(src);
        return 0;
    }

    fn run_prompt(&mut self) -> u8 {
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
                1..usize::MAX => self.run(buf),
                _ => break 1,
            }
        };
        println!();
        return status;
    }

    fn run(&mut self, src: String) {
        println!("src: {:?}", &src);
        let mut scanner = Scanner::new(&src, &mut self.error_handler);
        scanner.scan_tokens();
        let mut parser = Parser::new(scanner.tokens);
        if self.error_handler.had_err {
            todo!("There is a syntax error!");
        }
        parser.parse();
    }
}
