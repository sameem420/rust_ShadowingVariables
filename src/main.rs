fn main() {
    let _number = 5;
    // shadowing the variable (same name)
    let _number = 15 * 4; 
    println!("The result after shadowing is : {}", _number);
    // shadowing the variable (same name but different type)
    let _number = "Sameem";
    println!("The result after shadowing is : {}", _number);
}
