fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; //this line works fine
    let z = &mut x; // this line causes an error
    *z = 15;
}