use log::{error, info};
use std::io::Error;
use std::mem;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

const TIMEOUT_SLEEP_MODE_MS: u32 = 1000;
const TIMEOUT_TURBO_MODE_MS: u32 = 100;

fn get_child_cwd(child_pid: u32) -> Result<String, String> {
    let mut buffer = [0 as std::os::raw::c_char; 4096]; // 4096 = PATH_MAX
    let ret = unsafe {
        libc::readlink(
            format!("/proc/{}/cwd", child_pid).as_str().as_ptr() as *const i8,
            buffer.as_mut_ptr() as *mut i8,
            4095,
        )
    };
    match ret {
        -1 => Err(Error::last_os_error().to_string()),
        _ => unsafe {
            let message = from_utf8(&mem::transmute::<[i8; 4096], [u8; 4096]>(buffer))
                .unwrap_or("")
                .to_string();
            Ok(message
                .find('\0')
                .map(|pos| message.split_at(pos).0)
                .unwrap_or(&message)
                .to_string())
        },
    }
}

pub fn send_message(socket: &zmq::Socket, message: String) {
    let msg = zmq::Message::from(message.as_str());
    match socket.send(msg, zmq::DONTWAIT) {
        Ok(_) => {
            info!("[log-alacritty] sent msg `{}`", message)
        },
        Err(e) => {
            match e {
                zmq::Error::EAGAIN => (),
                _ => {
                    error!("[error-alacritty] failed to send msg `{}`: {}",
                           message, e);
                }
            }
        }
    }
}

pub fn handle_message(message: String,
                      socket: &zmq::Socket,
                      window_id: usize,
                      child_pid: u32,
                      timeoutms: &mut u32) {
    match message.as_str() {
        "XID?" => {
            send_message(socket,
                format!("XID:{}", window_id),
            );
        },
        "PWD?" => {
            match get_child_cwd(child_pid) {
                Ok(env) => {
                    send_message(
                        socket,
                        format!("PWD:{}", env),
                    );
                },
                Err(e) => {
                    info!(
                        "[log-alacritty] run_tcp_client : \
                         cannot get child PWD, {}",
                        e
                    );
                },
            }
        },
        "turbo" => {
            *timeoutms = TIMEOUT_TURBO_MODE_MS;
            info!("[log-alacritty] Going Turbo !");
        },
        "sleep" => {
            *timeoutms = TIMEOUT_SLEEP_MODE_MS;
            info!("[log-alacritty] Going asleep");
        },
        "" => {}, // Ignoring empty messages
        _ => {
            error!("[error-alacritty] run_tcp_client : unknown
                message `{}`, size: {}", message, message.len());
        },
    }
}

pub fn run_tcp_client(port: u16,
                      window_id: usize,
                      child_pid: u32,
                      running: Arc<AtomicBool>) {
    unsafe {
        info!("[log-alacritty] rust_tcp_client : pid is `{}`", libc::getpid());
    }
    info!("[log-alacritty] connecting...");

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PAIR).unwrap();
    match socket.connect(format!("tcp://0.0.0.0:{}", port).as_str()) {
        Ok(_) => (),
        Err(_) => {
            error!("[error-alacritty] can't connect to port {}", port);
            return;
        }
    }
    info!(
        "[log-alacritty] run_tcp_client : successfully connected to server in port {}",
        port
    );

    let mut timeoutms: u32 = TIMEOUT_TURBO_MODE_MS;

    while running.load(Ordering::Relaxed) {
        match socket.recv_msg(zmq::DONTWAIT) {
            Ok(valid_msg) => {
                match from_utf8(&valid_msg) {
                    Err(e) => {
                        error!("[error-alacritty] failed to \
                               decode UTF-8 msg: {}", e);
                    },
                    Ok(msg_str) => {
                        handle_message (
                            msg_str.get(0..msg_str.len()-3).unwrap()
                                .to_string(),
                            &socket, window_id, child_pid, &mut timeoutms);
                    }
                }
            },
            Err(e) => {
                match e {
                    zmq::Error::EAGAIN => {
                        thread::sleep(Duration::from_millis(timeoutms.into()));
                    },
                    _ => {
                        error!("[error-alacritty] failed to receive msg: {}",
                               e);
                    }
                }
            }
        }
    }
    info!("[debug-alacritty] run_tcp_client : xembed thread exitted loop...");
    info!("[log-alacritty] run_tcp_client : terminated.");
}
