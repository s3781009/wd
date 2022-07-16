use std::process::Command;

fn main() {
    let pwd = Command::new("pwd").output().expect("cannot pwd");

    let mut arg = String::from_utf8(pwd.stdout).unwrap();
    arg.pop();
    Command::new("alacritty")
        .arg("--working-directory")
        .arg(arg)
        .spawn();
}
