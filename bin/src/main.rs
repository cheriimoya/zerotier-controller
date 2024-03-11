use clap::{arg, command, value_parser, ArgAction, Command};
use anyhow::Error;

use zerotierone_controller::{self, local_client_from_file, authtoken_path};

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
        .subcommand(
            Command::new("network-info")
                .about("Print information about network")
                .arg(arg!([name] "Name of the network").required(true)),
        )
        .get_matches();

    let client = local_client_from_file(authtoken_path(None)).unwrap();

    if matches.subcommand_matches("status").is_some() {
        let status = zerotierone_controller::get_status().await?;
        let config = status.config.clone().unwrap();

        println!("Version: {}", status.version.clone().unwrap());
        println!("Address: {}", status.address.clone().unwrap());
        println!("Port: {}", config.settings.unwrap().primary_port.unwrap());
    }

    if matches.subcommand_matches("list-networks").is_some() {
        let networks = zerotierone_controller::get_controller_networks().await?;

        if networks.len() == 0 {
            println!("There are no networks");
        } else {
            println!("This controller has the following networks:");
            for network in networks {
                println!("- {}", network);
            }
        }
    }

    if let Some(matche) = matches.subcommand_matches("create-network") {
        let network_id = matche.get_one::<String>("network_id");

        let network = zerotierone_controller::generate_new_network(network_id.cloned()).await?;
        println!("Network ID is: {}", network.id.unwrap())
    }

    if let Some(matche) = matches.subcommand_matches("network-info") {
        let network_id = matche.get_one::<String>("name").expect("No network_id was given");

        let network = zerotierone_controller::get_controller_network(network_id).await?;
        let members = client.get_controller_network_members(network_id).await.unwrap().into_inner();

        let mut member_list = Vec::new();
        for (member, _) in members {
            let network_member = client.get_controller_network_member(network_id, &member).await?.into_inner();
            member_list.push(network_member);
        }
        println!("Network ID is: {}", network.id.unwrap());
        println!("Members:");
        for member in member_list {
            println!("- {}: authorized = {}", member.id.unwrap(), member.authorized.unwrap());
        }
    }

    Ok(())
}
