use std::env;

fn main() {
    for i in (0..21).map(baum_sweet) {
        println!("{}", i)
    }
}

fn baum_sweet(x: u64) -> u8 {
    let mut bits = x;

    loop {
        if bits == 0 {
            return 1;
        };
        let n = bits.trailing_zeros();
        if n % 2 == 1 {
            return 0;
        };
        bits >>= n + 1;
    }
}
