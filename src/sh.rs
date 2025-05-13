pub fn sh(command: &str, args: &str) -> String {
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c")
        .arg(format!("{} {}", command, args))
        .stderr(std::process::Stdio::inherit());

    String::from_utf8(cmd.output().unwrap().stdout).unwrap()
}
