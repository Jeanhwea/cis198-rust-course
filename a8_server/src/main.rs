use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // 绑定端口 12345
    let socket = UdpSocket::bind("127.0.0.1:12345")?;

    loop {
        // 读取 socket 中的数据
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // 将输入逆序
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
}
