#[macro_export]
macro_rules! sh {
    ($cmd:expr) => {
        std::process::Command::new("sh")
            .arg("-c")
            .arg($cmd)
            .status()
            .expect("Failed to execute command")
            .code()
            .unwrap_or(-1);
    }
}
