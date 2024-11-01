

#[cfg(test)]
mod tests {
    use crate::Lexer;

    #[test]
    fn test_lexer_ok() {
        let mut lexer = Lexer::new(String::from("+-/"));
        // println!("{:?}", lexer);
        // lexer.next_char();
        // println!("{:?}", lexer);
        // lexer.next_char();
        // println!("{:?}", lexer);
        // lexer.next_char();
        // println!("{:?}", lexer);

        for (index, token) in lexer.enumerate() {
            println!("n({}): {:?}", index, token);
        }
    }
}