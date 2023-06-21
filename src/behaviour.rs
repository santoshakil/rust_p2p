use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::async_io::Behaviour,
}
