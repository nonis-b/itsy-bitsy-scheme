pub fn tokenize<'r>(program: &'r String) -> Vec<String> {

    let mut tokens = vec![];
    let mut buf = String::new();
    
    for c in program.as_slice().chars() {
        match c {
            '(' => {
                if buf.len() > 0 && !buf.as_slice().starts_with(";") {
                    tokens.push(buf.clone());
                    buf.clear();
                }
                tokens.push("(".to_string());
            },
            ')' => {
                if buf.len() > 0 && !buf.as_slice().starts_with(";") {
                    tokens.push(buf.clone());
                    buf.clear();
                }
                tokens.push(")".to_string());
            },
            ' '|'\t' => {
                if buf.len() > 0 && !buf.as_slice().starts_with(";"){
                    tokens.push(buf.clone());
                    buf.clear();
                }
            },
            '\n' => {
                if buf.len() > 0 && buf.as_slice().starts_with(";"){
                    buf.clear();
                }
            }
            _ => {
                buf.push(c);
            }
        }
    }
    tokens
}



#[cfg(test)]
mod test {
    use super::tokenize;

    #[test]
    fn tokenize_simple() {
        let input = "(hej )hå (\t  \nhoj )".to_string();
        let tokens = tokenize(&input);
        assert_eq!(tokens, vec!("(".to_string(),
                                "hej".to_string(),
                                ")".to_string(),
                                "hå".to_string(),
                                "(".to_string(),
                                "hoj".to_string(),
                                ")".to_string()));
    }

    #[test]
    fn tokenize_comment() {
        let input = ";;this is a comment\n(hej )hå (\t\
            ;more comment\n \nhoj )".to_string();
        let tokens = tokenize(&input);
        assert_eq!(tokens, vec!("(".to_string(),
                                "hej".to_string(),
                                ")".to_string(),
                                "hå".to_string(),
                                "(".to_string(),
                                "hoj".to_string(),
                                ")".to_string()));
    }
}
