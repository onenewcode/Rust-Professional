/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.last()
        }
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();

    // Define the matching pairs
    let matching_pairs = |open: char, close: char| -> bool {
        match open {
            '(' => close == ')',
            '{' => close == '}',
            '[' => close == ']',
            _ => false,
        }
    };

    for c in bracket.chars() {
        // If it's an opening bracket, push it to the stack
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);
        }
        // If it's a closing bracket, check if it matches the top of the stack
        else if c == ')' || c == '}' || c == ']' {
            if let Some(top) = stack.pop() {
                if !matching_pairs(top, c) {
                    return false; // If there's no match, return false
                }
            } else {
                return false; // If the stack is empty but a closing bracket is found
            }
        }
    }

    // If the stack is empty at the end, all brackets matched
    stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}