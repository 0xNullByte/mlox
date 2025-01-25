#[derive(Default)]
pub struct MloxError {
    pub had_err: bool,
}

impl MloxError {
    pub fn send(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, where_: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_, message);
        self.had_err = true;
    }
}
