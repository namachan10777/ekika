#[macro_export]
macro_rules! def_subtypes {
    ($base:ident, $sub:ident, [ $($super_t:ident),* ], { $($name:ident),* }) => {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        pub enum $sub {
            $( $name($name) ),*
        }

        $(
        impl From<$base> for $super_t {
            fn from(value: $base) -> Self {
                value._super.into()
            }
        }
        )*

        impl From<$sub> for $base {
            fn from(value: $sub) -> Self {
                match value {
                    $( $sub::$name(x) => x.into() ),*
                }
            }
        }

        $(
        impl From<$sub> for $super_t {
            fn from(value: $sub) -> Self {
                Into::<$base>::into(value).into()
            }
        }
        )*
    };
}
