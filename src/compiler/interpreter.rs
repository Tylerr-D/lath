pub struct Interpreter;

use crate::ast::Node;
use crate::compiler::Compile;
use crate::parser::Rule;

use crate::ast::Eval;

impl Compile for Interpreter {

    type Output = Result<i32, pest::error::Error<Rule>>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0i32;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_interpreter(){
                assert_eq!(Interpreter::from_source("1 + 2").unwrap(), 3);
    }
}