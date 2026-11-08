// Homework 2 — Operators and a stack for the calculator
// Author: Berk Kirmizigul

// ---------------------------------------------------------------
// Part 1 — Operators
// ---------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// 1. Character -> operator ('+', '-', '*', '/'), otherwise None.
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    /// 2. Operator -> character.
    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    /// 3. 1 for + and -, 2 for * and /.
    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    /// 4. Applies the operator; None when dividing by zero.
    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

/// 5. One line, using `?`.
fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
    Op::from_char(op)?.apply(a, b)
}

// ---------------------------------------------------------------
// Part 2 — A stack for numbers
// ---------------------------------------------------------------

#[derive(Debug)]
struct Stack {
    items: Vec<f64>,
}

impl Stack {
    /// 6. Creates an empty stack.
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// 6. Puts a value on top.
    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    /// 6. Removes and returns the top value.
    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    /// 7. Returns a reference to the top value without removing it.
    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    /// 7. Number of values on the stack.
    fn len(&self) -> usize {
        self.items.len()
    }

    /// 8 + 9. Pops b, then a, and pushes op.apply(a, b).
    /// If there are fewer than two values or the operation fails,
    /// returns None and leaves the stack unchanged.
    fn apply(&mut self, op: Op) -> Option<()> {
        let n = self.items.len();
        if n < 2 {
            return None;
        }
        let a = self.items[n - 2];
        let b = self.items[n - 1];
        // Compute first: if this fails, `?` returns None
        // before anything has been removed from the stack.
        let result = op.apply(a, b)?;
        self.pop();
        self.pop();
        self.push(result);
        Some(())
    }
}

// ---------------------------------------------------------------
// 10. (2 + 3) * 4 - 1 using only push and apply
// ---------------------------------------------------------------

fn main() {
    // Part 1 examples
    println!("from_char('*') = {:?}", Op::from_char('*'));
    println!("from_char('x') = {:?}", Op::from_char('x'));
    println!("Div.symbol()   = {}", Op::Div.symbol());
    println!("precedence: + = {}, * = {}", Op::Add.precedence(), Op::Mul.precedence());
    println!("Sub.apply(5,3) = {:?}", Op::Sub.apply(5.0, 3.0));
    println!("Div.apply(1,0) = {:?}", Op::Div.apply(1.0, 0.0));
    println!("evaluate_simple(6, '/', 4) = {:?}", evaluate_simple(6.0, '/', 4.0));
    println!();

    // Task 10
    let mut stack = Stack::new();
    stack.push(2.0);
    stack.push(3.0);
    stack.apply(Op::Add); // 5
    stack.push(4.0);
    stack.apply(Op::Mul); // 20
    stack.push(1.0);
    stack.apply(Op::Sub); // 19

    println!("(2 + 3) * 4 - 1 = {:?} (len = {})", stack.peek(), stack.len());
    while let Some(v) = stack.pop() {
        println!("{}", v);
    }
    println!("stack is now empty: len = {}", stack.len());
}

// ---------------------------------------------------------------
// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1_from_char() {
        assert_eq!(Op::from_char('+'), Some(Op::Add));
        assert_eq!(Op::from_char('-'), Some(Op::Sub));
        assert_eq!(Op::from_char('*'), Some(Op::Mul));
        assert_eq!(Op::from_char('/'), Some(Op::Div));
        assert_eq!(Op::from_char('x'), None);
    }

    #[test]
    fn t2_symbol() {
        assert_eq!(Op::Div.symbol(), '/');
        for op in [Op::Add, Op::Sub, Op::Mul, Op::Div] {
            assert_eq!(Op::from_char(op.symbol()), Some(op));
        }
    }

    #[test]
    fn t3_precedence() {
        assert_eq!(Op::Add.precedence(), 1);
        assert_eq!(Op::Sub.precedence(), 1);
        assert_eq!(Op::Mul.precedence(), 2);
        assert_eq!(Op::Div.precedence(), 2);
    }

    #[test]
    fn t4_apply() {
        assert_eq!(Op::Sub.apply(5.0, 3.0), Some(2.0));
        assert_eq!(Op::Div.apply(1.0, 0.0), None);
    }

    #[test]
    fn t5_evaluate_simple() {
        assert_eq!(evaluate_simple(6.0, '/', 4.0), Some(1.5));
        assert_eq!(evaluate_simple(1.0, '/', 0.0), None);
        assert_eq!(evaluate_simple(1.0, '?', 2.0), None);
    }

    #[test]
    fn t6_push_pop() {
        let mut s = Stack::new();
        s.push(1.0);
        s.push(2.0);
        assert_eq!(s.pop(), Some(2.0));
        assert_eq!(s.pop(), Some(1.0));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn t7_peek_len() {
        let mut s = Stack::new();
        assert_eq!(s.peek(), None);
        assert_eq!(s.len(), 0);
        s.push(1.0);
        s.push(2.0);
        assert_eq!(s.peek(), Some(&2.0));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn t8_stack_apply() {
        let mut s = Stack::new();
        s.push(10.0);
        s.push(4.0);
        assert_eq!(s.apply(Op::Sub), Some(()));
        assert_eq!(s.items, vec![6.0]);
    }

    #[test]
    fn t9_stack_apply_failure() {
        let mut s = Stack::new();
        s.push(5.0);
        assert_eq!(s.apply(Op::Add), None);
        assert_eq!(s.items, vec![5.0]);

        let mut s = Stack::new();
        s.push(1.0);
        s.push(0.0);
        assert_eq!(s.apply(Op::Div), None);
        assert_eq!(s.items, vec![1.0, 0.0]);
    }

    #[test]
    fn t10_expression() {
        let mut s = Stack::new();
        s.push(2.0);
        s.push(3.0);
        s.apply(Op::Add);
        s.push(4.0);
        s.apply(Op::Mul);
        s.push(1.0);
        s.apply(Op::Sub);
        assert_eq!(s.peek(), Some(&19.0));
        let mut popped = Vec::new();
        while let Some(v) = s.pop() {
            popped.push(v);
        }
        assert_eq!(popped, vec![19.0]);
        assert_eq!(s.len(), 0);
    }
}
