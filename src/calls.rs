use getopts;
use visitor;

use rustc::session::Session;
use rustc::session::config::{ self, Input };
use rustc_driver::{ driver, CompilerCalls, Compilation, RustcDefaultCalls };

use syntax::{ diagnostics, visit };

pub struct RjscCompilerCalls {
    default_calls: RustcDefaultCalls,
}

impl RjscCompilerCalls {
    pub fn new() -> RjscCompilerCalls {
        RjscCompilerCalls { default_calls: RustcDefaultCalls }
    }
}

impl<'a> CompilerCalls<'a> for RjscCompilerCalls {
    fn early_callback(&mut self, _: &getopts::Matches, _: &diagnostics::registry::Registry) -> Compilation {
        Compilation::Continue
    }

    fn late_callback(&mut self, m: &getopts::Matches, s: &Session, i: &Input, odir: &Option<Path>, ofile: &Option<Path>) -> Compilation {
        self.default_calls.late_callback(m, s, i, odir, ofile);
        Compilation::Continue
    }

    fn some_input(&mut self, input: Input, input_path: Option<Path>) -> (Input, Option<Path>) {
        (input, input_path)
    }

    fn no_input(&mut self, m: &getopts::Matches, o: &config::Options, odir: &Option<Path>, ofile: &Option<Path>, r: &diagnostics::registry::Registry) -> Option<(Input, Option<Path>)> {
        self.default_calls.no_input(m, o, odir, ofile, r);
        panic!("No input supplied to rjsc");
    }

    fn build_controller(&mut self, _: &Session) -> driver::CompileController<'a> {
        let mut control = driver::CompileController::basic();

        control.after_analysis.stop = Compilation::Stop;
        control.after_analysis.callback = box |state| {
            let krate = state.expanded_crate.unwrap();

            let mut visitor = visitor::RjscVisitor::new();
            visit::walk_crate(&mut visitor, krate);
            if visitor.seen_main {
                println!("main();");
            }
        };

        control
    }
}

