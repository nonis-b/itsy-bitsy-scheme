use environment::Environment;
use std::fmt;

#[derive(Clone)]
pub struct LambdaDefinition {
    pub arguments: Vec<EvalItem>,
    pub body: EvalItem,
    pub environment: Box<Environment>,
}

#[derive(Clone)]
pub enum EvalItem {
    List(Vec<EvalItem>),
    Value(String),
    Lambda(Box<LambdaDefinition>),
    Empty,
}

impl fmt::Display for EvalItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalItem::List(ref n) => write!(f, "[{:?}]", n),
            EvalItem::Value(ref n) => write!(f, "<{}>", *n),
            EvalItem::Lambda(ref n) => write!(f, "Lambda"),
            EvalItem::Empty => write!(f, "Empty"),
        }
    }
}

impl fmt::Debug for EvalItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalItem::List(ref n) => write!(f, "[{:?}]", n),
            EvalItem::Value(ref n) => write!(f, "<{}>", *n),
            EvalItem::Lambda(ref n) => write!(f, "Lambda"),
            EvalItem::Empty => write!(f, "Empty"),
        }
    }
}

impl PartialEq for EvalItem {
    fn eq(&self, other: &EvalItem) -> bool {
        match *self {
            EvalItem::List(ref n) => {
                match *other {
                    EvalItem::List(ref o) => n == o,                    
                    _ => false,
                }
            },
            EvalItem::Value(ref n) => {
                match *other {
                    EvalItem::Value(ref o) => n == o,                    
                    _ => false,
                }
            },
            EvalItem::Lambda(ref lambda) => {
                match *other {
                    EvalItem::Lambda(ref other_lambda) => {
                        if lambda.arguments != other_lambda.arguments { return false; }
                        if lambda.body != other_lambda.body { return false; }
                        if lambda.environment != other_lambda.environment { return false; }
                        true
                    },                    
                    _ => false,
                }
            },
            EvalItem::Empty => {
                match *other {
                    EvalItem::Empty => true,                    
                    _ => false,
                }
            },
        }
    }
}
