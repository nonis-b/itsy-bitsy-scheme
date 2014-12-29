use evalitem::EvalItem;
use environment::Environment;

pub evaluate(EvalItem item, Environment env) -> EvalItem {
    
}

#[cfg(test)]
mod test {
    use super::build_list;
    use evalitem::EvalItem;

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
}
