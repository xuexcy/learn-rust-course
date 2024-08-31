use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("Hello, world! {:?}", mix(blue, yellow));
}
