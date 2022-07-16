use std::process::Command;

fn main() {
    let pwd = Command::new("pwd").output().expect("cannot execute pwd");

    let mut curr_dir = String::from_utf8(pwd.stdout).expect("cannot parse stdout");
    curr_dir.pop();
    Command::new("alacritty")
        .arg("--working-directory")
        .arg(curr_dir)
        .spawn()
        .expect("cannot execute alacritty");
}
