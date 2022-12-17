use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::env;
use std::collections::HashMap;
use serde_json;
use serde::{Serialize, Deserialize};
use clap::{arg, Command};

fn cli() -> Command {
    Command::new("onion")
        .about("Add or remove environment layers.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(-f <ENV_FILE> "file for reading and writing layers"))
        .subcommand(
            Command::new("init")
                .about("Saves the current environment as the onion env base.")
        )
        .subcommand(
            Command::new("save")
                .about("Saves the current environment in a new layer.")
                .arg(arg!(<NAME> "the name under which the environment layer is saved"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("deactivate")
                .about("Deactivates the named layer.")
                .arg(arg!(<NAME> "the name for the environment layer to unload")),
        )
        .subcommand(
            Command::new("reactivate")
                .about("Reapplies the named layer.")
                .arg(arg!(<NAME> "the name for the environment layer to reload")),
        )
        .subcommand(
            Command::new("export")
                .about("Prints commands for setting the environment.")

        )
        .subcommand(
            Command::new("env")
                .about("Prints the environment colored according to the layers.")
        )
}

///////////////////////////////////////////////////////////////////////////////
//                                definitions                                //
///////////////////////////////////////////////////////////////////////////////

type Env = HashMap<String, String>;
// type EnvVarDiff = TextDiff<str>;

#[derive(Serialize, Deserialize, Debug)]
struct EnvLayer {
    is_active: bool,
    // diff: HashMap<String, EnvVarDiff>
}

#[derive(Serialize, Deserialize, Debug)]
struct EnvOnion {
    base: Env,
    layers: HashMap<String, EnvLayer>
}

///////////////////////////////////////////////////////////////////////////////
//                                   utils                                   //
///////////////////////////////////////////////////////////////////////////////

fn get_current_env() -> Env {
    let mut env = HashMap::new();
    for (key, value) in env::vars() {
        env.insert(key, value);
    }

    return env
}

fn read_onion(env_file_path: &Path) -> EnvOnion {
    let mut file = File::open(env_file_path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    return serde_json::from_str(&s).unwrap();
}

fn write_onion(onion: EnvOnion, env_file_path: &Path) {
    let mut file = File::create(env_file_path).unwrap();
    let serialized = serde_json::to_string(&onion).unwrap();
    writeln!(&mut file, "{}", serialized).unwrap();
}

///////////////////////////////////////////////////////////////////////////////
//                                  commands                                 //
///////////////////////////////////////////////////////////////////////////////

fn init(env_file_path: &Path) {
    let onion = EnvOnion {
        base: get_current_env(),
        layers: HashMap::new()
    };

    write_onion(onion, env_file_path);
}

fn save(env_file_path: &Path, name: &str) {
    let mut onion = read_onion(env_file_path);
    let new_layer = EnvLayer {
        is_active: true,
    };

    onion.layers.insert(name.to_string(), new_layer);
    write_onion(onion, env_file_path);
}

fn deactivate(env_file_path: &Path, name: &str) {
    let mut onion = read_onion(env_file_path);
    onion.layers.entry(name.to_string()).and_modify(|layer| layer.is_active = false);
    write_onion(onion, env_file_path);
}

fn reactivate(env_file_path: &Path, name: &str) {
    let mut onion = read_onion(env_file_path);
    onion.layers.entry(name.to_string()).and_modify(|layer| layer.is_active = true);
    write_onion(onion, env_file_path);
}

fn export(env_file_path: &Path) {
    let onion = read_onion(env_file_path);

    for (key, value) in onion.base {
        println!("{key}={value}");
    }
}

fn env(env_file_path: &Path) {
    let onion = read_onion(env_file_path);

    for (key, value) in onion.base {
        println!("{key}={value}");
    }
}

fn main() {
    let matches = cli().get_matches();

    let env_file_arg = matches.get_one::<String>("ENV_FILE").expect("required");
    let env_file_path = Path::new(env_file_arg);

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            init(env_file_path);
        }
        Some(("save", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            save(env_file_path, name);
        }
        Some(("deactivate", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            deactivate(env_file_path, name);
        }
        Some(("reactivate", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            reactivate(env_file_path, name);
        }
        Some(("env", _sub_matches)) => {
            env(env_file_path);
        }
        _ => unreachable!(),
    }
}
