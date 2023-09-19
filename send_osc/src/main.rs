use rosc::{
    encoder,
    OscMessage,
    OscPacket,
    OscType,
};
use std::net::{
    SocketAddrV4,
    UdpSocket,
};
use std::str::FromStr;

fn get_addr(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

fn main() {
    let sock_addr = get_addr("127.0.0.1:9001");
    let send_addr = get_addr("127.0.0.1:9000");

    let sock = UdpSocket::bind(sock_addr).unwrap();
    let buf = encoder::encode(
        &OscPacket::Message(
            OscMessage {
                addr: "/avatar/parameters/VRCEmote".to_string(),
                args: vec![OscType::Int(2)],
            }
        )
    ).unwrap();

    sock.send_to(&buf, send_addr).unwrap();
}
