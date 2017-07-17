#![feature(slice_patterns)]
#![feature(libc)]

mod ptrace;

extern crate getopts;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::env;

use getopts::Options;
use std::io;
use std::io::Write;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATCH [options]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(Debug)]
enum ParseErr {
    OptionsParseError,
    PIDInputError,
    NotEnoughParametersError,
    PIDParseError,
    DoNothing
}

fn parse_args(opts: &Options, args: &Vec<String>) -> Result<(i32, String), ParseErr> {

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            println!("Option parsing error: {}", f);
            return Err(ParseErr::OptionsParseError);
        }
    };

    if matches.opt_present("help") {
        return Err(ParseErr::DoNothing);
    }

    if matches.opt_present("version") {
        println!("Version: {:?}", VERSION);
        return Err(ParseErr::DoNothing);
    }


    let mut pid = String::new();
    pid.clear();
    print!("Process PID: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut pid) {
        Ok(_) => println!("{}", pid),
        Err(error) => {
            println!("PID input error: {}", error);
            return Err(ParseErr::PIDInputError);
        }
    }

    let free: Vec<_> = matches.free.iter().map(|x| &x[..]).collect();
    let patch_filename = match &free[..] {
        &[k] => k,
        _ => {
            println!("Not enough parameters");
            return Err(ParseErr::NotEnoughParametersError);
        }
    };

    match pid.trim().parse::<i32>(){
        Ok(v) => Ok((v, patch_filename.to_string())),
        Err(_) => {
            return Err(ParseErr::PIDParseError);
        }
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("p", "pid", "process PID", "PID");
    opts.optflag("h", "help", "output version information and exit");
    opts.optflag("v", "version", "output version information and exit");

    let program = args[0].clone();

    match parse_args(&opts, &args){
        Err(f) => {
            println!("Error {:?}", f);
            print_usage(&program, opts);
            return;
        }
        Ok((pid, patch_filename)) => ptrace::patch(pid, &patch_filename)
    }
}
