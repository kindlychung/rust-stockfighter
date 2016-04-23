extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use docopt::Docopt;
mod check_api;
mod request;
mod check_venue;

macro_rules! cargo_pkg_name {
    () => {{
        env!("CARGO_PKG_NAME")
    }}
}
macro_rules! cargo_manifest_dir {
    () => {{
        env!("CARGO_MANIFEST_DIR")
    }}
}
macro_rules! cargo_pkg_version_major {
    () => {{
        env!("CARGO_PKG_VERSION_MAJOR")
    }}
}
macro_rules! cargo_pkg_version_minor {
    () => {{
        env!("CARGO_PKG_VERSION_MINOR")
    }}
}
macro_rules! cargo_pkg_version_patch {
    () => {{
        env!("CARGO_PKG_VERSION_PATCH")
    }}
}
macro_rules! cargo_pkg_version_pre {
    () => {{
        env!("CARGO_PKG_VERSION_PRE")
    }}
}
macro_rules! cargo_pkg_version {
    () => {{
        env!("CARGO_PKG_VERSION")
    }}
}



const USAGE: &'static str = "
Usage:
  stockfighter checkapi
  stockfighter checkvenue <venue>
  stockfighter -h | --help
  stockfighter -v | --version

Options:
  -h, --help          Show this message
  -v, --version       Show the version of stockfighter
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

    let mut args: Args = Docopt::new(USAGE)
                             .unwrap()
                             .version(Some(cargo_pkg_name!().to_string() + " version: " +
                                           cargo_pkg_version!()))
                             .decode()
                             .unwrap_or_else(|e| e.exit());

    if args.cmd_checkapi {
        let check_api = check_api::check_api();

        if check_api.ok {
            println!("StockFighter API is up!");
        } else {
            println!("StockFighter API is down. Error is {}", check_api.error)
        }
    } else if args.cmd_checkvenue {
        let venue = args.arg_venue.to_string();
        let check_venue = check_venue::check_venue(args.arg_venue);

        if check_venue.ok {
            println!("Venue {} is up", venue);
        } else {
            println!("Unable to check venue. Error is {}", check_venue.error);
        }
    }
}
