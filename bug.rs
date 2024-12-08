fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(10).unwrap();
    println!("The value is: {}", number);
}