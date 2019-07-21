mod utils;

use utils::vector::Vec2;

fn main() {
    let v = Vec2::new(2, -2);
    let v2 = Vec2::new(3, 3);
    println!("{}", v + v2);
}
