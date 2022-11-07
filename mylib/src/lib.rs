#[no_mangle]
pub fn hello() {
    println!("Hello");
}

#[no_mangle]
pub fn return_value() -> u32 {
    155
}
