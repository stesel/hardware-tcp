use std::thread::spawn;

mod button;

/// Entry point
fn main() {
    let t = spawn(move || {
        button::create_connection("9999");
    });
    match t.join() {
        Ok(result) => println!("Thread result {:?}", result),
        Err(error) => println!("Thread error {:?}", error),
    }
}
