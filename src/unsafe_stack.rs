#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<*mut Box<Node<T>>>
}

impl<T> Node<T> {
    fn new_without_next(value: T) -> Self {
        Node {
            value: value,
            next: None
        }
    }

    fn new_with_next(value: T, next: *mut Box<Node<T>>) -> Self {
        Node {
            value: value,
            next: Some(next)
        }
    }
}
#[derive(Debug, Clone)]
pub struct Stack<T: Clone> {
    head: Option<*mut Box<Node<T>>>
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None
        }
    }

    fn new_with_node_value(value: T, next: *mut Box<Node<T>>) -> Option<*mut Box<Node<T>>> {
        let node = Box::new(Box::new(Node::new_with_next(value, next)));
        let node_ptr = Box::into_raw(node);
        Some(node_ptr)
    }
    fn new_with_node(value: T) -> Option<*mut Box<Node<T>>> {
        let node = Box::new(Box::new(Node::new_without_next(value)));
        let node_ptr = Box::into_raw(node);
        Some(node_ptr)
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Stack::new_with_node(value);
        } else {
            let next = self.head.unwrap() as *mut Box<Node<T>>;
            let head = Stack::new_with_node_value(value, next);
            self.head = head;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() { return None; }

        let head_ptr: *mut Box<Node<T>> = self.head.unwrap();
        unsafe {
            let value = &head_ptr.as_ref().unwrap().value;
            if let Some(next_ptr) = head_ptr.as_ref().unwrap().next {
                self.head = Some(next_ptr);
            } else {
                self.head = None;
            }
            return Some(value.clone());
        }    
    }

    pub fn peek(&self) -> Option<T> {
        if self.head.is_none() { return None; }
        let head_ptr = self.head.unwrap();
        unsafe {
            let value = &head_ptr.as_ref().unwrap().value;
            return Some(value.clone());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_push_node() {
    //   //  println!("inside push test");
    //     let mut stack: Stack<u8> = Stack::new();
    //     stack.push(45);
    //     stack.push(46);
    //     let ptr: *mut Box<Node<u8>> = stack.head.unwrap();
    //     unsafe {
    //         println!("pushed value {:?}",ptr.as_ref().unwrap().value);
    //         let next_ptr = ptr.as_ref().unwrap().next.unwrap();
    //         println!("pushed value {:?}",next_ptr.as_ref().unwrap().value);
    //     }
    //  //   println!("end of push test");  
    // }

    // #[test]
    // fn test_pop_node() {
    //  //   println!("inside pop test");
    //     let mut stack: Stack<u8> = Stack::new();
    //     stack.push(3);
    //     stack.push(5);
    //     stack.push(7);
    //     println!("pop {:?}",stack.pop());
    //     println!("pop {:?}",stack.pop());
    //     println!("pop {:?}",stack.pop());
    //     println!("pop {:?}",stack.pop());
    //   //  println!("end of pop test");
    // }

    // #[test]
    // fn test_peek_node() {
    //  //   println!("inside peek test");
    //     let mut stack: Stack<u8> = Stack::new();
    //     stack.push(3);
    //     stack.push(5);
    //     stack.push(7);
    //     println!("peek {:?}",stack.peek());
    //     stack.pop();
    //     println!("peek {:?}",stack.peek());
    //     stack.pop();
    //     println!("peek {:?}",stack.peek());
    //     stack.pop();
    //     println!("peek {:?}",stack.peek());
    //  //   println!("end of peek test");
    // }

    // #[test]
    // fn test_is_empty() {
    //     let mut stack: Stack<u8> = Stack::new();
    //     println!("stack empty: {}", stack.is_empty());
    //     stack.push(45);
    //     println!("stack empty: {}", stack.is_empty());
    //     stack.pop();
    //     println!("stack empty: {}", stack.is_empty());
    // }
}