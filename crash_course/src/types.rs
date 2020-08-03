pub fn typesfile() {
    
    //Default 'i32'
    let x = 32;
    println!("Value of x is: {}", x);
    
    //Default 'f64'
    let y = 9.2;
    println!("Value of y is: {}", y);
    
    //Adding explicit type
    let z: i64 = 567893;
    println!("Value of z is: {}", z);

    //Finding max size
    println!("Max i8 = {} Max i32 = {} Max i64 = {}", std::i8::MAX, std::i32::MAX, std::i64::MAX);

}
