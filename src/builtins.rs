use evalitem::EvalItem;
use environment::Environment;

pub fn evaluate_builtin(name: &String, list: &Vec<EvalItem>, env: &mut Box<Environment>) -> EvalItem {
    match name.as_slice() {
        "+" => {
            let mut sum = 0i;
            for i in range(1, list.len()) {
                if i > 0 {
                    match list[i] {
                        EvalItem::Value(ref value) => {
                            let parsed_int = from_str(value.as_slice());
                            match parsed_int {
                                Some(num) => sum = sum + num,
                                None => panic!("Element was not a number!"),
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
