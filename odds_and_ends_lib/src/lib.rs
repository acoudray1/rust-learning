// rust uses directories and files equivalent for the declaration of its submodules
// greetings is going to be a folder while english a file
// french could be a file but it works also if we write it inside greetings
pub mod greetings {
    pub mod english;
    pub mod french {
        pub fn hello() -> String { "bonjour!".to_string() }
        pub fn bye() -> String { "au revoir!".to_string() }
    }
}