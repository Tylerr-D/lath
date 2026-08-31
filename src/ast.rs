use crate::parser::{CalcParser, Rule};
use pest::Parser;


pub struct Eval;



#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(Debug)]

pub enum Node {
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

    match pair.as_rule(){

        Rule::Expr | Rule::Term => {
            let inner = pair.into_inner().next().unwrap();
            build_ast_from_expr(inner)
        }

        Rule::Int => {
            let n = pair.as_str().parse::<i32>().unwrap();
            Node::Int(n)
        }

        Rule::UnaryExpr => {

            let mut pairs = pair.into_inner();
            let op_pair = pairs.next().unwrap();
            let child_pair = pairs.next().unwrap();

            let op = match op_pair.as_str() {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                _ => unreachable!(),
            };


            Node::UnaryExpr {
                op,
                child: Box::new(build_ast_from_expr(child_pair)),
            }
        }

        Rule::BinaryExpr => {
            let mut pairs = pair.into_inner();


                  let mut lhs = build_ast_from_expr(pairs.next().unwrap());

            // then keep folding in (operator, term) pairs left to right
            while let Some(op_pair) = pairs.next() {
                let op = match op_pair.as_str() {
                    "+" => Operator::Plus,
                    "-" => Operator::Minus,
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
        _=> unreachable!(),
    }
  
}

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    
    let mut ast = vec![];

    let pairs = CalcParser::parse(Rule::Program, source)?;
    
    // it checks each and makes coverts it into ast
    for pair in pairs {
        if let Rule::Expr = pair.as_rule(){
                        ast.push(build_ast_from_expr(pair));
        }
    } 
//
    Ok(ast)
}

impl Eval{

    pub fn new() -> Self {
    Eval
}

pub fn eval(&self, node: &Node) -> i32 {
    match node {
        Node::Int(n) => *n,
        Node::UnaryExpr {op, child} => {
            let child = self.eval(child);

            match op {

                // this lowk is my favourite part of code till now, i mean why dont just keep this place this simple
                // this is want we want to become better
                // this is simplicity, this is the truth  
                Operator::Plus => child,
                Operator::Minus => -child,
            }
        }
        

        Node::BinaryExpr {op, lhs, rhs} => {

        // this i good code too yk, taking stuff from the right side and assigning it.  taking stuff from the left side and assigning it
            let lhs_ret = self.eval(lhs);
            let rhs_ret = self.eval(rhs);

            match op {
                Operator::Plus => lhs_ret + rhs_ret,
                Operator::Minus => lhs_ret - rhs_ret,
            }
        }


    }
}

}