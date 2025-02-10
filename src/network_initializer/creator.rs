use crate::network::{DroneInfo, LeafInfo, NodeInfo, TypeInfo};
use crate::utils::factory::{DroneEvent, DroneFactory, LeafFactory, NodeId, Packet};
use crate::NetworkInitializer;
use common_structs::leaf::LeafEvent;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::config;

impl NetworkInitializer {
    /// Generate node info.
    fn new_info(
        neighbours: Vec<NodeId>,
        type_info: TypeInfo,
        packet_in_channel: Sender<Packet>,
    ) -> NodeInfo {
        NodeInfo {
            neighbours: neighbours.into_iter().collect(),
            packet_in_channel,
            type_info,
        }
    }

    /// Add new drone to the network.
    /// Start running the drone on a new thread.
    /// # Returns
    /// Node info of the new drone.
    pub(super) fn new_drone(
        data: &config::Drone,
        factory: &DroneFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<DroneEvent>,
    ) -> NodeInfo {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_node_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut drone = factory(
            data.id,
            event_send,
            command_rcv,
            packet_rcv,
            packet_send,
            data.pdr,
        );

        thread::spawn(move || drone.run());

        let type_info = TypeInfo::Drone(DroneInfo {
            pdr: data.pdr,
            command_send_channel: command_send,
        });
        Self::new_info(data.connected_node_ids.clone(), type_info, packet_in)
    }

    /// Add new client (leaf) to the network.
    /// Start running the client on a new thread.
    /// # Returns
    /// Node info of the new client.
    pub(super) fn new_client(
        data: &config::Client,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafEvent>,
    ) -> NodeInfo {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_drone_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut leaf = factory(data.id, event_send, command_rcv, packet_rcv, packet_send);

        thread::spawn(move || {
            leaf.run();
        });

        let type_info = TypeInfo::Client(LeafInfo {
            command_send_channel: command_send,
        });
        Self::new_info(data.connected_drone_ids.clone(), type_info, packet_in)
    }

    /// Add new server (leaf) to the network.
    /// Start running the server on a new thread.
    /// Returns node info of the new server.
    pub(super) fn new_server(
        data: &config::Server,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafEvent>,
    ) -> NodeInfo {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_drone_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut leaf = factory(data.id, event_send, command_rcv, packet_rcv, packet_send);

        thread::spawn(move || {
            leaf.run();
        });

        let type_info = TypeInfo::Server(LeafInfo {
            command_send_channel: command_send,
        });
        Self::new_info(data.connected_drone_ids.clone(), type_info, packet_in)
    }
}

/// Create hashmap using the nodes in filter as key.
/// Value (sender channel) is taken from a lookup of the respective node in all.
fn filter_hashmap_sender(
    all: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    filter: &Vec<NodeId>,
) -> HashMap<NodeId, Sender<Packet>> {
    let mut res = HashMap::new();

    for k in filter {
        res.insert(*k, all[k].0.clone());
    }

    res
}
