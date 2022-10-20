fn main() {
    println!("Hello, world!");

    println!("Gcd of 5 and 7 is {}", gcd(5, 7));
}

fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    n
}
