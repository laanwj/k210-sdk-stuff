use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use esp8266at::response::parse_response;

fn main() {
    let f = File::open("data/parses.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if l.len() >= 2 {
            let mut lb = l[2..].as_bytes().to_vec();
            lb.push(13);
            lb.push(10);
            let res = parse_response(&lb);
            match res {
                Err(x) => {
                    println!("failed command was: {}", l);
                    println!("{:?}", x);
                }
                Ok((res, x)) => {
                    if res.is_empty() {
                        println!("{:?}", x);
                    } else {
                        println!("non-empty residue command was: {}", l);
                        println!("{:?} {:?}", res, x);
                    }
                }
            }
        }
    }
}
