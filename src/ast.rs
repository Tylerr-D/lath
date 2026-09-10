use crate::parser::{CalcParser, Rule};
use pest::Parser;

// hashmap lowk got aura
use std::collections::HashMap;

pub struct Eval {
    env: HashMap<String, i32>,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Node {

    Ident(String),

    Let {
        name:String,
        value:Box<Node>
    },

    Assign {
        name: String,
        value: Box<Node>,
    },

    Int(i32),

    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },

    BinaryExpr {

        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,

    },
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {

         match pair.as_rule() {
        Rule::Expr | Rule::Term => {


            let inner = pair.into_inner().next().unwrap();
            build_ast_from_expr(inner)


        }

        Rule::Ident => {
            Node::Ident(pair.as_str().to_string())
        }

        Rule::Int => {
            
            let n = pair.as_str().parse::<i32>().unwrap();
            Node::Int(n)
            
        }

        Rule::UnaryExpr => {

        let mut pairs = pair.into_inner();

let first = pairs.next().unwrap();

            if first.as_rule() == Rule::UnaryOp {

                let op = match first.as_str() {

                    "+" => Operator::Plus,
                    "-" => Operator::Minus,


                    _ => unreachable!(),
                };

                let child_pair = pairs.next().unwrap();

                Node::UnaryExpr {
                    op,
                    child: Box::new(build_ast_from_expr(child_pair)),
                }
            } else {
                build_ast_from_expr(first)
            }
        }
        Rule::AddSub | Rule::MulDiv => {
            let mut pairs = pair.into_inner();

            let mut lhs = build_ast_from_expr(pairs.next().unwrap());

            // then keep folding in (operator, term) pairs left to right
            while let Some(op_pair) = pairs.next() {

                let op = match op_pair.as_str() {


                    "+" => Operator::Plus,
                    "-" => Operator::Minus,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,


                    _ => unreachable!(),
                };
                let rhs_pair = pairs.next().unwrap();
                let rhs = build_ast_from_expr(rhs_pair);

                lhs = Node::BinaryExpr {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            lhs
        }
        _ => unreachable!(),
    }
}

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];

    let pairs = CalcParser::parse(Rule::Program, source)?;

    // it checks each and makes coverts it into ast
    for pair in pairs {

        if pair.as_rule() == Rule::Smt {
            let inner = pair.into_inner().next().unwrap();

            match inner.as_rule(){
                Rule::Let => astpush(build_binding(inner, true)),
                Rule::Assign => ast.push(build_binding(inner, false)),
                Rule::Expr => ast.push(build_ast_from_expr(inner)),

                _ => {}
            }
        }

    }
    
    Ok(ast)
}

fn build_binding(pair: pest::iterators::Pair<Rule>, is_let: bool()) -> Node {


    // gonna change let later, maybe 
    let mut inner = pair.into_inner().filter(|p| p.as_rule() != Rule::LetKw);

    let name = inner.next().unwrap().as_str().to_string();
    let value = build_ast_from_expr(inner.next().unwrap());

    if is_let {
        Node::Let (name, value: Box::new(value))
    }

    else {
        Node::Assign { name, value: Box::new(value) }
    }
}

impl Eval {
    pub fn new() -> Self {
        Eval
    }

    pub fn eval(&self, node: &Node) -> i32 {
        match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);

                match op {
                    // this lowk is my favourite part of code till now, i mean why dont just keep this place this simple
                    // this is want we want to become better
                    // this is simplicity, this is the truth
                    Operator::Plus => child,
                    Operator::Minus => -child,

                    _ => unreachable!(),
                }
            }

            Node::BinaryExpr { op, lhs, rhs } => {
                // this i good code too yk, taking stuff from the right side and assigning it.  taking stuff from the left side and assigning it
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Plus => lhs_ret + rhs_ret,
                    Operator::Minus => lhs_ret - rhs_ret,
                    Operator::Multiply => lhs_ret * rhs_ret,
                    Operator::Divide => lhs_ret / rhs_ret,
        }
                    }

                    Node::Ident(name) => {
                *self.env.get(name)
                    .unwrap_or_else(|| panic!("undefined variable: {}", name))
            }



            Node::Let { name, value } | Node::Assign { name, value } => {
                let v = self.eval(value);
                self.env.insert(name.clone(), v);
                v
            }

            
                        }
    }
}
