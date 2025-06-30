use std::io;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Token {
    Num(f64),
    Op(char),
}

#[derive(Default)]
struct Operation {
    value: Option<Token>,
    left_op: Option<Box<Operation>>,
    right_op: Option<Box<Operation>>,
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => println!("{error}"),
        Ok(_) => {
            let mut tokens: Vec<Token> = Vec::new();
            in2postfix(&input, &mut tokens);
            let mut main_op: Operation = Default::default();
            build_operation_tree(&mut tokens, &mut main_op);
            println!("{}", calc_tree(&main_op));
        }
    }
    Ok(())
}

fn is_unary(_c: &char) -> bool { false } // not implemented yet

fn get_precedence(op: char) -> u8 {
    match op {
        '+' | '-' | '~' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        '!' => 4,
        'r' | 'R' | 'a' | 's' | 'c' | 't' | 'l' | 'L' => 5,
        _ => 0
    }
}

fn omit_mult(before: char, stack: &mut Vec<char>, result: &mut Vec<Token>) {
    if before.is_ascii_digit() || before == ')' || before == '!' || before == 'x' || before == '.' {
        while !stack.is_empty() && get_precedence('*') <= get_precedence(*stack.last().unwrap()) {
            result.push(Token::Op(stack.pop().unwrap()));
        }
        stack.push('*');
    }
}

fn in2postfix(infix: &str, result: &mut Vec<Token>) {
    let mut stack: Vec<char> = Vec::new();

    let regex = Regex::new(r"(?<num>[0-9\.]+)|(?<op>[\+\-\*/\(\)])").unwrap();
    let tokens = regex.captures_iter(infix);

    let mut before: char = ' ';

    for tok in tokens {
        if let Some(num) = tok.name("num") { 
            result.push(Token::Num(num.as_str().parse().expect("couldn't parse number!")));
        }
        if let Some(op) = tok.name("op") { 
            match op.as_str() {
                "(" => {
                    omit_mult(before, &mut stack, result);
                    stack.push('(');
                },
                ")" => {
                    while !stack.is_empty() && *stack.last().unwrap() != '(' {
                        result.push(Token::Op(stack.pop().unwrap()));
                    }
                    stack.pop(); // pop '('
                },
                d => {
                    while !stack.is_empty() && get_precedence(d.chars().next().unwrap()) <= get_precedence(*stack.last().unwrap()) {
                        result.push(Token::Op(stack.pop().unwrap()));
                    }
                    stack.push(d.chars().next().unwrap());
                }
            }
        }
        before = tok.get(0).unwrap().as_str().chars().next().unwrap();
    }
    stack.into_iter().rev().for_each(|t| result.push(Token::Op(t)));
}

fn build_operation_tree(postfix: &mut Vec<Token>, root: &mut Operation) {
    if postfix.is_empty() { return; }
    root.value = postfix.pop();
    if let Token::Num(_) = root.value.as_ref().unwrap() { return; }
    root.right_op = Some(Box::new(Operation {..Default::default()}));
    build_operation_tree(postfix, root.right_op.as_mut().unwrap());
    if let Token::Op(op) = root.value.as_ref().unwrap() { 
        if !is_unary(op) {
            root.left_op = Some(Box::new(Operation {..Default::default()}));
            build_operation_tree(postfix, root.left_op.as_mut().unwrap());
        }
    }
}

fn calc_tree(root: &Operation) -> f64 {
    if let Token::Num(num) = root.value.as_ref().unwrap() { return *num; }

    let right_result = calc_tree(root.right_op.as_ref().unwrap());
    let left_result = match root.left_op.is_some() {
        true => calc_tree(root.left_op.as_ref().unwrap()),
        false => 0.0
    };

    let op: char;
    if let Token::Op(x) = root.value.as_ref().unwrap() { op = *x; } else { op = ' '; };
    match op {
        '+' => left_result + right_result,
        '-' => left_result - right_result,
        '*' => left_result * right_result,
        '/' => left_result / right_result,
        _ => 0.0
    }
}
