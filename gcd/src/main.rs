use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing arguments"));
    }

    if numbers.len() == 0 {
        eprintln!("Arguments not specified");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("Greatest common devisor of {:?} is {}", numbers, d);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // swap
            let t = m;
            m = n;
            n = t;
        }

        m = m%n;
    }
    n
}