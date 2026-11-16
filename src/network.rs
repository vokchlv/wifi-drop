use std::net::{IpAddr, TcpListener, UdpSocket};

pub struct BoundServerDTO {
    pub listener: TcpListener,
    pub ip: IpAddr,
    pub port: u16,
}

impl BoundServerDTO {
    pub fn url(&self) -> String {
        format!("http://{}:{}/", self.ip, self.port)
    }
}

pub fn create_tcp_listener() -> BoundServerDTO {
    let listener = TcpListener::bind("0.0.0.0:7878")
        .expect("Couldn't bind to TcpListener, please check if the port is already in use.");
    let port = listener.local_addr().unwrap().port();
    let ip = get_local_ip_address().expect("No valid IP-Address found!");

    BoundServerDTO { listener, ip, port }
}

fn get_local_ip_address() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}
