use log::{error, info};
use std::path::PathBuf;
use std::str::FromStr;

use std::sync::Arc;

#[cfg(not(windows))]
use libproc::libproc::proc_pid::pidcwd;
use tokio::sync::Notify;
use zeromq::util::PeerIdentity;
use zeromq::{Socket, SocketOptions, SocketRecv, SocketSend};

const TIMEOUT_SLEEP_MODE_MS: u32 = 1000;
const TIMEOUT_TURBO_MODE_MS: u32 = 100;

#[cfg(not(windows))]
pub fn get_child_cwd(child_pid: i32) -> Result<PathBuf, String> {
    match pidcwd(child_pid) {
        Ok(cwd) => Ok(cwd),
        Err(err) => Err(format!("Failed to get the current working directory: {}", err)),
    }
}

pub async fn send_message(socket: &mut zeromq::RepSocket, message: String) {
    match socket.send(message.clone().into()).await {
        Ok(_) => {},
        Err(e) => error!("[error-alacritty] failed to send msg `{}`: {}", message, e),
    }
}

pub async fn handle_message(
    message: String,
    socket: &mut zeromq::RepSocket,
    window_id: &String,
    child_pid: i32,
    timeoutms: &mut u32,
) {
    match message.as_str() {
        "XID?" => {
            send_message(socket, format!("XID:{}", window_id)).await;
        },
        "PWD?" => match get_child_cwd(child_pid) {
            Ok(env) => {
                let pwd = env.into_os_string().into_string().unwrap();
                send_message(socket, format!("PWD:{}", pwd)).await;
                info!("run_tcp_client : sending pwd, {}", pwd);
            },
            Err(e) => {
                send_message(socket, "OK".to_string()).await;
                info!("run_tcp_client : cannot get child PWD, {}", e);
            },
        },
        "turbo" => {
            send_message(socket, "OK".to_string()).await;
            *timeoutms = TIMEOUT_TURBO_MODE_MS;
            info!("Going Turbo !");
        },
        "sleep" => {
            send_message(socket, "OK".to_string()).await;
            *timeoutms = TIMEOUT_SLEEP_MODE_MS;
            info!("Going asleep");
        },
        "" => {
            send_message(socket, "OK".to_string()).await;
        }, // Ignoring empty messages
        _ => {
            send_message(socket, "OK".to_string()).await;
            error!(
                "[error-alacritty] run_tcp_client : unknown
                message `{}`, size: {}",
                message,
                message.len()
            );
        },
    }
}

pub async fn run_tcp_client(
    port: u16,
    window_id: &String,
    child_pid: i32,
    shutdown_ntfy: Arc<Notify>,
) {
    unsafe {
        info!("rust_tcp_client : pid is `{}`", libc::getpid());
    }
    info!("connecting...");
    let mut options = SocketOptions::default();
    options.peer_identity(PeerIdentity::from_str("alacritty").unwrap());
    let mut socket = zeromq::RepSocket::with_options(options);
    match socket.connect(format!("tcp://0.0.0.0:{}", port).as_str()).await {
        Ok(_) => (),
        Err(_) => {
            error!("[error-alacritty] can't connect to port {}", port);
            return;
        },
    }
    info!("run_tcp_client : successfully connected to server in port {}", port);

    let mut timeoutms: u32 = TIMEOUT_TURBO_MODE_MS;

    loop {
        tokio::select! {
            request = socket.recv() => {
                match request
                        .map_err(|e| e.to_string())
                        .and_then(|x| { <zeromq::ZmqMessage as std::convert::TryInto<String>>::try_into(x).map_err(|e| e.to_string()) }) {
                    Ok(msg_str) => {
                        handle_message(
                            msg_str.get(0..msg_str.len() - 3).unwrap().to_string(),
                            &mut socket,
                            window_id,
                            child_pid,
                            &mut timeoutms,
                        ).await
                    },
                    Err(e) => error!("[run_tcp_client] can't receive message on port {}: {}", port, e),
                }
            }
            _ = shutdown_ntfy.notified() => {
                break;
            }
        }
    }

    info!("run_tcp_client : xembed thread exitted loop...");
    info!("run_tcp_client : terminated.");
}
