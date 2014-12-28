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

fn build_list(tokens: &Vec<String>, index_init: uint) -> (uint, EvalItem) {
    if tokens.is_empty() { return (0, Empty) }
    
    let mut item = vec![];

    let mut index = index_init;
    loop {
        if index >= tokens.len() { return (index, List(item)) }
        let token_chars = tokens[index].as_slice();
        match token_chars {
            "(" => {
                let (index, new_item) = build_list(tokens, index + 1);
            },
            ")" => {                
                return (index, List(item))
            },
            _ => {
                item.push(Value(token_chars.to_string()));
            },
        }
        index = index + 1;
    }
    (0, Empty)
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
        let (index, item) = build_list(&tokenize(&expr), 0);
        match item {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }
}
