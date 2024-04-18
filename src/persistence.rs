//https://edabit.com/challenge/fKZ6m9rDHDvgp6aeu

use std::str::Chars;

fn additive_persistence(input: u32) -> usize {
    let adder = |input: &mut dyn Iterator<Item = u32>| input.sum();
    operation_persistence(input, &adder)
}

fn multiplicative_persistence(input: u32) -> usize {
    let multiplier = |input: &mut dyn Iterator<Item = u32>| input.product();
    operation_persistence(input, &multiplier)
}

fn operation_persistence(
    input: u32,
    fn_op: &dyn Fn(&mut dyn Iterator<Item = u32>) -> u32,
) -> usize {
    fn do_persistence(
        chrs: Chars<'_>,
        it: usize,
        fn_op: &dyn Fn(&mut dyn Iterator<Item = u32>) -> u32,
    ) -> usize {
        let prod: u32 = fn_op(&mut chrs.flat_map(|c| c.to_digit(10)));
        if prod < 10 {
            if it == 1 {
                0
            } else {
                it
            }
        } else {
            do_persistence(prod.to_string().chars(), it + 1, fn_op)
        }
    }

    do_persistence(input.to_string().chars(), 1, fn_op)
}

#[cfg(test)]
mod tests {
    use crate::persistence::additive_persistence;
    use crate::persistence::multiplicative_persistence;

    #[test]
    fn it_works() {
        let result = additive_persistence(1679583);
        assert_eq!(result, 3);

        let result = additive_persistence(123456);
        assert_eq!(result, 2);

        let result = additive_persistence(6);
        assert_eq!(result, 0);

        let result = multiplicative_persistence(77);
        assert_eq!(result, 4);

        let result = multiplicative_persistence(123456);
        assert_eq!(result, 2);

        let result = multiplicative_persistence(4);
        assert_eq!(result, 0);
    }
}
