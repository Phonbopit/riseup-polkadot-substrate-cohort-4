fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result: String = String::from(a);
    result.push_str(b);
    result
}

fn main() {
    let string1: &str = "Hello, ";
    let string2: &str = "world!";
    let result: String = concatenate_strings(string1, string2);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_strings() {
        assert_eq!(concatenate_strings("Hello, ", "world!"), "Hello, world!");
    }
}
