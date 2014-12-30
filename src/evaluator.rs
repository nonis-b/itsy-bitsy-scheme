use evalitem::EvalItem;
use environment::Environment;

pub fn evaluate(item: EvalItem, env: &mut Box<Environment>) -> EvalItem {
    let mut loops = 0i;
    loop {
        if loops > 100 { panic!("Max recursion reached!"); }

        match item {
            EvalItem::Value(ref value) => {
                match env.find_environment_with_var(value.as_slice()) {
                    Some(env_with_var) =>
                        return env_with_var.find_value(value.as_slice()).unwrap().clone(),
                    None => return item.clone(),
                }
            },
            EvalItem::List(ref list) => {
                if list.is_empty() { return EvalItem::Empty; }
                match list[0] {
                    EvalItem::Value(ref keyword) => {
                        match keyword.as_slice() {
                            "quote" => {
                                if list.len() == 1 { return EvalItem::Empty; }
                                let mut quoted = list.clone();
                                quoted.remove(0);
                                return EvalItem::List(quoted);
                            },
                            "define" => {
                                if list.len() != 3 {
                                    panic!("define takes 2 arguments.");
                                }
                                let item_to_evaluate = list[2].clone();
                                let item_to_add = evaluate(item_to_evaluate, env);
                                let name_to_add = match list[1].clone() {
                                    EvalItem::Value(val) => val,
                                    _ => panic!("Must give string as name!"),
                                };
                                env.add(name_to_add.as_slice(), item_to_add);
                                return EvalItem::Empty;
                            },
                            _ => panic!("Unknown keyword!"),
                        }
                    },
                    _ => panic!("Must start list evaluation with keyword!"),
                }                        
            },
            _ => panic!("Shouldn't get here!"),            
        }
    }
}

#[cfg(test)]
mod test {
    use super::evaluate;
    use evalitem::EvalItem;
    use environment::Environment;
    use std::collections::HashMap;
    
    #[test]
    fn evaluate_item() {
        let mut env = box Environment { vars: HashMap::new(), outer: None };
        let item = EvalItem::Value("ping".to_string());
        assert_eq!(EvalItem::Value("ping".to_string()),
                   evaluate(item, &mut env));
    }

    #[test]
    fn evaluate_quote() {
        let mut env = box Environment { vars: HashMap::new(), outer: None };
        let item = EvalItem::List(vec!(
            EvalItem::Value("quote".to_string()),
            EvalItem::Value("1".to_string()),
            EvalItem::Value("2".to_string())));
        assert_eq!(EvalItem::List(vec!(
            EvalItem::Value("1".to_string()),
            EvalItem::Value("2".to_string()))),
                   evaluate(item, &mut env));
    }

    #[test]
    fn evaluate_define_value() {
        let mut env = box Environment { vars: HashMap::new(), outer: None };
        let item = EvalItem::List(vec!(
            EvalItem::Value("define".to_string()),
            EvalItem::Value("one".to_string()),
            EvalItem::Value("einz".to_string())));
        assert_eq!(EvalItem::Empty, evaluate(item, &mut env));
        assert_eq!(EvalItem::Value("einz".to_string()), *env.find_value("one").unwrap());
    }

    #[test]
    fn evaluate_define_solve() {
        let mut env = box Environment { vars: HashMap::new(), outer: None };
        let item = EvalItem::List(vec!(
            EvalItem::Value("define".to_string()),
            EvalItem::Value("one".to_string()),
            EvalItem::Value("einz".to_string())));
        assert_eq!(EvalItem::Empty, evaluate(item, &mut env));
        assert_eq!(EvalItem::Value("einz".to_string()), *env.find_value("one").unwrap());

        let item = EvalItem::List(vec!(
            EvalItem::Value("define".to_string()),
            EvalItem::Value("yksi".to_string()),
            EvalItem::Value("one".to_string())));
        assert_eq!(EvalItem::Empty, evaluate(item, &mut env));
        assert_eq!(EvalItem::Value("einz".to_string()), *env.find_value("yksi").unwrap());
    }
}
