fn main() {
    // This will fail to compile until the crate exists and is published to crates.io.
    println!("Hello from sample-tool!");

    // After publishing, uncomment this:
    // println!("{}", does_not_exist_yet::hello());
}
