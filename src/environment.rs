use evalitem::EvalItem;
use std::collections::HashMap;

pub struct Environment {
    vars: HashMap<String, EvalItem>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    fn add(&mut self, name: &str, value: EvalItem) {
        self.vars.insert(name.to_string(), value);
    }

    fn find_value(&self, name: &str) -> Option<&EvalItem> {
        if !self.vars.contains_key(&name.to_string()) { return None; }
        Some(&self.vars[name.to_string()])
    }
}


#[cfg(test)]
mod test {
    use super::Environment;
    use evalitem::EvalItem;
    use std::collections::HashMap;
    
    #[test]
    fn add_item() {
        let mut env = Environment { vars: HashMap::new(), outer: None };
        env.add("theVar", EvalItem::Empty);
        env.add("theOtherVar", EvalItem::List(vec!(EvalItem::Value("hej".to_string()), EvalItem::Value("hå".to_string()))));
        assert_eq!(EvalItem::Empty, *(env.find_value("theVar")).unwrap());
        assert_eq!(EvalItem::List(vec!(EvalItem::Value("hej".to_string()), EvalItem::Value("hå".to_string()))), *(env.find_value("theOtherVar")).unwrap());
        assert!(env.find_value("somethingElse").is_none());
    }
}
