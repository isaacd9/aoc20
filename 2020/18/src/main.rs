use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Mult,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    LParen,
    RParen,
    Op(Operator),
    Num(u64),
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Num(u64),
    Add,
    Mult,
    Paren,
}

fn lex_input(lines: Lines<StdinLock>) -> Vec<Vec<Token>> {
    use crate::Token::*;

    lines
        .map(|li| li.unwrap())
        .map(|li| {
            li.chars()
                .filter(|c| *c != ' ')
                .map(|c| match c {
                    '(' => LParen,
                    ')' => RParen,
                    '*' => Op(Operator::Mult),
                    '+' => Op(Operator::Add),
                    num if num.to_string().parse::<u64>().is_ok() => {
                        Num(num.to_string().parse::<u64>().unwrap())
                    }
                    _ => panic!("unexpected token: {}", c),
                })
                .collect()
        })
        .collect()
}

//fn parse_tokens(tokens: Vec<Token>) -> Expr {
//    //use crate::Expr::*;
//    use crate::Token::*;
//
//    let mut stack: Vec<Token> = vec![];
//
//    for tok in tokens {
//        match tok {
//        Num(n) => stack.push(tok),
//        Op(op) => {
//            let prev_tok = stack.pop();
//        }
//    }
//
//}

fn polish(tokens: &[Token]) -> Vec<Expr> {
    use crate::Token::*;

    //match (tokens[0], tokens[1], tokens[3]) {
    //    (Num(a), Op(op), Num(b)) => match op {
    //        Operator::Add => a + b,
    //        Operator::Mult => a * b,
    //    },
    //};

    //tokens[1..]
    //    .chunks(2)
    //    .enumerate()
    //    .fold(acc, |acc, (i, chunk)| {
    //        println!("{:?}", chunk);
    //        match (&chunk[0], &chunk[1]) {
    //            (Op(op), Num(n)) => match op {
    //                Operator::Add => acc + n,
    //                Operator::Mult => acc * n,
    //            },
    //            (Op(op), LParen) => {
    //                //println!("recursing with: {:?}", &tokens[i + 3..]);
    //                match op {
    //                    Operator::Add => acc + eval_expr(&tokens[i + 3..]),
    //                    Operator::Mult => acc * eval_expr(&tokens[i + 3..]),
    //                }
    //            }
    //            (RParen, _) => acc,
    //            _ => panic!("nonsense expr: {:?}", chunk),
    //        }
    //    })
    //
    let mut op_stack: Vec<Token> = vec![];
    let mut output_stack: Vec<Expr> = vec![];

    for tok in tokens {
        match tok {
            Num(n) => {
                output_stack.insert(0, Expr::Num(*n));
            }
            Op(op) => {
                op_stack.insert(0, tok.clone());
            }
            LParen => {
                op_stack.insert(0, tok.clone());
                output_stack.insert(0, Expr::Paren);
            }
            RParen => {
                let mut done = false;
                let mut num_popped = 0;
                while !done {
                    let popped = op_stack.remove(0);
                    //println!("popped {:?}", popped);
                    num_popped += 1;
                    //println!("num {:?}", popped);

                    match &popped {
                        Op(o) => {
                            let e = match o {
                                Operator::Add => Expr::Add,
                                Operator::Mult => Expr::Mult,
                            };

                            for i in 0..output_stack.len() {
                                if output_stack[i] == Expr::Paren {
                                    //println!("inserting {:?} to {:?}", popped, output_stack);
                                    output_stack.insert(i + 1, e);
                                    break;
                                }
                            }
                        }
                        LParen => {
                            for i in 0..output_stack.len() {
                                if output_stack[i] == Expr::Paren {
                                    //println!("inserting {:?} to {:?}", popped, output_stack);
                                    output_stack.remove(i);
                                    break;
                                }
                            }
                            done = true
                        }
                        RParen => (),
                        _ => panic!("unexpected item popped: {:?}", popped),
                    };
                }
            }
        };
    }

    while op_stack.len() > 0 {
        let o = op_stack.pop().unwrap();
        let e = match o {
            Op(Operator::Add) => Expr::Add,
            Op(Operator::Mult) => Expr::Mult,
            _ => panic!("unexpected iterm popped: {:?}", o),
        };
        output_stack.push(e)
    }

    //output_stack.reverse();

    output_stack
}

fn eval_polish(tokens: &[Expr]) -> u64 {
    use crate::Expr::*;
    //println!("{:?}", tokens);
    //match tokens[0] {
    //    Num(n) => n,
    //    Add => {
    //        let r = eval_polish(&tokens[1..tokens.len() - 1])
    //            + eval_polish(&tokens[tokens.len() - 1..]);
    //        println!("{}", r);
    //        r
    //    }
    //    Mult => {
    //        let r = eval_polish(&tokens[tokens.len() - 1..])
    //            * eval_polish(&tokens[1..tokens.len() - 1]);
    //        println!("{}", r);
    //        r
    //    }
    //    _ => panic!("unexpected"),
    //}
    //tokens.reverse();
    let mut stack: Vec<u64> = vec![];

    let mut sum: u64 = 0;
    for tok in tokens {
        match tok {
            Num(n) => stack.push(*n),
            Add => {
                let first = stack.pop();
                let second = stack.pop();

                //println!("{:?} + {:?}", first, second);

                let result = first.unwrap() + second.unwrap();
                stack.push(result);
            }
            Mult => {
                let first = stack.pop();
                let second = stack.pop();

                //println!("{:?} * {:?}", first, second);

                let result = first.unwrap() * second.unwrap();
                stack.push(result);
            }
            Paren => (),
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let exprs = lex_input(lines);

    //println!("{:?}", &polish(&exprs[0]));
    //println!("{:?}", &exprs[1]);
    //println!("{:?}", &polish(&exprs[0]));
    //println!("{:?}", eval_polish(&polish(&exprs[0])));
    //println!("{:?}", &polish(&exprs[1]));
    //println!("{:?}", eval_polish(&polish(&exprs[1])));
    //println!("{:?}", &polish(&exprs[2]));
    //println!("{:?}", &polish(&exprs[3]));
    //
    let mut sum = 0;
    for expr in exprs {
        let k = eval_polish(&polish(&expr));
        sum += k;
        println!("{:?}", k);
    }
    println!("{}", sum)
}
