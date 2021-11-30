#[macro_use]
extern crate clap;

use std::process::exit;

fn main() {
    let matches = clap_app!(
        kvs =>
            (@setting DisableHelpSubcommand)
            (@setting SubcommandRequiredElseHelp)
            (@setting VersionlessSubcommands)
            (version: env!("CARGO_PKG_VERSION"))
            (author: env!("CARGO_PKG_AUTHORS"))
            (about: env!("CARGO_PKG_DESCRIPTION"))
            (@subcommand set =>
             (about: "Set the value of a string key to a string")
             (@arg KEY: +required "A string key")
             (@arg VALUE: +required "The string value of the key")
            )
            (@subcommand get =>
             (about: "Get the string value of a given string key")
             (@arg KEY: +required "A string key")
            )
            (@subcommand rm =>
             (about: "Remove a given key")
             (@arg KEY: +required "A string key")
            )
    )
    .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
