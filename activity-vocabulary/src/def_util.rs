#[macro_export]
macro_rules! def_subtypes {
    ($base:ident, $sub:ident, [ ], { $($name:ident),* }) => {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        pub enum $sub {
            $( $name($name) ),*
        }

        impl From<$sub> for $base {
            fn from(value: $sub) -> Self {
                match value {
                    $( $sub::$name(x) => x.into() ),*
                }
            }
        }
    };
    ($base:ident, $sub:ident, [ $direct_super:ident ], { $($name:ident),* }) => {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        pub enum $sub {
            $( $name($name) ),*
        }

        impl From<$base> for $direct_super {
            fn from(value: $base) -> Self {
                value._super
            }
        }

        impl From<$sub> for $base {
            fn from(value: $sub) -> Self {
                match value {
                    $( $sub::$name(x) => x.into() ),*
                }
            }
        }

        impl From<$sub> for $direct_super {
            fn from(value: $sub) -> Self {

                Into::<$base>::into(value)._super
            }
        }

        impl std::ops::Deref for $base {
            type Target = $direct_super;

            fn deref(&self) -> &Self::Target {
                &self._super
            }
        }

        impl std::ops::DerefMut for $base {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self._super
            }
        }
    };
    ($base:ident, $sub:ident, [ $direct_super:ident, $($super:ident),+ ], { $($name:ident),* }) => {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        pub enum $sub {
            $( $name($name) ),*
        }

        impl From<$base> for $direct_super {
            fn from(value: $base) -> Self {
                value._super
            }
        }

        $(
        impl From<$base> for $super {
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

        impl From<$sub> for $direct_super {
            fn from(value: $sub) -> Self {

                Into::<$base>::into(value)._super
            }
        }

        $(
        impl From<$sub> for $super {
            fn from(value: $sub) -> Self {
                Into::<$base>::into(value).into()
            }
        }
        )*

        impl std::ops::Deref for $base {
            type Target = $direct_super;

            fn deref(&self) -> &Self::Target {
                &self._super
            }
        }

        impl std::ops::DerefMut for $base {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self._super
            }
        }
    };
}
