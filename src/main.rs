use std::ffi::OsString;
use std::io;
use gethostname::gethostname;
use pyo3::prelude::*;
use std::time::SystemTime;

const LOGO: &str = r"
   |\---/|
   | ,_, |
    \_`_/-..----.
 ___/ `   ' ,''+  \
(__...'   __\     |`.___.';
  (_,...'(_,.`__)/'.....+

";
const NAME: &str = "Sleepy Cat";

fn prompt(tool: &str) -> String {
    let time = SystemTime::now();
    let mut command = String::new();
    println!("({:?})-({})\n({})-> ", time, tool, whoami::username());
    std::io::stdin().read_line(&mut line).unwrap();
    return command;
}

fn main() {
    println!("{LOGO}");
    println!("Welcome to Sleepy Cat {:?}", whoami::realname());

    println!("{}", whoami::distro());
    println!("{}", whoami::devicename());
    println!("{}", whoami::hostname());

}