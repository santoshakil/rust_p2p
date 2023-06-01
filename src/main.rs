mod keypair;

use libp2p::PeerId;

fn main() {
    let peer_id: PeerId = crate::keypair::KeyPair::generate().peer_id();
    println!("Peer ID: {:?}", peer_id);
}
