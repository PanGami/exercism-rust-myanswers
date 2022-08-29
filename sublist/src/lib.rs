#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use self::Comparison::*;
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (first_len, second_len) if first_len < second_len => 
            if _second_list.windows(first_len).any(|sl| sl == _first_list) { Sublist } else { Unequal },
        (first_len, second_len) if first_len > second_len => 
            if _first_list.windows(second_len).any(|sl| sl == _second_list) { Superlist } else { Unequal },
        (_, _) => if _first_list == _second_list { Equal } else { Unequal }
    }
}
