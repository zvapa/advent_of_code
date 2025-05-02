use std::collections::HashMap;

// part 1
const ALLOWED_VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const EXCLUDED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn contains_at_least_three_vowels(s: &str) -> bool {
    let mut count = 0;
    s.chars().any(|c| {
        if ALLOWED_VOWELS.contains(&c) {
            count += 1;
            if count == 3 { return true } else { false }
        } else {
            false
        }
    })
}

pub fn contains_at_least_one_letter_that_appears_twice_in_a_row(s: &str) -> bool {
    let mut last_char: char = std::default::Default::default();
    s.chars().any(|c| {
        if c == last_char {
            return true;
        } else {
            last_char = c;
            false
        }
    })
}

pub fn does_not_contain_excluded_strings(s: &str) -> bool {
    !s.chars().collect::<Vec<char>>().windows(2).any(|group| {
        let pair: String = group.iter().collect();
        EXCLUDED_STRINGS.contains(&pair.as_str())
    })
}

// part 2

// WRONG - forgot about overlapping !
// pub fn contains_any_two_letters_pair_twice_without_overlapping(s: &str) -> bool {
//     let mut two_letters_appearances: HashMap<&[char], u8> = HashMap::new();
//     s.chars()
//     .collect::<Vec<char>>()
//     .windows(2)
//     .any(|two_letters| {
//         if let Some(count) = two_letters_appearances.get_mut(two_letters) {
//             *count += 1;
//             if *count == 2 { true } else { false }
//         } else {
//             two_letters_appearances.insert(two_letters, 1);
//             false
//         }
//     })
// }

pub fn contains_any_two_letters_pair_twice_without_overlapping(s: &str) -> bool {
    let mut any_two_letters_appearence: HashMap<(char, char), usize> = HashMap::new();
    s.chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(2)
        .any(|two_letters| {
            if let [(idx1, char1), (_, char2)] = two_letters {
                if let Some(first_appearing) = any_two_letters_appearence.get(&(*char1, *char2)) {
                    if idx1 - 1 > *first_appearing {
                        return true;
                    } else {
                        any_two_letters_appearence.insert((*char1, *char2), *idx1);
                        return false;
                    }
                } else {
                    any_two_letters_appearence.insert((*char1, *char2), *idx1);
                    return false;
                }
            } else {
                false
            }
        })
}

pub fn contains_repeating_letter_with_one_between(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|three_letters_sandwich| {
            if let [a, _, c] = three_letters_sandwich {
                if a == c { true } else { false }
            } else {
                false
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // part 1
    #[test]
    fn test_contains_at_least_three_vowels() {
        let valid_input = "aaa";
        let invalid_input = "ii";
        assert!(contains_at_least_three_vowels(valid_input));
        assert!(!contains_at_least_three_vowels(invalid_input));
    }

    #[test]
    fn test_contains_at_least_one_letter_that_appears_twice_in_a_row() {
        let valid_input = "ugknbfddgicrmopn";
        let invalid_input = "jchzalrnumimnmhp";
        assert!(contains_at_least_one_letter_that_appears_twice_in_a_row(
            valid_input
        ));
        assert!(!contains_at_least_one_letter_that_appears_twice_in_a_row(
            invalid_input
        ));
    }

    #[test]
    fn test_does_not_contain_excluded_strings() {
        let valid_input = "ugknbfddgicrmopn";
        let invalid_input = "haegwjzuvuyypxyu";
        assert!(does_not_contain_excluded_strings(valid_input));
        assert!(!does_not_contain_excluded_strings(invalid_input));
    }

    // part 2
    #[test]
    fn test_contains_any_two_letters_pair_twice_without_overlapping() {
        let valid_input = "qjhvhtzxzqqjkmpb";
        let invalid_input = "ueihvxviirnooomi";
        assert!(contains_any_two_letters_pair_twice_without_overlapping(
            valid_input
        ));
        assert!(!contains_any_two_letters_pair_twice_without_overlapping(
            invalid_input
        ));
    }

    #[test]
    fn test_contains_repeating_letter_with_one_between() {
        let valid_input = "ieodomkazucvgmuy";
        let invalid_input = "uurcxstgmygtbstg";
        assert!(contains_repeating_letter_with_one_between(valid_input));
        assert!(!contains_repeating_letter_with_one_between(invalid_input));
    }
}
