use std::cmp::Ordering;

use super::file_struct::Node;

/// If the value is found then [`Ok`] is returned, containing the index of the matching element.
/// If the value is not found then [`Err`] is returned,
/// containing the index where a matching element could be inserted while maintaining sorted order
///
/// Sorting follow this rules:
/// - all directory before files
/// - alpha numeric (ASCII), with case insensitive (a = A)
/// - 2 before 10
/// - some other rules
///
/// Condition: content must be sorted with this rules before using this function
///
pub fn get_index_sorted(name: String, is_dir: bool, content: &[Node]) -> Result<usize, usize> {
    // notice we use negation when node is a dir
    // because 0 will have a smaller index than 1
    //
    // we lower all letter because 'A' < '_' < 'a' in ASCII, and
    // I prefer having '.' and '_' files on top

    // we use a third key in case of equality, because Linux is sensitive (a != A)
    content.binary_search_by_key(
        &(
            !is_dir,
            extract_for_search(&name, true),
            extract_for_search(&name, false),
        ),
        |n| {
            (
                !n.is_dir(),
                extract_for_search(&n.common().name, true),
                extract_for_search(&n.common().name, false),
            )
        },
    )
}

#[derive(Debug, Clone)]
enum CharsType {
    Str(String),
    I32(i32),
    Spe(String),
}

impl Ord for CharsType {
    /// basically: Spe < I32 < Str (in binarry so Spe have the smaller index)
    ///
    /// Note: on vscode and dolphin, specials caracteres are priotary but somehow
    /// they don't follow ASCII, for example, '_' is display before '*', but ASCII of
    /// '*' < '_'.
    /// If this a problem for someone, we will just need to change the Spe Spe comparaison
    /// with custom impl.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (CharsType::Str(str1), CharsType::Str(str2)) => str1.cmp(str2),
            (CharsType::I32(num1), CharsType::I32(num2)) => num1.cmp(num2),
            (CharsType::Str(_), CharsType::I32(_)) => std::cmp::Ordering::Greater,
            (CharsType::I32(_), CharsType::Str(_)) => std::cmp::Ordering::Less,
            (CharsType::Str(_), CharsType::Spe(_)) => Ordering::Greater,
            (CharsType::I32(_), CharsType::Spe(_)) => Ordering::Greater,
            (CharsType::Spe(_), CharsType::Str(_)) => Ordering::Less,
            (CharsType::Spe(_), CharsType::I32(_)) => Ordering::Less,
            (CharsType::Spe(spe1), CharsType::Spe(spe2)) => spe1.cmp(spe2),
        }
    }
}

/// not needed, but not safe to do that though
impl PartialOrd for CharsType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialEq for CharsType {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for CharsType {}

/// automate states
enum State {
    Start,
    String,
    Number,
    Special,
}

/// automate
fn extract_for_search(name: &str, lower: bool) -> Vec<CharsType> {
    let mut result = Vec::new();
    let mut current_str = String::new();
    let mut current_num = String::new();
    let mut current_spe = String::new();
    let mut state = State::Start;

    for c in name.chars() {
        match state {
            State::Start => {
                if c.is_ascii_digit() {
                    state = State::Number;
                    current_num.push(c);
                } else if c.is_alphabetic() {
                    state = State::String;
                    if lower {
                        current_str.push(c.to_lowercase().next().unwrap());
                    } else {
                        current_str.push(c);
                    }
                } else {
                    state = State::Special;
                    current_spe.push(c);
                }
            }
            State::String => {
                if c.is_ascii_digit() {
                    result.push(CharsType::Str(current_str.clone()));
                    current_str.clear();
                    state = State::Number;
                    current_num.push(c);
                } else if c.is_alphabetic() {
                    if lower {
                        current_str.push(c.to_lowercase().next().unwrap());
                    } else {
                        current_str.push(c);
                    }
                } else {
                    result.push(CharsType::Str(current_str.clone()));
                    current_str.clear();
                    state = State::Special;
                    current_spe.push(c);
                }
            }
            State::Number => {
                if c.is_ascii_digit() {
                    current_num.push(c);
                } else if c.is_alphabetic() {
                    result.push(CharsType::I32(current_num.parse::<i32>().unwrap()));
                    current_num.clear();
                    state = State::String;
                    if lower {
                        current_str.push(c.to_lowercase().next().unwrap());
                    } else {
                        current_str.push(c);
                    }
                } else {
                    result.push(CharsType::I32(current_num.parse::<i32>().unwrap()));
                    current_num.clear();
                    state = State::Special;
                    current_spe.push(c);
                }
            }
            State::Special => {
                if c.is_ascii_digit() {
                    result.push(CharsType::Spe(current_spe.clone()));
                    current_spe.clear();
                    state = State::Number;
                    current_num.push(c);
                } else if c.is_alphabetic() {
                    result.push(CharsType::Spe(current_spe.clone()));
                    current_spe.clear();
                    state = State::String;
                    if lower {
                        current_str.push(c.to_lowercase().next().unwrap());
                    } else {
                        current_str.push(c);
                    }
                } else {
                    current_spe.push(c);
                }
            }
        }
    }
    match state {
        State::Start => {
            panic!()
        }
        State::String => {
            result.push(CharsType::Str(current_str));
        }
        State::Number => {
            result.push(CharsType::I32(current_num.parse::<i32>().unwrap()));
        }
        State::Special => {
            result.push(CharsType::Spe(current_spe));
        }
    }

    result
}
