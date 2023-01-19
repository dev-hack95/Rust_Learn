fn main() {
    println!("{number:>5}", number=2);
    println!("{number:0<5}", number=3);
    println!("{number:0>width$}", number=4, width=5);
}
