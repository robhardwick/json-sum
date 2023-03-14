use simd_json::{BorrowedValue, Value, ValueAccess};

/// Recursively search through JSON values, summing up numeric values of the supplied `search_key`
///
/// # Arguments
///
/// * `search_key` - The key that will have its values summed
/// * `value` - A simd_json `Value` struct
///
/// # Returns
///
/// The sum of numeric values for the `search_key` as a 64-bit floating point number
pub fn sum_json(search_key: &str, value: BorrowedValue) -> f64 {
    match value {
        BorrowedValue::Object(obj) => {
            // Recursively sum values for an object
            obj.into_iter()
                .map(|(key, value)| {
                    if search_key.eq(&key) && value.is_number() {
                        // If the key matches the search key and the value is a number then cast
                        // it to a 64-bit floating point number
                        value.cast_f64().unwrap_or(0.0)
                    } else {
                        // Otherwise, recurse into the value
                        sum_json(search_key, value)
                    }
                })
                .sum()
        }
        BorrowedValue::Array(array) => {
            // Recurse into array values
            array
                .into_iter()
                .map(|value| sum_json(search_key, value))
                .sum()
        }
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_json {
        ($name:ident, $json:literal, $expected:literal) => {
            #[test]
            fn $name() {
                let mut d = $json.to_vec();
                let value: BorrowedValue = simd_json::to_borrowed_value(&mut d).unwrap();

                assert_eq!(sum_json("key", value), $expected);
            }
        };
    }

    // Ensure empty JSON returns 0
    test_json!(empty, br#"{}"#, 0.0);

    // Ensure a single integer returns its value
    test_json!(single_integer, br#"{"some": "value", "key": 9}"#, 9.0);

    // Ensure multiple integers return their sum
    test_json!(
        multiple_integers,
        br#"{"values": [{"key": 9}, {"key": 7}]}"#,
        16.0
    );

    // Ensure multiple integers in nested search keys returns their sum
    test_json!(
        nested_integers,
        br#"{"value": {"key": 9, "another": {"key": 3}}}"#,
        12.0
    );

    // Ensure a single float returns its value
    test_json!(single_float, br#"{"some": "value", "key": 9.9}"#, 9.9);

    // Ensure multiple floats return their sum
    test_json!(
        multiple_floats,
        br#"{"values": [{"key": 8.5}, {"key": 7.5}]}"#,
        16.0
    );

    // Ensure multiple floats in nested search keys returns their sum
    test_json!(
        nested_floats,
        br#"{"value": {"key": 6.3, "another": {"key": 5.7}}}"#,
        12.0
    );

    // Ensure search keys with string values are ignored
    test_json!(
        ignore_strings,
        br#"{"values": [{"key": 24}, {"key": "test"}]}"#,
        24.0
    );

    // Ensure search keys with array values are recursed into
    test_json!(
        recurse_array,
        br#"{"key": [{"key": 12}, {"key": 6}]}"#,
        18.0
    );

    // Ensure search keys with object values are recursed into
    test_json!(recurse_object, br#"{"key": {"key": 6}}"#, 6.0);
}
