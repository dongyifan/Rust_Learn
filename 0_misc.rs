fn main() {
    let mut b = 3;
    let a  = &mut b;
    *a += 10;
    println!("{}", a);
    println!("{}", b);
}