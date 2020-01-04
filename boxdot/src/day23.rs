use crate::day09::{execute, parse, Memory};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError, Sender, TryRecvError};
use std::sync::Arc;
use std::time::Duration;

pub fn solve(input: &str) -> (i64, i64) {
    let mem = parse(input);
    network_simulation(mem)
}

fn network_simulation(mem: Memory) -> (i64, i64) {
    let packets_inflight = Arc::new(AtomicUsize::new(0));
    let (bus_sender, bus_receiver) = channel();
    let ports: Vec<_> = (0..50)
        .map(|addr| {
            run_async(
                addr,
                mem.clone(),
                bus_sender.clone(),
                packets_inflight.clone(),
            )
        })
        .collect();

    let broker = std::thread::spawn(move || {
        let mut first_nat_packet = Packet::default();
        let mut prev_nat_packet: Option<Packet> = None;
        let mut nat_packet: Option<Packet> = None;

        let timeout = Duration::from_millis(100);
        loop {
            match bus_receiver.recv_timeout(timeout) {
                Ok(packet) if packet.addr < 50 => {
                    ports[packet.addr].send(packet).expect("port disconnected")
                }
                Ok(packet) if packet.addr == 255 => {
                    if nat_packet.is_some() {
                        packets_inflight.fetch_sub(1, Ordering::SeqCst);
                    }
                    if prev_nat_packet.is_none() {
                        first_nat_packet = packet;
                    }
                    nat_packet = Some(packet)
                }
                Ok(packet) => panic!("invalid packet addr: {}", packet.addr),
                Err(RecvTimeoutError::Disconnected) => panic!("bus receiver disconnected"),
                Err(RecvTimeoutError::Timeout) => (),
            }

            if let Some(mut packet) = nat_packet {
                let packets_inflight = packets_inflight.load(Ordering::SeqCst);
                if packets_inflight == 1 {
                    packet.addr = 0;
                    ports[0].send(packet).expect("port 0 disconnected");

                    if prev_nat_packet
                        .map(|prev_packet| prev_packet.y == packet.y)
                        .unwrap_or(false)
                    {
                        return (first_nat_packet, packet);
                    }
                    prev_nat_packet = Some(packet);
                    nat_packet = None;
                }
            }
        }
    });

    let (first_nat_packet, repeating_nat_packet) = broker.join().unwrap();
    (first_nat_packet.y, repeating_nat_packet.y)
}

#[derive(Debug, Clone, Copy)]
enum InputState {
    Boot,
    Recv,
    Data(Packet),
}

#[derive(Debug, Clone, Copy, Default)]
struct Packet {
    addr: usize,
    x: i64,
    y: i64,
}

fn run_async(
    addr: usize,
    mut mem: Memory,
    output_sender: Sender<Packet>,
    packets_inflight: Arc<AtomicUsize>,
) -> Sender<Packet> {
    use InputState::*;

    let (input_sender, input_receiver) = channel::<Packet>();

    std::thread::spawn(move || {
        let mut input_state = InputState::Boot;
        let mut output_state = 0; // we use mod 3 arithmetic
        let mut output_packet = Packet::default();

        let mut ip = Some(0);
        while let Some(next_ip) = ip {
            ip = execute(
                &mut mem,
                next_ip,
                || match input_state {
                    Boot => {
                        input_state = Recv;
                        addr as i64
                    }
                    Recv => match input_receiver.try_recv() {
                        Ok(packet) => {
                            packets_inflight.fetch_sub(1, Ordering::SeqCst);
                            assert_eq!(packet.addr, addr);
                            input_state = Data(packet);
                            packet.x
                        }
                        Err(TryRecvError::Empty) => -1,
                        Err(TryRecvError::Disconnected) => {
                            ip = Some(9999); // stop command
                            -1
                        }
                    },
                    Data(packet) => {
                        input_state = Recv;
                        packet.y
                    }
                },
                |value| {
                    match output_state % 3 {
                        0 => {
                            output_packet = Packet::default();
                            output_packet.addr = value as usize;
                        }
                        1 => {
                            output_packet.x = value;
                        }
                        2 => {
                            packets_inflight.fetch_add(1, Ordering::SeqCst);
                            output_packet.y = value;
                            output_sender
                                .send(output_packet)
                                .expect("sender disconnected");
                        }
                        _ => unreachable!(),
                    }
                    output_state += 1;
                },
            )
            .unwrap();
        }
    }); // detached thread

    input_sender
}
