fn another_function() {
    println!("Another function.");
}

fn main() {
    let x = another_function();
    println!("function.{x:?}");
}

