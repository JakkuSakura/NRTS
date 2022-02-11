use bevy::app::ScheduleRunnerSettings;
use bevy::log::*;
use bevy::prelude::*;
use bevy_networking_turbulence::{
    ConnectionHandle, NetworkEvent, NetworkResource, NetworkingPlugin,
};
use nrts_core::core::NrtsCore;
use nrts_core::network::{
    decode_request, encode_response, get_type_registry, make_world_backup, NetworkRequest,
    NetworkResponse,
};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

const SERVER_PORT: u16 = 14191;

struct Args {
    port: u16,
}
#[derive(Default)]
struct ConnectedClients {
    clients: Vec<ConnectionHandle>,
}

fn main() {
    App::empty()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(Args { port: SERVER_PORT })
        .insert_resource(get_type_registry())
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_plugin(NrtsCore {})
        .add_startup_system(startup.system())
        .add_system(handle_packets.exclusive_system())
        .run();
}

fn startup(mut net: ResMut<NetworkResource>, args: Res<Args>) {
    info!("Starting server at port {}", args.port);
    let server_address = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), args.port);
    net.listen(server_address, None, None);
}

fn handle_packets(
    world: ResMut<World>,
    mut net: ResMut<NetworkResource>,
    mut clients: ResMut<ConnectedClients>,
    mut reader: EventReader<NetworkEvent>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                info!("Client {:?} connected", handle);
                clients.clients.push(*handle);
            }
            NetworkEvent::Disconnected(handle) => {
                info!("Client {:?} disconnected", handle);
                clients.clients.retain(|x| x != handle);
            }
            NetworkEvent::Packet(handle, packet) => {
                let request = decode_request(packet.as_ref());
                info!("Got packet on [{}]: {:?}", handle, request);
                match request {
                    NetworkRequest::RequestWorld => {
                        let backup = make_world_backup(&*world);
                        net.send(
                            *handle,
                            encode_response(&NetworkResponse::ResponseWorld(backup)),
                        )
                        .unwrap();
                    }
                }
            }
            NetworkEvent::Error(handle, err) => warn!("{:?} error {:?}!", handle, err),
        }
    }
}
