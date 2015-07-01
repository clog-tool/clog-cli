// regex cheat thanks to https://github.com/BurntSushi

// Until regex_macros compiles with nightly again, this directive should be commented out
// #[cfg(not(unstable))]
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);

#[cfg(feature = "debug")]
macro_rules! debugln {
    ($fmt:expr) => (println!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt:expr) => (print!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}

#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

macro_rules! werr {
    () => ()
}