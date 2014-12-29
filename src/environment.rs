use evalitem::EvalItem;
use std::collections::HashMap;

pub struct Environment {
    pub vars: HashMap<String, EvalItem>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn add(&mut self, name: &str, value: EvalItem) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn find_value(&self, name: &str) -> Option<&EvalItem> {
        if !self.vars.contains_key(&name.to_string()) { return None; }
        Some(&self.vars[name.to_string()])
    }

    pub fn find_environment_with_var(&self, name: &str) -> Option<&Environment> {
        if self.find_value(name).is_some() { return Some(self); }
        match self.outer {
            Some(ref n) => n.find_environment_with_var(name),
            None => None,
        }
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
        env.add("theOtherVar",
                EvalItem::List(vec!(EvalItem::Value(
                    "hej".to_string()), EvalItem::Value("hå".to_string()))));
        assert_eq!(EvalItem::Empty, *(env.find_value("theVar")).unwrap());
        assert_eq!(EvalItem::List(vec!(EvalItem::Value("hej".to_string()),
                                       EvalItem::Value("hå".to_string()))),
                   *(env.find_value("theOtherVar")).unwrap());
        assert!(env.find_value("somethingElse").is_none());
    }

    #[test]
    fn nested_environments() {
        let mut env3 = box Environment { vars: HashMap::new(), outer: None };
        env3.add("theVar", EvalItem::Empty);
        let mut env2 = box Environment { vars: HashMap::new(), outer: Some(env3) };
        let mut env = Environment { vars: HashMap::new(), outer: Some(env2) };
        env.add("theOtherVar",                
                EvalItem::List(vec!(EvalItem::Value(
                    "hej".to_string()), EvalItem::Value("hå".to_string()))));

        let env_find1 = env.find_environment_with_var("theVar").unwrap();
        assert_eq!(EvalItem::Empty, *(env_find1.find_value("theVar")).unwrap());
        let env_find2 = env.find_environment_with_var("theOtherVar").unwrap();
        assert_eq!(EvalItem::List(vec!(EvalItem::Value("hej".to_string()),
                                       EvalItem::Value("hå".to_string()))),
                   *(env_find2.find_value("theOtherVar")).unwrap());
        assert!(env.find_environment_with_var("whatever").is_none());
        assert!(env.find_value("somethingElse").is_none());
    }
}
