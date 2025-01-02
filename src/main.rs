mod drone_test;
mod network_initializer;
mod utils;

use crate::network_initializer::{load_from_file, NetworkInitializer};
use crate::utils::dummy::DummyLeaf;
use crate::utils::factory::*;
use common_structs::leaf::Leaf;
use wg_2024::drone::Drone;

use ap2024_unitn_cppenjoyers_drone::CppEnjoyersDrone;
use bagel_bomber::BagelBomber;
use common_structs::network::Network;
use d_r_o_n_e_drone::MyDrone as DRONEDrone;
use dr_ones::Drone as DrOnes;
use drone::RustBustersDrone;
use fungi_drone::FungiDrone;
use lockheedrustin_drone::LockheedRustin;
use rustafarian_drone::RustafarianDrone;
use wg_2024_rust::drone::RustDrone;
use LeDron_James::Drone as LeDronJames;

fn main() {
    initialize_network();
}

fn initialize_network() -> Network {
    let drone_factories = drone_factories!(
        RustafarianDrone,
        DrOnes,
        FungiDrone,
        DRONEDrone,
        CppEnjoyersDrone,
        LockheedRustin,
        LeDronJames,
        BagelBomber,
        RustDrone,
        RustBustersDrone
    );

    let client_factories = leaf_factories!(DummyLeaf, DummyLeaf);
    let server_factories = leaf_factories!(DummyLeaf, DummyLeaf);

    let config = load_from_file("./config.toml");
    NetworkInitializer::start_simulation_from_config(
        config,
        drone_factories,
        client_factories,
        server_factories,
    )
}
