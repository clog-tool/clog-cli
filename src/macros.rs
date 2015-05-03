// regex cheat thanks to https://github.com/BurntSushi
#[cfg(not(unstable))]
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);
