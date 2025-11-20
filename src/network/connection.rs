use anyhow::{Error, Result};
use tokio::{io::AsyncReadExt, net::TcpStream};
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut tmp_bytes: [u8; 1024] = [0; 1024];

        loop {
            let n = match self.stream.read(&mut tmp_bytes).await {
                Ok(n) => n,
                Err(e) => {
                    return Err(Error::msg(format!("Failed to read from stream: {}", e)));
                }
            };

            if n == 0 {
                if bytes.is_empty() {
                    // 连接关闭且未读取到任何数据
                    return Err(Error::msg("Connection closed by peer"));
                } else {
                    // 连接关闭但已读取部分数据
                    break;
                }
            }

            bytes.extend_from_slice(&tmp_bytes[..n]);
            if n < tmp_bytes.len() {
                // 读取完毕
                break;
            }
        }

        Ok(bytes)
    }
}
