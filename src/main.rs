mod core;

use crate::core::tl::Tl;
use crate::core::dp::DP;

fn main() {
    let s = DP::new();
    s.feed(|data: &Vec<u8>| {
        let t = Tl::new(data);
        t._do();
    });

    println!();
}
