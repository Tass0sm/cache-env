use clap::{arg, Command};

fn cli() -> Command {
    Command::new("onion")
        .about("Add or remove environment layers.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("save")
                .about("Saves the current environment in a new layer.")
                .arg(arg!(<NAME> "the name under which the environment layer is saved"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("unload")
                .about("Removes the named layer.")
                .arg(arg!(<NAME> "the name for the environment layer to unload")),
        )
        .subcommand(
            Command::new("reload")
                .about("Reapplies the named layer.")
                .arg(arg!(<NAME> "the name for the environment layer to reload")),
        )
}

fn main() {
   let matches = cli().get_matches();

   match matches.subcommand() {
       Some(("save", sub_matches)) => {
           println!(
               "Save {}",
               sub_matches.get_one::<String>("NAME").expect("required")
           );
       }
       Some(("unload", sub_matches)) => {
           println!(
               "Unload {}",
               sub_matches.get_one::<String>("NAME").expect("required")
           );
       }
       Some(("reload", sub_matches)) => {
           println!(
               "Reload {}",
               sub_matches.get_one::<String>("NAME").expect("required")
           );
       }
       _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
   }
}