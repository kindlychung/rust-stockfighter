extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
mod check_api;
mod request;
mod check_venue;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
  stockfighter checkapi
  stockfighter checkvenue <venue>
  stockfighter -h | --help
  stockfighter -v | --version

Options:
  -h, --help  Show this message
  -v, --version  Show the version of stockfighter
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_version: bool,
    cmd_checkapi: bool,
    cmd_checkvenue: bool,
    arg_venue: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("StockFighter v{}", VERSION)
    } else if args.cmd_checkapi {
        match check_api::check_api() {
            Ok(()) => println!("Api is Up"),
            Err(err) => println!("{}",err )
        };
    } else if args.cmd_checkvenue {
        match check_venue::check_venue(args.arg_venue) {
            Ok(val) => println!("Venue {} is up", val),
            Err(err) => println!("{}", err)
        }
    }
}
