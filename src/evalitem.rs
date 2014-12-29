use std::fmt;

#[deriving(Clone)]
pub enum EvalItem {
    List(Vec<EvalItem>),
    Value(String),
    Lambda,
    Empty,
}

impl fmt::Show for EvalItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List(ref n) => write!(f, "[{}]", n),
            Value(ref n) => write!(f, "<{}>", *n),
            Lambda => write!(f, "Lambda"),
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
            Lambda => {
                match *other {
                    Lambda => true,                    
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
