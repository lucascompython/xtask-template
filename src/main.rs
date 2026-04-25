fn main() {
    println!("Hello, world!\nGot the following arguments:");
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
    }
}
