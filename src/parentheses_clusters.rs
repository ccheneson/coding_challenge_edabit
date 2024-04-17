fn split(input: &str) -> Vec<&str> {
    let mut result: Vec<&str> = vec![];
    let mut index_from: Option<usize> = None;
    let mut counter: i16 = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                if index_from.is_none() {
                    index_from = Some(i);
                }
                counter += 1;
            }
            _ => {
                counter -= 1;
                if let Some(index) = index_from {
                    if counter == 0 {
                        result.push(&input[index..=i]);
                        index_from = None;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::parentheses_clusters::split;

    #[test]
    fn it_works() {
        let result = split("()()()");
        assert_eq!(result, vec!("()", "()", "()"));

        let result = split("((()))");
        assert_eq!(result, vec!("((()))"));

        let result = split("((()))(())()()(()())");
        assert_eq!(result, vec!("((()))", "(())", "()", "()", "(()())"));

        let result = split("((())())(()(()()))");
        assert_eq!(result, vec!("((())())", "(()(()()))"));
    }
}
