use std::fmt;

enum EvalItem {
    List(Vec<EvalItem>),
    Value(String),
    Lambda,
    Empty,
}

impl fmt::Show for EvalItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List(ref n) => write!(f, "List"),
            Value(ref n) => write!(f, "Value {}", *n),
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
                    List(ref o) => {
                        n == o
                    },
                    _ => false,
                }
            },
            Value(ref n) => {
                match *other {
                    Value(ref o) => {
                        n == o
                    },
                    _ => false,
                }
            },
            Lambda => {
                match *other {
                    Lambda => {
                        true
                    },
                    _ => false,
                }
            },
            Empty => {
                match *other {
                    Empty => {
                        true
                    },
                    _ => false,
                }
            },
        }
    }
}

fn build_list(tokens: Vec<String>, index: int) -> EvalItem {
    if tokens.is_empty() { return Empty }

    let mut stack = vec![];
    let mut root: EvalItem = List(vec![]);
    stack.push(root);
    
    for token in tokens.iter() {
        let chars = token.as_slice();
        match chars {
            "(" => assert!(true),
            ")" => assert!(true),
            _ => {
                match stack.pop() {
                    Some(stackItem) => {
                        match stackItem {
                            List(mut listOld) => {
                                listOld.push(EvalItem::Value(chars.to_string()));
                                stack.push(List(listOld));
                            },
                            _ => panic!("Shouldn't happen!")
                        }
                    },
                    None => panic!("Shouldn't get here!"),
                }
            },            
        }
    }
    match stack.pop() {
        Some(n) => n,
        None => Empty,
    }
}

#[cfg(test)]
mod test {
    use super::build_list;
    use super::EvalItem;
    use tokenizer::tokenize;
    
    #[test]
    fn stuff() {
        let expectedList = vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::Value("hå".to_string()));
        let expr = "(hej hoj hå)".to_string();
        match build_list(tokenize(&expr), 0) {
            EvalItem::List(n) => {
                assert_eq!(expectedList, n);
            },
            _ => assert!(false),
        }
    }
}
