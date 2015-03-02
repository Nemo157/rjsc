#![feature(box_syntax)]
#![feature(old_path)]
#![feature(rustc_private)]
#![feature(exit_status)]

extern crate getopts;
extern crate rustc;
extern crate rustc_driver;
extern crate syntax;

mod calls;
mod visitor;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut calls::RjscCompilerCalls::new());
    std::env::set_exit_status(0);
}
