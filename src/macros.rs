// regex cheat thanks to https://github.com/BurntSushi

// Until regex_macros compiles with nightly again, this directive should be commented out
// #[cfg(not(unstable))]
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);
