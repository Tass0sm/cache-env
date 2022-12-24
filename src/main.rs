use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::collections::HashMap;
use serde_json;
use serde::{Serialize, Deserialize};
use clap::{arg, Command};

fn cli() -> Command<'static> {
    Command::new("cache-env")
        .about("save and restore environments")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(-f <ENV_FILE> "file for reading and writing layers"))
        .subcommand(
            Command::new("save")
                .about("Saves the current environment under NAME")
                .arg(arg!(<NAME> "the name under which the environment layer is saved"))
                .arg_required_else_help(true),
        )
	.subcommand(
            Command::new("print")
                .about("prints the saved environment under NAME")
                .arg(arg!(<NAME> "the name under which the environment layer is saved"))
                .arg_required_else_help(true),
        )
}

///////////////////////////////////////////////////////////////////////////////
//                                definitions                                //
///////////////////////////////////////////////////////////////////////////////

// #[derive(Deserialize, Serialize, Debug)]
// struct EnvVarDiffElement {
//     tag: ChangeTag,
//     old_index: Option<usize>,
//     new_index: Option<usize>,
//     value: String,
// }

// type Env = HashMap<String, String>;
// type EnvVarDiff = Vec<EnvVarDiffElement>;

#[derive(Deserialize, Serialize, Debug)]
struct Env {
    vars: HashMap<String, String>
}

type EnvCache = HashMap<String, Env>;

///////////////////////////////////////////////////////////////////////////////
//                                   utils                                   //
///////////////////////////////////////////////////////////////////////////////

fn get_current_env() -> Env {
    let mut var_map = HashMap::new();
    for (key, value) in env::vars() {
        var_map.insert(key, value);
    }

    let env = Env {
	vars: var_map
    };

    return env
}

fn read_env_cache(env_cache_path: &Path) -> EnvCache {
    let mut file = OpenOptions::new()
	.read(true)
	.write(true)
        .create(true)
        .open(env_cache_path)
        .unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    match serde_json::from_str(&s) {
        Ok(env_cache) => {
	    return env_cache;
        }
	_ => {
	    return HashMap::new();
	}
    }
}

fn write_env_cache(env_cache: EnvCache, env_cache_path: &Path) {
    let mut file = File::create(env_cache_path).unwrap();
    let serialized = serde_json::to_string_pretty(&env_cache).unwrap();
    writeln!(&mut file, "{}", serialized).unwrap();
}

///////////////////////////////////////////////////////////////////////////////
//                                  commands                                 //
///////////////////////////////////////////////////////////////////////////////

fn save(env_cache_path: &Path, name: &str) {
    let mut env_cache = read_env_cache(env_cache_path);
    env_cache.insert(name.to_string(), get_current_env());
    write_env_cache(env_cache, env_cache_path);
}

fn print(env_cache_path: &Path, name: &str) {
    let env_cache = read_env_cache(env_cache_path);
    let env = env_cache.get(name).unwrap();

    for (key, value) in &env.vars {
        println!("export {key}=\"{value}\"");
    }
}

fn main() {
    let matches = cli().get_matches();

    let env_file_arg = matches.get_one::<String>("ENV_FILE").expect("required");
    let env_file_path = Path::new(env_file_arg);

    match matches.subcommand() {
        Some(("save", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            save(env_file_path, name);
        }
        Some(("print", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            print(env_file_path, name);
        }
        _ => unreachable!(),
    }
}
