use crossbeam::channel::*;
use std::thread;
use std::time::Instant;

//Optimized: cargo run --color=always --package crossbeam-demo --bin crossbeam-demo --release

#[derive(PartialEq)]
struct Msg {
    id: i64,
    pad: i64,
    wrd: String,
    category: String,
}

fn main() {
    let buf_size = 1048576;
    let producer_msg_count = 50000000;
    let (s, r) = bounded(buf_size);
    let s2 = s.clone();

    let start_time = Instant::now();
    let t1 = thread::spawn(move || {
        for msg_num in 0..producer_msg_count {
            let msg = Msg {
                id: msg_num & 0xf,
                pad: msg_num,
                wrd: String::from("ABCDEFEKAKJS"),
                category: String::from("A")
            };
            s.send(msg).unwrap();
        }
    });

    let t2 = thread::spawn(move || {
        for msg_num in 0..producer_msg_count {
            let msg = Msg {
                id: msg_num & 0xff,
                pad: msg_num,
                wrd: String::from("ABCDEFEKAKJS"),
                category: String::from("A")
            };
            s2.send(msg).unwrap();
        }
    });

    let mut sum = 0;
    let mut cnt = 0;
    for msg in r {
        let tmp = msg;
        sum += tmp.id;
        cnt += 1;
    }

    let _ = t1.join();
    let _ = t2.join();

    let d = Instant::now().duration_since(start_time);
    let delta = d.as_millis();
    println!("Message count: {}, sum: {}, processed  time: {}", cnt, sum, delta);
}