#![feature(slice_strip)]
use std::convert::TryInto;
use std::io;
use std::io::BufRead;

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().collect();
    println!(
        "{}",
        lines
            .iter()
            .map(|l| eval(&l.as_ref().unwrap()))
            .sum::<usize>()
    );
    println!(
        "{}",
        lines
            .iter()
            .map(|l| eval2(&l.as_ref().unwrap()))
            .sum::<usize>()
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(usize),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

fn parse2(acc: Option<usize>, tokens: &[Token]) -> Option<usize> {
    return match (acc, tokens) {
        (Some(x), []) => Some(x),
        (None, [Token::Number(x)]) => Some(*x),
        (None, [Token::Number(x), rest @ ..]) => parse2(Some(*x), rest),
        (_, [Token::OpenParen, rest @ ..]) => {
            let index = match match_paren(tokens) {
                Some(i) => i,
                None => panic!("failed to match_paren: {:?}", tokens),
            };
            let (paren, actual_rest) = rest.split_at(index);
            return parse2(parse2(acc, &paren[..paren.len() - 1]), actual_rest);
        }
        (Some(a), [Token::Mul, rest @ ..]) => match parse2(None, rest) {
            Some(result) => Some(a * result),
            None => panic!("Failed to parse mul: {:?}", rest),
        },
        (Some(a), [Token::Add, Token::Number(x), rest @ ..]) => parse2(Some(a + x), rest),
        (Some(a), [Token::Add, Token::OpenParen, rest @ ..]) => {
            let index = match match_paren(&tokens[1..]) {
                Some(i) => i,
                None => panic!("failed to match_oaren: {:?}", &tokens[1..]),
            };
            let (paren, actual_rest) = rest.split_at(index);
            let inner = match parse2(None, &paren[..paren.len() - 1]) {
                Some(i) => i,
                None => panic!("failed to parse inner: {:?}", paren),
            };
            return parse2(Some(a + inner), actual_rest);
        }
        x => panic!("unknown: {:?}", x),
    };
}

fn parse(acc: Option<usize>, tokens: &[Token]) -> Option<usize> {
    return match (acc, tokens) {
        (Some(x), []) => Some(x),
        (None, [Token::Number(x)]) => Some(*x),
        (None, [Token::Number(x), rest @ ..]) => parse(Some(*x), rest),
        (_, [Token::OpenParen, rest @ ..]) => {
            let index = match match_paren(tokens) {
                Some(i) => i,
                None => panic!("failed to match_paren: {:?}", tokens),
            };
            let (paren, actual_rest) = rest.split_at(index);
            return parse(parse(acc, &paren[..paren.len() - 1]), actual_rest);
        }
        (Some(a), [operator, Token::Number(x), rest @ ..]) => {
            parse(Some(match_operator(*operator, a, *x)), rest)
        }
        (Some(a), [operator, Token::OpenParen, rest @ ..]) => {
            let index = match match_paren(&tokens[1..]) {
                Some(i) => i,
                None => panic!("failed to match_oaren: {:?}", &tokens[1..]),
            };
            let (paren, actual_rest) = rest.split_at(index);
            let inner = match parse(None, &paren[..paren.len() - 1]) {
                Some(i) => i,
                None => panic!("failed to parse inner: {:?}", paren),
            };
            return parse(Some(match_operator(*operator, a, inner)), actual_rest);
        }
        x => panic!("unknown: {:?}", x),
    };
}

fn match_operator(operator: Token, a: usize, b: usize) -> usize {
    return match operator {
        Token::Add => a + b,
        Token::Mul => a * b,
        _ => panic!("Not an operator: {:?}", operator),
    };
}

fn match_paren(tokens: &[Token]) -> Option<usize> {
    let mut depth = 0;
    for i in 0..tokens.len() {
        match tokens[i] {
            Token::OpenParen => {
                depth += 1;
            }
            Token::CloseParen => {
                depth -= 1;
            }
            _ => continue,
        }
        if depth == 0 {
            return Some(i);
        }
    }
    return None;
}

fn eval(expr: &str) -> usize {
    return parse(None, &tokenize(expr)).unwrap();
}

fn eval2(expr: &str) -> usize {
    return parse2(None, &tokenize(expr)).unwrap();
}

fn tokenize(expr: &str) -> Vec<Token> {
    return expr
        .replace(" ", "")
        .chars()
        .map(|c| match c {
            '+' => Token::Add,
            '*' => Token::Mul,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            x => Token::Number(x.to_digit(10).unwrap().try_into().unwrap()),
        })
        .collect::<Vec<_>>();
}

#[test]
fn test_examples() {
    assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}

#[test]
fn test_examples2() {
    assert_eq!(eval2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        eval2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
    let _a = ((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2 + 4) * 2;
}
