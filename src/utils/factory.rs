use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
pub use crossbeam_channel::{Receiver, Sender};
pub use std::collections::HashMap;
pub use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::drone::Drone;
pub use wg_2024::network::NodeId;
pub use wg_2024::packet::Packet;

pub trait DroneRunnable: Drone {}
impl<T: Drone> DroneRunnable for T {}
pub type DroneFactory = Box<
    dyn Fn(
        NodeId,
        Sender<DroneEvent>,
        Receiver<DroneCommand>,
        Receiver<Packet>,
        HashMap<NodeId, Sender<Packet>>,
        f32,
    ) -> Box<dyn DroneRunnable>,
>;
pub struct DroneImpl {
    pub name: String,
    pub create: DroneFactory,
}

pub trait LeafRunnable: Leaf {}
impl<T: Leaf> LeafRunnable for T {}
pub type LeafFactory = Box<
    dyn Fn(
        NodeId,
        Sender<LeafEvent>,
        Receiver<LeafCommand>,
        Receiver<Packet>,
        HashMap<NodeId, Sender<Packet>>,
    ) -> Box<dyn LeafRunnable>,
>;
pub struct LeafImpl {
    pub name: String,
    pub create: LeafFactory,
}

#[macro_export]
macro_rules! drone_factories {
    ($($type_name:ty, $name_impl:tt),* $(,)?) => {{
        vec![
            $(
                DroneImpl{
                    name: ($name_impl).to_string(),
                    create: Box::new(
                        |id, csend, crecv, precv, psend, pdr| -> Box<dyn DroneRunnable> {
                            Box::new(<$type_name>::new(id, csend, crecv, precv, psend, pdr))
                        }
                    ) as DroneFactory
                }
            ),*
        ]
    }};
}

#[macro_export]
macro_rules! leaf_factories {
    ($($type_name:ty),* $(,)?) => {{
        vec![
            $(
                LeafImpl{
                    name: ($name_impl).to_string(),
                    init: Box::new(
                        |id, csend, crecv, precv, psend| -> Box<dyn LeafRunnable> {
                            Box::new(<$type_name>::new(id, csend, crecv, precv, psend))
                        }
                    ) as LeafFactory
                }
            ),*
        ]
    }};
}
