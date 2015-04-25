// regex cheat thanks to https://github.com/BurntSushi
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);

// A macro creating an entry types, and their aliases
macro_rules! commit_type_enum {
    (#[derive($($d:ident),+)] pub enum $e:ident { $($v:ident ( $($a:ident),* ) ),+ }) => {
        #[derive($($d,)+)]
        pub enum $e {
            Unknown,
            $($v,)+
        }

        impl $e {
            #[allow(dead_code)]
            pub fn aliases(&self) -> Option<Vec<&'static str>> {
                match *self {
                    $e::Unknown => None,
                    $($e::$v    => Some(vec![
                        $( stringify!($a) ),*
                    ]),)+
                }
            }
            #[allow(dead_code)]
            pub fn all_aliases() -> Vec<&'static str> {
                vec![
                   $( $( stringify!($a),)* )+
                ]
            }
        }
        impl ::std::str::FromStr for $e {
            type Err = $e;

            #[allow(dead_code)]
            fn from_str(s: &str) -> Result<Self,Self::Err> {
                match s {
                    $(stringify!($v)  $( | stringify!($a) )* => Ok($e::$v),)+
                    _                                        => Err($e::Unknown)
                }
            }
        }
    };
}
