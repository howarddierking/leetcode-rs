// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// Given a string containing digits from 2-9 inclusive, return all possible letter combinations
// that the number could represent. Return the answer in any order.
//
// A mapping of digits to letters (just like on the telephone buttons) is given below. Note
// that 1 does not map to any letters.

use std::collections::HashMap;
use std::iter::repeat;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let keypad = HashMap::from([
        ('2', vec!['a','b','c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g','h','i']),
        ('5', vec!['j','k','l']),
        ('6', vec!['m','n','o']),
        ('7', vec!['p','q','r','s']),
        ('8', vec!['t','u','v']),
        ('9', vec!['w','x','y','z'])]);

    // error case? I guess we'll just panic
    // quick escape
    let mut combinations: Vec<String> = vec![];
    let input_length = digits.len();
    if input_length == 0 {
        return vec![];
    }
    let output_length: i32 = digits.chars().map(|d|{
        keypad.get(&d).unwrap().len() as i32
    }).product();

    let mut combination_ctr = output_length;
    let mut combination_iterators = Vec::new();

    for c in digits.chars(){
        let digit_chars = keypad.get(&c).unwrap();
        let r = combination_ctr / digit_chars.len() as i32;
        combination_ctr = combination_ctr / digit_chars.len() as i32;

        // generate vectors by repeating the correct pattern
        // TODO: I feel like this can be improved by not causing the iterators to run until
        //  generating the final output, but I've got a lifetime issue
        let i: Vec<char> = digit_chars.iter()
            .flat_map(|d|{
                repeat(*d).take(r as usize)
            })
            .cycle()
            .take(output_length as usize)
            .collect();

        combination_iterators.push(i);
    }

    for i in 0..output_length {
        let mut combination_chars = Vec::new();
        for c_iter in combination_iterators.iter() {
            combination_chars.push(c_iter[i as usize])
        }
        combinations.push(combination_chars.into_iter().collect())
    }

    combinations
}

#[cfg(test)]
mod tests {
    use crate::phone_letter_combinations::letter_combinations;

    #[test]
    fn example1() {
        let expected: Vec<String> = ["ad","ae","af","bd","be","bf","cd","ce","cf"].map(String::from).to_vec();
        let actual = letter_combinations("23".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn example2() {
        let expected: Vec<String> = vec![];
        let actual = letter_combinations("".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn example3() {
        let expected: Vec<String> = ["a","b","c"].map(String::from).to_vec();
        let actual = letter_combinations("2".to_string());
        assert_eq!(expected, actual);
    }
}