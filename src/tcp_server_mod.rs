///
///tcp client
///use ip and port to connect the server
///and get response
///

pub mod tcp
{
    use std::str::from_utf8;
    use std::io::prelude::*;
    use std::net::{SocketAddrV4, TcpStream, Ipv4Addr};

    pub struct TcpServer
    {
        pub msg:String,
        pub stream:TcpStream,
        pub response:String
    }

    impl TcpServer
    {
        pub fn new(ip:Ipv4Addr,port:u16,msg:String)->TcpServer
        {
            let stream =TcpStream::connect(SocketAddrV4::new(ip,port)).unwrap();
            TcpServer
            {
                msg:msg,
                stream:stream,
                response:"".to_string()
            }

        }

        pub fn send_request(& mut self)
        {
            let _=self.stream.write(self.msg.as_bytes());

        }

        pub fn get_response(& mut self)
        {
            let mut buf:[u8;1024]=[0;1024];
            let _=self.stream.read(& mut buf);
            self.response=from_utf8(& mut buf).unwrap().to_string();
        }

    }

}

