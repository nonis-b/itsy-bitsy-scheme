use evalitem::EvalItem;
use environment::Environment;

pub fn evaluate(item_init: EvalItem, env: &mut Box<Environment>) -> EvalItem {
    let mut loops = 0i;
    let mut item = item_init;
    loop {
        if loops > 100 { panic!("Max recursion reached!"); }

        match item {
            EvalItem::Value(ref value) => {
                match env.find_environment_with_var(value.as_slice()) {
                    Some(env_with_var) =>
                        return env_with_var.find_value(value.as_slice())
                        .unwrap().clone(),
                    None => return item.clone(),
                }
            },
            EvalItem::List(n) => {
                panic!("TODO!");
            },
            _ => {
                panic!("Shouldn't get here!");
            }
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
}
