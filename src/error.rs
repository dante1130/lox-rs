static mut HAD_ERROR: bool = false;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn had_error() -> bool {
    unsafe { HAD_ERROR }
}

pub fn reset_error() {
    unsafe { HAD_ERROR = false };
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, location, message);
    unsafe { HAD_ERROR = true };
}
