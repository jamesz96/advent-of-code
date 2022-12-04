pub fn fixed_sorted_array_insert(items: &Vec<i32>, value: i32) -> Vec<i32> {
    let length = items.len();

    let mut idx = 0;
    while idx < length && value > items[idx] {
        idx += 1;
    }

    if idx == 0 { return items.clone(); }

    let mut result = vec![];

    if idx == length {
        let result_lower = &items[1..idx];

        result.extend_from_slice(result_lower);
        result.push(value);
    } else {
        let result_lower = &items[1..idx]; // Eject first element
        let result_upper = &items[idx..length];

        result.extend_from_slice(result_lower);
        result.push(value);
        result.extend_from_slice(result_upper);
    }
    return result;
}


