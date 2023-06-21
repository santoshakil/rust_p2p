use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::Boxed;
use libp2p::gossipsub::IdentTopic;
use libp2p::{
    core::upgrade::Version, gossipsub, identity, mdns, noise, tcp, yamux, PeerId, Transport,
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::behaviour::MyBehaviour;

pub fn configure_swarm() -> Result<
    (
        PeerId,
        Boxed<(PeerId, StreamMuxerBox)>,
        MyBehaviour,
        IdentTopic,
    ),
    Box<dyn Error>,
> {
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {}", local_peer_id);

    let noise_config = noise::Config::new(&id_keys)?;
    let trns = tcp::async_io::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .boxed();

    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()?;

    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    )?;
    let topic = gossipsub::IdentTopic::new("test-net");
    gossipsub.subscribe(&topic)?;

    let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
    let behaviour = MyBehaviour { gossipsub, mdns };

    Ok((local_peer_id, trns, behaviour, topic))
}
