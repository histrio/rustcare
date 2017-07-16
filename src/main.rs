#![feature(slice_patterns)]
#![feature(libc)]

mod ptrace;

extern crate getopts;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::env;

use getopts::Options;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PID PATCH [options]", program);
    print!("{}", opts.usage(&brief));
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    // opts.optopt("p", "peek", "peek data", "PID");
    opts.optflag("h", "help", "output version information and exit");
    opts.optflag("v", "version", "output version information and exit");

    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Option parsing error: {}", f);
            return;
        }
    };



    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("version") {
        println!("{} version: {}", program, VERSION);
        return;
    }

    let free: Vec<_> = matches.free.iter().map(|x| &x[..]).collect();
    let (pid, patch_filename) = match &free[..] {
        &[k, v] => (k, v),
        _ => {
            println!("Not enough parameters");
            print_usage(&program, opts);
            return;
        }
    };

    match pid.parse::<i32>(){
        Ok(v) => ptrace::patch(v, patch_filename),
        Err(f) => {
            println!("Wrong PID");
            print_usage(&program, opts);
            return;
        }
    }
}
