/// Naive parser for JSON string containing pairs of points. The only supported format is:
/// ```json
/// {"pairs":[
///    {"x0":104.2012976847175310, "y0":11.0467543354408946, "x1":-75.7142017353818488, "y1":74.9748080350816508},
///    {"x0":147.7268925813784790, "y0":-29.2567384522948117, "x1":163.0052293198308462, "y1":-3.9442208853715357}
/// ]}
/// ```
pub fn parse_pairs(input: &[u8]) -> Result<Vec<[f64; 4]>, Box<dyn std::error::Error>> {
    naive_validate_json(input)?;

    let mut index = 0;
    let mut pairs = Vec::new();

    while index < input.len() {
        if index < 10 {
            index += 1;
            // we know first 10 bytes are '["pairs": [' so we can skip them
            continue;
        }

        if input[index] == b'{' {
            let mut points = [0.0, 0.0, 0.0, 0.0];
            let mut pair_index = 0;
            while input[index] != b'}' {
                if input[index] == b':' {
                    let mut num_length = 0;
                    while input[index + 1 + num_length] != b','
                        && input[index + 1 + num_length] != b'}'
                    {
                        num_length += 1;
                    }
                    let number = &input[index + 1..index + 1 + num_length];
                    let number_str = std::str::from_utf8(number)?;
                    let n = number_str.parse::<f64>()?;
                    points[pair_index] = n;

                    pair_index += 1;
                    index += num_length;

                    continue;
                }
                index += 1;
            }
            pairs.push(points);
        }

        index += 1;
    }

    Ok(pairs)
}

/// Naively checks whether JSON string is valid by checking for matching brackets.
/// Assumptions:
/// - only ASCII characters
/// - no [] and {} characters in keys and values
fn naive_validate_json(input: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut stack = Vec::new();
    for byte in input {
        if !byte.is_ascii() {
            return Err("Invalid pairs JSON: Non-ASCII character found".into());
        }
        match byte {
            b'[' | b'{' => stack.push(byte),
            b']' if stack.pop() != Some(&b'[') => {
                return Err("Invalid pairs JSON: Mismatched brackets: \"]\"".into());
            }
            b'}' if stack.pop() != Some(&b'{') => {
                return Err("Invalid pairs JSON: Mismatched brackets: \"}\"".into());
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pairs() {
        let input = br#"{"pairs":[{"x0":104.2012976847175310, "y0":11.0467543354408946, "x1":-75.7142017353818488, "y1":74.9748080350816508},{"x0":147.7268925813784790, "y0":-29.2567384522948117, "x1":163.0052293198308462, "y1":-3.9442208853715357}]}"#;
        let pairs = parse_pairs(input).unwrap();
        assert_eq!(pairs.len(), 2);
        assert_eq!(
            pairs[0],
            [
                104.20129768471753,
                11.046754335440895,
                -75.71420173538185,
                74.97480803508165
            ]
        );
        assert_eq!(
            pairs[1],
            [
                147.72689258137848,
                -29.256738452294812,
                163.00522931983085,
                -3.9442208853715357
            ]
        );
    }
}
