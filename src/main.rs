use std::ffi::OsString;
use std::io;
use gethostname::gethostname;
use pyo3::prelude::*;

const LOGO: &str = r"
   |\---/|
   | ,_, |
    \_`_/-..----.
 ___/ `   ' ,''+  \
(__...'   __\     |`.___.';
  (_,...'(_,.`__)/'.....+

";
const NAME: &str = "Sleepy Cat";
const PROMPT: &str = ("┌─[{user}\033[1;33m@\033[0;36m\033[31m]─[{dir}]\n└──╼ $\033[1;37m");



fn main() {
    println!("{LOGO}");
    println!("Welcome to Sleepy Cat {:?}", user);
}