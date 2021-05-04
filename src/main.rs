use std::net::SocketAddr;
use std::io::Read;
use quinn::{Incoming, Endpoint, ServerConfig, PrivateKey, CertificateChain, TransportConfig, ServerConfigBuilder};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // quinnの挙動をわかりやすくするためにログをTRACEレベルまで出す
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();

    // ポート番号はとりあえずクライアントと合わせておけば良いという認識、、未指定の時にどこに繋がるのか分からなかったので誰か教えて下さい
    let server_addr = "127.0.0.1:4433".parse().unwrap();
    let mut incoming = make_server_endpoint(server_addr)?;

    tokio::spawn(async move {
        loop {
            let incoming_conn = incoming.next().await.unwrap();
            let mut new_conn = incoming_conn.await.unwrap();

            println!(
                "[server] connection accepted: addr={}",
                new_conn.connection.remote_address()
            );

            // 繋がったらu8の数字を一つクライアントに送る(その後クライアントとこの数字をデクリメントしつつキャッチボールする)
            new_conn.connection.send_datagram(Bytes::from(vec![10])).unwrap();

            while let Some(Ok(received_bytes)) = new_conn.datagrams.next().await {
                println!("receive {}", received_bytes[0]);
                if received_bytes[0] > 0 {
                    std::thread::sleep(std::time::Duration::from_secs(1)); // すぐ終わると悲しいから無駄に1秒sleep
                    new_conn.connection.send_datagram(Bytes::from(vec![received_bytes[0] - 1])).unwrap(); // デクリメントしたものを送信
                } else {
                    println!("fin"); // 0になったら終了
                    break;
                }
            }
        }
    });

    wait_ctrl_c();

    Ok(())
}

#[allow(unused)]
pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<Incoming, Box<dyn Error>> {
    let server_config = configure_server()?;
    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.listen(server_config);
    let (_endpoint, incoming) = endpoint_builder.bind(&bind_addr)?;
    Ok(incoming)
}

#[allow(unused)]
fn configure_server() -> Result<ServerConfig, Box<dyn Error>> {
    let priv_key = PrivateKey::from_pem(&read_file("privkey.pem")?)?;
    let cert_chain = CertificateChain::from_pem(&read_file("fullchain.pem")?)?;
    let transport_config = TransportConfig::default();
    let mut server_config = ServerConfig::default();

    server_config.transport = Arc::new(transport_config);

    let mut cfg_builder = ServerConfigBuilder::new(server_config);

    cfg_builder.certificate(cert_chain, priv_key)?;
    cfg_builder.protocols(&[b"wq-vvv-01"]); // quic-transportの場合はwq-vvv-01を指定する

    Ok(cfg_builder.build())
}

/// ファイルを読んでVec<u8>して返すだけ
#[allow(unused)]
fn read_file(name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();

    std::fs::File::open(name)?.read_to_end(&mut buf);
    Ok(buf)
}

/// Ctrl+Cが押されるまで待つだけの処理
fn wait_ctrl_c() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
}
