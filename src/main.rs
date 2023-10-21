use std::{env, thread};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;

fn connect(ip: &str) {

    if let Ok( stream) = TcpStream::connect(ip) {
        let now = Local::now();
        let _timestamp = format!("{}.{:06}", now.format("%Y-%m-%d %H:%M:%S"),
                                 now.timestamp_subsec_micros());
        println!("{_timestamp} ok: {}:{} --> {}",
                 stream.local_addr().unwrap().ip(),
                 stream.local_addr().unwrap().port(),
                 ip);
        // stream.write(b"verify").unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        let now = Local::now();
        let _timestamp = format!("{}.{:06}", now.format("%Y-%m-%d %H:%M:%S"),
                                 now.timestamp_subsec_micros());
        println!("{_timestamp} fail: src 0 --> {}",ip);
    }

}
fn ip_list() {
    let filepath: Vec<String> = env::args().collect();
    let remote = read_to_list(&filepath[2]);
    let handle = thread::spawn(move||
        {
            for i in 0..remote.len() {
                connect(&remote[i]);
            }
        });

    handle.join().unwrap();
}
fn read_to_list(path: &str) -> Vec<String> {
    let path = std::fs::read_to_string(path).unwrap();
    let result: Vec<String> = path
        .lines()
        .into_iter()
        .map(move |ch|ch.to_string())
        .collect();
    result
}

fn main() {
    let args: Vec<String>= env::args().collect();
    let cnt = args[1].parse::<i32>().unwrap();
    for _ in 0..cnt{
        sleep(Duration::from_secs(1));
        ip_list();
        println!("********")
    }
}
