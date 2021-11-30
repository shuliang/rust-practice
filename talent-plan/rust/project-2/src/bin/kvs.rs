#[macro_use]
extern crate clap;

use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
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
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_owned(), value.to_owned())?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
