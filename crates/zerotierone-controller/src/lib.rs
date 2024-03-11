use std::path::Path;

use zerotier_one_api::types::{ Network, ControllerNetwork };
use reqwest::header::{HeaderMap, HeaderValue};

// determine the path of the authtoken.secret
pub fn authtoken_path(arg: Option<&Path>) -> &Path {
    if let Some(arg) = arg {
        return arg;
    }

    if cfg!(target_os = "linux") {
        Path::new("/var/lib/zerotier-one/authtoken.secret")
    } else if cfg!(target_os = "windows") {
        Path::new("C:/ProgramData/ZeroTier/One/authtoken.secret")
    } else if cfg!(target_os = "macos") {
        Path::new("/Library/Application Support/ZeroTier/One/authtoken.secret")
    } else {
        panic!("authtoken.secret not found; please provide the -s option to provide a custom path")
    }
}

pub fn local_client_from_file(
    authtoken_path: &Path,
) -> Result<zerotier_one_api::Client, anyhow::Error> {
    let authtoken = std::fs::read_to_string(authtoken_path)?;
    local_client(authtoken)
}

fn local_client(authtoken: String) -> Result<zerotier_one_api::Client, anyhow::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("X-ZT1-Auth", HeaderValue::from_str(&authtoken)?);

    Ok(zerotier_one_api::Client::new_with_client(
        "http://127.0.0.1:9993",
        reqwest::Client::builder()
            .default_headers(headers)
            .build()?,
    ))
}

pub async fn get_controller_networks() -> Result<Vec<String>, anyhow::Error> {
    let client = local_client_from_file(authtoken_path(None))?;
    let networks = client.get_controller_networks().await?;

    Ok(networks.to_vec())
}

pub async fn get_controller_network(network_id: &str) -> Result<ControllerNetwork, anyhow::Error> {
    let client = local_client_from_file(authtoken_path(None))?;
    let network = client.get_controller_network(network_id).await?;

    Ok(network.into_inner())
}

pub async fn generate_new_network(network_id: Option<String>) -> Result<ControllerNetwork, zerotier_one_api::Error> {
    let client = local_client_from_file(authtoken_path(None)).unwrap();
    let status = client.get_status().await.unwrap().into_inner();
    let response = client.generate_controller_network(
            &status.address.unwrap(),
            &ControllerNetwork {
                capabilities: Vec::new(),
                creation_time: None,
                enable_broadcast: None,
                id: network_id,
                ip_assignment_pools: Vec::new(),
                mtu: None,
                multicast_limit: None,
                name: None,
                nwid: None,
                objtype: None,
                private: None,
                remote_trace_level: None,
                remote_trace_target: None,
                revision: None,
                routes: Vec::new(),
                rules: Vec::new(),
                tags: Vec::new(),
                v4_assign_mode: None,
                v6_assign_mode: None,
            },
        ).await;

    println!("{:?}", response.unwrap_err());

    // Ok(response.unwrap().into_inner())
    Ok(ControllerNetwork {
                capabilities: Vec::new(),
                creation_time: None,
                enable_broadcast: None,
                id: None,
                ip_assignment_pools: Vec::new(),
                mtu: None,
                multicast_limit: None,
                name: None,
                nwid: None,
                objtype: None,
                private: None,
                remote_trace_level: None,
                remote_trace_target: None,
                revision: None,
                routes: Vec::new(),
                rules: Vec::new(),
                tags: Vec::new(),
                v4_assign_mode: None,
                v6_assign_mode: None,
            },
        )
}

pub async fn get_status() -> Result<zerotier_one_api::types::Status, anyhow::Error> {
    let client = local_client_from_file(authtoken_path(None))?;
    Ok(client.get_status().await.unwrap().to_owned())
}

pub async fn get_address() -> Result<String, anyhow::Error> {
    let status = get_status().await?;
    Ok(status.address.unwrap())
}

// #[tokio::main]
pub async fn main() {
    // generate_new_network();
    let c = local_client_from_file(authtoken_path(None)).unwrap();
    let address = &c.get_status().await.unwrap().address.clone().unwrap();
    println!("My id is: {}", address);

    let networks = &c.get_controller_networks().await.unwrap().to_vec();
    println!("My networks are:");
    for network in networks {
        println!("- {}", network)
    }



    // let gn = c.generate_controller_network(&format!("{}______", address), &cn).await;
}
