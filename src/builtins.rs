use evalitem::EvalItem;
use environment::Environment;

pub fn evaluate_builtin(name: &String, list: &Vec<EvalItem>, env: &mut Box<Environment>) -> EvalItem {
    match name.as_ref() {
        "+" => {
            let mut sum = 0u32;
            for i in 1..list.len() {
                if i > 0 {
                    match list[i] {
                        EvalItem::Value(ref value) => {
                            let parsed_int = (&value).parse::<u32>();
                            match parsed_int {
                                Ok(num) => sum = sum + num,
                                Err(E) => panic!("Element was not a number!"),
                            }
                        },
                        _ => panic!("Shouldn't happen!"),
                    }
                }
            }
            return EvalItem::Value(sum.to_string());
        },
        _ => {
            panic!("Unknown builtin name!");
        },
    }
}
