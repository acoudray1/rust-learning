#[cfg(test)]
mod test {
    extern crate odds_and_ends_lib;
    use odds_and_ends_lib::greetings;

    #[test]
    #[should_panic]
    #[ignore]
    fn english_hello_correct() {
        assert_eq!("hello!", greetings::english::hello());
    }

    #[test]
    fn french_hello_correct() {
        assert_eq!("bonjour!", greetings::french::hello());
    }

    #[test]
    fn english_bye_correct() {
        assert_eq!("bye!", greetings::english::bye());
    }

    #[test]
    fn french_bye_correct() {
        assert_eq!("au revoir!", greetings::french::bye());
    }
}

