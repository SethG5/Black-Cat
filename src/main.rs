use std::ffi::OsString;
use std::io;
use pyo3::prelude::*;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

const LOGO: &str = r"
   |\---/|
   | •_• |
    \_`_/-..----.
 ___/ `   ' ,''+  \
(__...'   __\     |`.___.';
  (_,...'(_,.`__)/'.....+

";
const NAME: &str = "Sleepy Cat";

fn prompt(tool: &str) -> String {
    let mut command = String::new();
    println!("({:?})-({})\n({})-> ", Utc::now(), tool, whoami::username());
    std::io::stdin().read_line(&mut command).unwrap();
    return command;
}

fn main() {
    println!("{LOGO}");
    println!("Welcome to Sleepy Cat {:?}", whoami::realname());

    println!("{}", whoami::distro());
    println!("{}", whoami::username());
    println!("{}", whoami::hostname());
    println!();

    loop{
        prompt("");
    }

}