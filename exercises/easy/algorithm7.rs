/*
	stack
	This question requires you to use a stack to achieve a bracket match
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
	fn bracket_matching_1() {
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_2() {
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_3() {
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_4() {
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_5() {
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_6() {
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}