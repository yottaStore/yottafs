use crate::tcp_loop::{Connection, State};
use io_uring::{opcode, types, IoUring};
use std::os::fd::RawFd;
use std::ptr;

pub fn submit_accept(ring: &mut IoUring, tcp_socket: RawFd, connections: &mut Vec<Connection>) {
    let id = connections.len();
    connections.push(Connection {
        id,
        state: State::Accept,
        fd_conn: 0,
    });

    let accept = opcode::Accept::new(types::Fd(tcp_socket), ptr::null_mut(), ptr::null_mut())
        .build()
        .user_data(id as u64);

    unsafe {
        ring.submission()
            .push(&accept)
            .expect("submission queue is full");
    }
}

pub fn submit_recv(ring: &mut IoUring, request: &mut Connection, buf: &mut Vec<u8>) {
    for item in &mut buf[..] {
        *item = 0;
    }

    let read_e = opcode::Recv::new(
        types::Fd(request.fd_conn),
        buf.as_mut_ptr(),
        buf.len() as u32,
    )
    .build()
    .user_data(request.id as u64);

    request.state = State::Recv;

    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("submission queue is full");
    }
}

pub fn submit_send(ring: &mut IoUring, connection: &mut Connection, buf: &mut Vec<u8>) {
    let read_e = opcode::Send::new(
        types::Fd(connection.fd_conn),
        buf.as_mut_ptr(),
        buf.len() as u32,
    )
    .build()
    .user_data(connection.id as u64);

    connection.state = State::Send;

    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("submission queue is full");
    }
}

pub fn prepend_string(vec: &mut Vec<u8>, len: usize) {
    let msg = "Hello \"".as_bytes();
    let msg_len = msg.len();
    for i in (0..len).rev() {
        vec[i + msg_len] = vec[i];
    }
    for i in 0..msg_len {
        vec[i] = msg[i];
    }

    vec[len + msg_len - 1] = '\"' as u8;
    vec[len + msg_len] = '!' as u8;
    vec[len + msg_len + 1] = '\n' as u8;
}
