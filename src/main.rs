use clap::{command, value_parser, Arg, ArgAction};
use core::net::SocketAddr;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");

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
                .short('p')
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
                .short('o')
                .long_help("wireguard client port")
                .value_name("WG_PORT")
                .default_value(default_port)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("wireguard-client-addresses-mask")
                .short('m')
                .long_help("wireguard client addresses mask")
                .value_name("WG_ADDRESSES_MASK")
                .required(false),
        )
        .get_matches();

    // println!("{:?}", args);

    let server_endpoint_addr = args.get_one::<String>("wireguard-server-endpoint").unwrap();

    println!("server_endpoint: {:?}", server_endpoint_addr);

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

    let client_addresses_maks: Option<Vec<String>> = args
        .get_one::<String>("wireguard-client-addresses-mask")
        .map(|x| x.split(',').map(|x| x.to_string()).collect());

    return;
    // let server_public_key = "N9ZPcCtSJJQIp/GtfD5+EAiNQlyABe06GPEaibKtmws=".to_string();
    // let client_private_key = "0PPBFCQ+p2OwJBPbw+OrYecb6pKp4DqIDT0GP4EIsF4=".to_string();
    // let client_address = "10.33.0.33".to_string();
    // let client_port = Some(12345);
    // let client_addresses_maks = Some(vec!["10.33.0.0/16".to_string()]);

    let result = wireguard_simple_rust_manager::connect_to_wireguard(
        server_endpoint,
        server_public_key,
        client_private_key,
        client_address,
        Some(client_port),
        client_addresses_maks,
    )
    .await;

    assert!(result.is_ok());
}
