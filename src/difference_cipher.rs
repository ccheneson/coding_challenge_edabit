//https://edabit.com/challenge/KZrmicjc8zCZyGNee

use std::string::FromUtf8Error;

fn encrypt(input: &str) -> Vec<i8> {
    let mut input_as_u8 = input
        .chars()
        .into_iter()
        .map(|e| e as i8)
        .collect::<Vec<i8>>();

    let mut previous_elem_for_next_iteration: i8 = 0;

    for i in 0..input_as_u8.len() {
        if i == 0 {
            previous_elem_for_next_iteration = input_as_u8[i];
            continue;
        } else {
            let current = input_as_u8[i];
            let diff = current - previous_elem_for_next_iteration;
            previous_elem_for_next_iteration = input_as_u8[i];
            input_as_u8[i] = diff;
        }
    }

    input_as_u8
}

fn decrypt(input: &[i8]) -> Result<String, FromUtf8Error> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());

    let mut previous_elem_for_next_iteration: i8 = 0;

    for i in 0..input.len() {
        if i == 0 {
            previous_elem_for_next_iteration = input[i];
            result.push(input[i] as u8);
            continue;
        } else {
            let current = input[i];
            let diff = current + previous_elem_for_next_iteration;
            previous_elem_for_next_iteration = diff;

            result.push(diff as u8);
        }
    }
    String::from_utf8(result)
}

#[cfg(test)]
mod tests {
    use crate::difference_cipher::decrypt;
    use crate::difference_cipher::encrypt;

    #[test]
    fn it_works() {
        let result = encrypt("Hello");
        assert_eq!(result, vec!(72, 29, 7, 0, 3));

        let result = encrypt("Sunshine");
        assert_eq!(result, vec!(83, 34, -7, 5, -11, 1, 5, -9));

        let result = decrypt(&[72, 33, -73, 84, -12, -3, 13, -13, -68]);
        assert_eq!(result, Ok("Hi there!".to_owned()));
    }
}
