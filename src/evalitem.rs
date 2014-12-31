use environment::Environment;
use std::fmt;

#[deriving(Clone)]
pub struct LambdaDefinition {
    pub arguments: Vec<EvalItem>,
    pub body: EvalItem,
    pub environment: Box<Environment>,
}

#[deriving(Clone)]
pub enum EvalItem {
    List(Vec<EvalItem>),
    Value(String),
    Lambda(Box<LambdaDefinition>),
    Empty,
}

impl fmt::Show for EvalItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List(ref n) => write!(f, "[{}]", n),
            Value(ref n) => write!(f, "<{}>", *n),
            Lambda(ref n) => write!(f, "Lambda"),
            Empty => write!(f, "Empty"),
        }
    }
}

impl PartialEq for EvalItem {
    fn eq(&self, other: &EvalItem) -> bool {
        match *self {
            List(ref n) => {
                match *other {
                    List(ref o) => n == o,                    
                    _ => false,
                }
            },
            Value(ref n) => {
                match *other {
                    Value(ref o) => n == o,                    
                    _ => false,
                }
            },
            Lambda(ref lambda) => {
                match *other {
                    Lambda(ref other_lambda) => {
                        if lambda.arguments != other_lambda.arguments { return false; }
                        if lambda.body != other_lambda.body { return false; }
                        if lambda.environment != other_lambda.environment { return false; }
                        true
                    },                    
                    _ => false,
                }
            },
            Empty => {
                match *other {
                    Empty => true,                    
                    _ => false,
                }
            },
        }
    }
}
