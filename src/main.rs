mod behaviour;
mod config;

use async_std::io;
use futures::{prelude::*, select};
use libp2p::{
    gossipsub, mdns,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
};
use std::error::Error;
use std::str::FromStr;

use crate::behaviour::MyBehaviourEvent;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (local_peer_id, trns, behaviour, topic) = config::configure_swarm()?;

    let mut swarm = SwarmBuilder::with_async_std_executor(trns, behaviour, local_peer_id).build();

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    loop {
        select! {
            line = stdin.select_next_some() => {
                if let Some(peer_id_str) = line.as_ref().expect("Stdin not to close").strip_prefix("PeerID: ") {
                    let peer_id = PeerId::from_str(peer_id_str.trim())?;
                    swarm.dial(peer_id)?;
                } else if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
                    eprintln!("Publish error: {:?}", e);
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => println!(
                        "Got message: '{}' with id: {} from peer: {}",
                        String::from_utf8_lossy(&message.data),
                        id,
                        peer_id,
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {}", address);
                }
                _ => {}
            }
        }
    }
}
