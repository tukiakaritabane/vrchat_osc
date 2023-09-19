use rosc::{
    decoder,
    OscPacket,
};
use std::net::{
    SocketAddrV4,
    UdpSocket,
};
use std::str::FromStr;

fn get_addr(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC arguments: {:?}", msg.args);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

fn main() {
    let recv = get_addr("127.0.0.1:9001");
    let sock = UdpSocket::bind(recv).unwrap();
    let mut buf = [0u8; decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let (_, packet) = decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet);
            }
            Err(_) => {},
        }
    }
}
