// https://edabit.com/challenge/JmuM2cP5MvruRjr6c

fn at_bash(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    input.chars().into_iter().for_each(|chr| match chr {
        'a'..='z' => {
            let diff = (chr as u8) - ('a' as u8);
            let encoded = ('z' as u8) - diff;
            result.push(encoded as char);
        }
        'A'..='Z' => {
            let diff = (chr as u8) - ('A' as u8);
            let encoded = ('Z' as u8) - diff;
            result.push(encoded as char);
        }
        _ => result.push(chr as char),
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::at_bash::at_bash;

    #[test]
    fn it_works() {
        let result = at_bash("apple");
        assert_eq!(result, "zkkov");

        let result = at_bash("Hello world!");
        assert_eq!(result, "Svool dliow!");

        let result = at_bash("Christmas is the 25th of December");
        assert_eq!(result, "Xsirhgnzh rh gsv 25gs lu Wvxvnyvi");
    }
}
