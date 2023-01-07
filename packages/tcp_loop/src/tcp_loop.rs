use crate::utils::{prepend_string, submit_accept, submit_recv, submit_send};
use io_uring::IoUring;
use std::error::Error;
use std::net::TcpListener;
use std::os::fd::{AsRawFd, RawFd};

pub enum State {
    Accept,
    Recv,
    Send,
    Closed,
}

pub struct Connection {
    pub id: usize,
    pub state: State,
    pub fd_conn: RawFd,
}

pub fn tcp_loop(ring: IoUring) -> Result<(), Box<dyn Error>> {
    let mut ring = ring;

    let listener = TcpListener::bind(("127.0.0.1", 3456))?;
    let tcp_socket = listener.as_raw_fd();
    let mut buffs = vec![vec![0u8; 4096]; 128];

    let mut connections: Vec<Connection> = Vec::with_capacity(10);
    submit_accept(&mut ring, tcp_socket, &mut connections);

    'outer: loop {
        ring.submit_and_wait(1)?;

        let cqe = ring.completion().next().expect("completion queue is empty");
        println!("cqe: {:?}", cqe);

        let id = cqe.user_data() as usize;
        let connection = connections.get_mut(id).unwrap();
        match connection.state {
            State::Accept => {
                let fd_conn = cqe.result();
                if fd_conn < 0 {
                    break 'outer;
                }
                connections[id].fd_conn = fd_conn;

                println!("accepted fd: {}", fd_conn);
                submit_accept(&mut ring, tcp_socket, &mut connections);
                submit_recv(&mut ring, &mut connections[id], &mut buffs[id]);
            }
            State::Recv => {
                let byte_read = cqe.result();
                if byte_read == 0 {
                    continue 'outer;
                }
                if byte_read < 0 {
                    break 'outer;
                }
                let byte_read = byte_read as usize;
                prepend_string(&mut buffs[id], byte_read);
                submit_send(&mut ring, &mut connections[id], &mut buffs[id]);
            }
            State::Send => {
                let bytes_written = cqe.result();
                if bytes_written == -32 {
                    connections[id].state = State::Closed;
                }
                if bytes_written < 0 {
                    break 'outer;
                }
                println!("write fd: {}", bytes_written);
                submit_recv(&mut ring, &mut connections[id], &mut buffs[id]);
            }
            State::Closed => unsafe {
                libc::close(connection.fd_conn);
            },
        }
    }

    Ok(())
}
