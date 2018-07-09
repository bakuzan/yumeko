use std::io;

pub fn take() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    String::from(input_text.trim())
}
