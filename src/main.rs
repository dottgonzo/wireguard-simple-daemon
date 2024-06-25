use clap::{command, value_parser, Arg};
use core::net::SocketAddr;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let default_port = "51820";

    let args = command!()
        .about("update main embedded app")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("wireguard-server-endpoint")
                .short('e')
                .long_help("wireguard server endpoint")
                .value_name("WG_ENDPOINT")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("wireguard-server-public-key")
                .short('k')
                .long_help("wireguard server public key")
                .value_name("WG_PUBLIC_KEY")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("wireguard-client-private-key")
                .short('c')
                .long_help("wireguard client private key")
                .value_name("WG_PRIVATE_KEY")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("wireguard-client-address")
                .short('a')
                .long_help("wireguard client address")
                .value_name("WG_ADDRESS")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("wireguard-client-port")
                .short('p')
                .long_help("wireguard client port")
                .value_name("WG_PORT")
                .default_value(default_port)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("wireguard-client-addresses-masks")
                .short('m')
                .long_help("wireguard client addresses masks")
                .value_name("WG_ADDRESSES_MASK")
                .required(false),
        )
        .arg(
            Arg::new("network-prefix")
                .short('n')
                .long_help("network prefix")
                .value_name("NETWORK_PREFIX")
                .required(true),
        )
        .get_matches();

    // println!("{:?}", args);

    let server_endpoint_addr = args.get_one::<String>("wireguard-server-endpoint").unwrap();

    let server_endpoint: SocketAddr = server_endpoint_addr.parse().unwrap();

    let server_public_key = args
        .get_one::<String>("wireguard-server-public-key")
        .unwrap()
        .to_string();
    let client_private_key = args
        .get_one::<String>("wireguard-client-private-key")
        .unwrap()
        .to_string();
    let client_address = args
        .get_one::<String>("wireguard-client-address")
        .unwrap()
        .to_string();
    let client_port = args
        .get_one::<u32>("wireguard-client-port")
        .unwrap()
        .to_owned();

    let client_addresses_masks: Option<Vec<String>> = args
        .get_one::<String>("wireguard-client-addresses-mask")
        .map(|x| x.split(',').map(|x| x.to_string()).collect());

    let network_prefix: u8 = args
        .get_one::<String>("network-prefix")
        .unwrap()
        .parse()
        .unwrap();

    println!("server_endpoint: {:?}", server_endpoint_addr);
    println!("server_public_key: {:?}", server_public_key);
    println!("client_private_key: {:?}", client_private_key);
    println!("client_address: {:?}", client_address);
    println!("client_port: {:?}", client_port);
    println!("client_addresses_masks: {:?}", client_addresses_masks);
    println!("network_prefix: {:?}", network_prefix);

    let _ = wireguard_simple_rust_manager::routine_connect_to_wireguard(
        server_endpoint,
        server_public_key,
        client_private_key,
        client_address,
        Some(client_port),
        client_addresses_masks,
        network_prefix,
    )
    .await;
}
