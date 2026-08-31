pub enum Operator {
    Plus,
    Minus,
}

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

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    
    let mut ast = vec![];

    let pairs = CalcParser::parse(Rule::Program, source)?;
    
    // it checks each and makes coverts it into ast
    for pair in pairs {
        if let Rule::Expr = pair.as_rule(){
                        ast.push(build_ast_from_expr(pair));

        }
    } 

    Ok(ast)
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
                Operator::Plus => lhs_ret - rhs_ret,
            }
        }


    }
}

