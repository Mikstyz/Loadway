use crate::helper::time;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Instant, timeout};

pub mod message;
pub mod state;

pub struct Connection {
    pub name: String,
    pub at: u128,

    pub stream: TcpStream,
    pub state: state::ConnectionState,
    pub ms_latenсy: u128,
}

impl Connection {
    const TRY: u32 = 3;
    const TIMEOUT_PER_TRY: Duration = Duration::from_secs(2);

    //create msg ping and waiting answer pong
    pub async fn ping(&mut self) {
        self.state = state::ConnectionState::Offline;
        self.ms_latenсy = 0;

        let mut buffer = [0; 4];

        for _i in 0..Self::TRY {
            let start = Instant::now();

            let ping_operation = async {
                self.stream.write_all(b"PING").await?;
                self.stream.flush().await?;

                self.stream.read_exact(&mut buffer).await?;

                Ok::<(), std::io::Error>(())
            };

            match timeout(Self::TIMEOUT_PER_TRY, ping_operation).await {
                Ok(Ok(())) => {
                    self.ms_latenсy = start.elapsed().as_millis();
                    self.state = state::ConnectionState::Online;
                    return;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    pub async fn close(&mut self) {}
    pub async fn message(&self, msg: message::Msg) {}
}

pub fn relay(mut inp_connectoin: Connection, mut out_connectoin: Connection) {}
