fn main() {
    println!("Hello from Rust!");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    
    println!("Original numbers: {:?}", numbers);
    println!("Doubled numbers: {:?}", doubled);
} 