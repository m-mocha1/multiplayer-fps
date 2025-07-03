mod map;
use map::make_map;

fn main() {
    let map = make_map(10, 10);
    println!("{:#?}", map)
}
