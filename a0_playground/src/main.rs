use std::net::UdpSocket;
use std::{io, str};

fn main() -> std::io::Result<()> {
    // 绑定端口
    let socket = UdpSocket::bind("127.0.0.1:11111")?;

    // 建立链接
    socket.connect("127.0.0.1:12345")?;

    loop {
        // 读取输入数据
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        // 发送数据，并接收结果
        let mut buffer = [0u8; 1500];
        socket.recv_from(&mut buffer)?;

        // 打印输出结果
        println!(
            "RECV: {}",
            str::from_utf8(&buffer).expect("failed to write")
        );
    }
}
