fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    *y = 15; // Modified to use single mutable reference
}