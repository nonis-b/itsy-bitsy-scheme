use evalitem::EvalItem;

fn build_list(tokens: &Vec<String>, index_init: uint) -> (uint, EvalItem) {
    if tokens.is_empty() { return (0, EvalItem::Empty) }
    let mut item = vec![];
    let mut index = index_init;
    loop {
        if index >= tokens.len() { return (index, EvalItem::List(item)) }
        let token_chars = tokens[index].as_slice();
        match token_chars {
            "(" => {
                let (index_updated, new_item) = build_list(tokens, index + 1);
                index = index_updated;
                item.push(new_item);
            },
            ")" => {                
                return (index, EvalItem::List(item))
            },
            _ => {
                item.push(EvalItem::Value(token_chars.to_string()));
            },
        }
        index = index + 1;
    }
    (0, EvalItem::Empty)
}

#[cfg(test)]
mod test {
    use super::build_list;
    use evalitem::EvalItem;
    use tokenizer::tokenize;
    
    #[test]
    fn flat_list() {
        let expected_list = vec!(EvalItem::List(vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::Value("hå".to_string()))));
        let expr = "(hej hoj hå)".to_string();
        let (index, item) = build_list(&tokenize(&expr), 0);
        match item {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn no_outer_parens_list() {
        let expected_list = vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::Value("hå".to_string()));
        let expr = "hej hoj hå)".to_string();
        let (index, item) = build_list(&tokenize(&expr), 0);
        match item {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn two_outer_parens_list() {
        let expected_list = vec!(EvalItem::List(vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::Value("hå".to_string()))), EvalItem::List(vec!(
            EvalItem::Value("hi".to_string()),
            EvalItem::Value("hi".to_string()))));
        let expr = "(hej hoj hå) (hi hi)".to_string();
        let (index, item) = build_list(&tokenize(&expr), 0);
        match item {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn nested_parens_list() {
        let expected_list = vec!(EvalItem::List(vec!(
            EvalItem::Value("hej".to_string()),
            EvalItem::Value("hoj".to_string()),
            EvalItem::List(vec!(
                EvalItem::List(vec!(
                    EvalItem::Value("yes".to_string()),
                    EvalItem::Value("yikes".to_string()))),
                EvalItem::Value("japp".to_string()),
                EvalItem::Value("jopp".to_string()))))));
        let expr = "(hej hoj ( (yes yikes) japp jopp))".to_string();
        let (index, item) = build_list(&tokenize(&expr), 0);
        match item {
            EvalItem::List(n) => {
                assert_eq!(expected_list, n);
            },
            _ => assert!(false),
        }
    }
}
