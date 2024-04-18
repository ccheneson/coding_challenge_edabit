//https://edabit.com/challenge/t9S9nNr79pLqjXgqb

fn can_complete(input: &str, word: &str) -> bool {
    if word.len() < input.len() {
        false
    } else {
        let mut it_word = word.chars();
        'outer: for ci in input.chars() {
            while let Some(cw) = it_word.next() {
                if cw == ci {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::complete_word::can_complete;

    #[test]
    fn it_works() {
        let result = can_complete("butl", "beautiful");
        assert_eq!(result, true);

        let result = can_complete("butlz", "beautiful");
        assert_eq!(result, false);

        let result = can_complete("tulb", "beautiful");
        assert_eq!(result, false);

        let result = can_complete("bbutl", "beautiful");
        assert_eq!(result, false);
    }
}
