use libp2p::{
    core::upgrade::Version,
    futures::executor::ThreadPool,
    identity::Keypair,
    noise, ping,
    swarm::{keep_alive, NetworkBehaviour, SwarmBuilder},
    tcp, yamux, PeerId, Transport,
};

fn main() {
    let local_key = Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", peer_id);

    let noise_config = noise::Config::new(&local_key).unwrap();

    let trns = tcp::async_io::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = Behaviour::default();

    let mut swarm = match ThreadPool::new() {
        Ok(tp) => SwarmBuilder::with_executor(trns, behaviour, peer_id, tp),
        Err(_) => SwarmBuilder::without_executor(trns, behaviour, peer_id),
    }
    .build();

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    println!("{:?}", swarm.network_info());
}

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
