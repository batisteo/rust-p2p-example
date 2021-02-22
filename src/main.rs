#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate log;

use libp2p::{
    core::transport::upgrade,
    floodsub::Topic,
    identity, mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    tcp::TokioTcpConfig,
    PeerId, Transport,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

const STORAGE_FILE_PATH: &str = "./recipes.json";

type Recipes = Vec<Repipe>;

#[derive(Debug, Serialize, Deserialize)]
struct Repipe {
    id: usize,
    name: String,
    ingredents: String,
    instuctions: String,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum EventType {
    Response(ListResponse),
    Input(String),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<EventType>();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("Cannot create auth keys");
    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    println!("      Ran")
}
