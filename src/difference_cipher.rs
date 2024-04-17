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

#[cfg(test)]
mod tests {
    use crate::difference_cipher::encrypt;

    #[test]
    fn it_works() {
        let result = encrypt("Hello");
        assert_eq!(result, vec!(72, 29, 7, 0, 3));

        let result = encrypt("Sunshine");
        assert_eq!(result, vec!(83, 34, -7, 5, -11, 1, 5, -9));
    }
}
