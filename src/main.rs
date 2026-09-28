fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(text: String) -> String {
    let first_word = text.split_whitespace().next().unwrap_or_default();

    let mut result = String::new();
    result.push_str(first_word);
    result.push('\n');

    let mut message = text;
    message.push_str(" rust");

    {
        let via_mut: &mut String = &mut message;
        result.push_str(via_mut);
        result.push('\n');
    }

    {
        let via_ref: &String = &message;
        result.push_str(via_ref);
        result.push('\n');
    }

    result
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
