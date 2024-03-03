use clap::{arg, command, value_parser, ArgAction, Command};
use anyhow::Error;

use zerotierone_controller;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = command!()
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("status")
                .about("Print status info about this controller")
        )
        .subcommand(
            Command::new("list-networks")
                .about("List the networks of this controller")
                // .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("create-network")
                .about("List the networks of this controller")
                .arg(arg!([network_id] "Network id, if left out, it will be generated automatically"))
        )
        .get_matches();

    if matches.subcommand_matches("status").is_some() {
        let status = zerotierone_controller::get_status().await?;
        let config = status.config.clone().unwrap();

        println!("Version: {}", status.version.clone().unwrap());
        println!("Address: {}", status.address.clone().unwrap());
        println!("Port: {}", config.settings.unwrap().primary_port.unwrap());
    }

    if matches.subcommand_matches("list-networks").is_some() {
        let networks = zerotierone_controller::get_networks().await?;

        if networks.len() == 0 {
            println!("There are no networks");
        } else {
            println!("{:?}", networks);
        }
    }

    if let Some(matche) = matches.subcommand_matches("create-network") {
        println!("create");
        let network_id = matche.get_one::<String>("network_id");
        zerotierone_controller::generate_new_network(network_id.cloned()).await?;
    }

    Ok(())
}
