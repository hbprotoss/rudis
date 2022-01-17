use tokio::{net::TcpStream, io::{AsyncBufReadExt, BufWriter, AsyncWriteExt, split, WriteHalf, ReadHalf, AsyncReadExt}};

use log::debug;

use super::{proto::Proto, reader::BufioReader};

pub struct Conn {
    writer: BufWriter<WriteHalf<TcpStream>>,
    reader: BufioReader<ReadHalf<TcpStream>>,
}

impl Conn {
    pub async fn new(host: String, port: u16) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", host, port)).await.unwrap();
        Conn::new_from_tcp_stream(stream).await
    }

    pub async fn new_from_tcp_stream(stream: TcpStream) -> Self {
        let (reader, writer) = split(stream);
        Self {
            writer: BufWriter::new(writer),
            reader: BufioReader::new(reader),
        }
    }

    pub async fn decode(&mut self, proto: &mut Proto) {
        match proto.decode(&mut self.reader).await {
            Err(e) => {
                debug!("{:?}", e);
            }
            Ok(_) => {},
        }
    }

    pub async fn encode(&mut self, proto: &Proto) {
        proto.encode(&mut self.writer).await.unwrap();
        self.writer.flush().await.unwrap();
    }

}