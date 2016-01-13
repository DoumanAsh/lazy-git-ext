//! Common utils

#[macro_export]
macro_rules! cmd {
    (name=>$name:expr, $($arg:expr),*) => { std::process::Command::new($name)$(.arg($arg))* }
}

#[macro_export]
macro_rules! git {
    ($($arg:expr),*) => { cmd!(name=>"git", $($arg),*) }
}

pub use std::env::args as cmd_args;

#[inline]
///Get command line argument by position
///
///Starts from zero.
pub fn get_cmd_arg(num: usize) -> Option<String> {
    cmd_args().skip(num).next()
}

#[inline]
pub fn git_add_all() {
    git!("add", "--all").status().unwrap();
}
