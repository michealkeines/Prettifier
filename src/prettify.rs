use crate::unsafe_stack::Stack;
use std::collections::HashMap;

pub struct Prettify {
    openning_map: HashMap<char, char>,
    closing_map: HashMap<char, char>
}

impl Prettify {
    pub fn new() -> Self {
        let mut close = HashMap::new();
        close.insert('}', '{');
        close.insert(')', '(');
        close.insert('>', '<');
        close.insert(']', '[');

        let mut open = HashMap::new();
        open.insert('{', '}');
        open.insert('(', ')');
        open.insert('<', '>');
        open.insert('[', ']');

        Prettify {
            openning_map: open,
            closing_map: close
        }
    }
    fn is_a_special_char_closing(&self, value: char) -> bool {
        self.closing_map.contains_key(&value)
    }

    fn is_a_special_char_openning(&self, value: char) -> bool {
        self.openning_map.contains_key(&value)
    }
    // { (a+b) && (a+c) }
    // { (())(] }
        // {{}}
    fn update(&self, c: char, i: usize, list: &Vec<u8>) -> Vec<u8>{
        let mut temp: Vec<u8> = vec![];
        // { (())(] }
        if self.is_a_special_char_closing(list[i] as char) {                   
            // [
            let map = *self.closing_map.get(&(list[i] as char)).unwrap()  as u8;
            let mut t = list.clone();
            if list.len() >= 2 {
                t.insert(i, map);
            } else {
                t.insert(0, map);
            }
            temp = t;
        } else {  // { (())([] }
            // )
            let map = *self.closing_map.get(&(list[i] as char)).unwrap()  as u8;
            let mut t = list.clone();
            if list.len() >= 2 {
                t.insert(i, map);
            } else {
                t.insert(0, map);
            }
            temp = t;
        }

        temp
    }
    
    pub fn prettify(&self, input: String) -> String {
        let mut bytes = input.as_bytes().to_vec();
        'test: while !self.check_equality(String::from_utf8(bytes.to_vec()).unwrap()) {
            //let vec = vec![];
            let mut stack = Stack::new();
           // let chars = input.as_bytes();
            for i in 0..bytes.len() {
                let c = bytes[i];
                if self.is_a_special_char_openning(c as char) {
                    stack.push(c);
                } 
                else if self.is_a_special_char_closing(c as char) {
                    // Some('{')
                    if let Some(current_value) = stack.peek() {
                        // Some('{') == 
                        let mapped = self.closing_map.get(&(c as char)).unwrap();
                        if current_value as char == *mapped {
                            let _ = stack.pop();
                            continue;
                        } else {
                            // TODO: what happens if a peek value is not map of closing value
                            // here
                            let temp = self.update(current_value as char, i, &bytes);
                            println!("{:?}",String::from_utf8(temp.clone()));
                            bytes = temp;
                            break 'test;
                        }
                    } else {
                        // here
                        let temp = self.update(c as char, i, &bytes);
                        bytes = temp;
                        break 'test;
                    }          
                } 
                else {
                    continue;
                }               
            }
            let mut count = 0;
            while !stack.is_empty() {

            }
        }
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    pub fn check_equality(&self, input: String) -> bool {
        let mut stack = Stack::new();
        let chars = input.as_bytes();

        for c in chars {
            if self.is_a_special_char_openning(*c as char) {
                stack.push(*c);
            } else if self.is_a_special_char_closing(*c as char) {
                // Some('{')
                if let Some(current_value) = stack.peek() {
                    // Some('{') == 
                    let mapped = self.closing_map.get(&(*c as char)).unwrap();
                    if current_value as char == *mapped {
                        let _ = stack.pop();
                        continue;
                    } else {
                        // TODO: what happens if a peek value is not map of closing value
                        return false;
                    }
                } else {
                    return false;
                }          
            } else {
                continue;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_check() {
    //     let pretty = Prettify::new();  
    //     println!("is even {}",pretty.check_equality("{}}".to_string()));
    //     println!("is even {}",pretty.check_equality("{{}}".to_string()));
    //     println!("is even {}",pretty.check_equality("{ (())(] }".to_string()));
    //     println!("is even {}",pretty.check_equality("{ (a+b) && (a+c) }".to_string()));
    // }

    #[test] 
    fn test_pretify() {
        let prettify = Prettify::new();
        println!("is even {}",prettify.prettify("{ (())(] }".to_string()));
        println!("is even {}",prettify.prettify("(((".to_string()));
    }
}