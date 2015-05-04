// regex cheat thanks to https://github.com/BurntSushi
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);
