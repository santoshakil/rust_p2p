pub mod keypair;

use libp2p::PeerId;

use crate::keypair::KeyPair;

fn main() {
    let peer_id: PeerId = KeyPair::generate().peer_id();
    println!("Peer ID: {:?}", peer_id);
}
