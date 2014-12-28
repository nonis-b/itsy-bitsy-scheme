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

fn build_list(tokens: Vec<String>) -> EvalItem {
    if tokens.is_empty() { return Empty }

    let mut stack = vec![];
    let root: EvalItem = List(vec![]);
    stack.push(root);
    
    for token in tokens.iter() {
        let chars = token.as_slice();
        match chars {
            "(" => assert!(true),
            ")" => assert!(true),
            _ => {
                match stack.pop() {
                    Some(stack_item) => {
                        match stack_item {
                            List(mut list) => {
                                list.push(EvalItem::Value(chars.to_string()));
                                stack.push(List(list));
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
        let expected_list = vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::Value("hå".to_string()));
        let expr = "(hej hoj hå)".to_string();
        match build_list(tokenize(&expr)) {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }
}
