use prettifier::unsafe_stack::Stack;

fn main() {
    let mut stack: Stack<u8> = Stack::new();

    stack.push(45);
    println!("peek value: {}", stack.peek().unwrap());

}
