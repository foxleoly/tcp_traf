use std::{env, thread};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
// use chrono::format::Item::Fixed;
use chrono::Local;
use log::info;
use log4rs;

fn connect(ip: &str) {
    // connect to remote ip with port

    if let Ok( stream) = TcpStream::connect(ip) {
        // let now = Local::now();
        // let _timestamp = format!("{}.{:06}", now.format("%Y-%m-%d %H:%M:%S"),
                                //  now.timestamp_subsec_micros());
        info!("ok: {}:{} --> {}",
                 stream.local_addr().unwrap().ip(),
                 stream.local_addr().unwrap().port(),
                 ip);
        // info!("ok test {_timestamp}");
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        // let now = Local::now();
        // let _timestamp = format!("{}.{:06}", now.format("%Y-%m-%d %H:%M:%S"),
                                //  now.timestamp_subsec_micros());
        info!("fail: src 0 --> {}",ip);
        // info!("fail test {_timestamp}");
    }

}
fn ip_list() {
    // send the tcp syn to remote ip address with thread
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
    // read the file content to ip list
    let path = std::fs::read_to_string(path).unwrap();
    let result: Vec<String> = path
        .lines()
        .into_iter()
        .map(move |ch|ch.to_string())
        .collect();
    result
}

fn main() {
    // todo: need add args parser
    let args: Vec<String>= env::args().collect();
    // read log4rs setting
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let cnt = args[1].parse::<i32>().unwrap();
    for _ in 0..cnt{
        sleep(Duration::from_secs(1));
        ip_list();
        // info!("********");
    }
}
