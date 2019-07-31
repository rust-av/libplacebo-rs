extern crate paste;

#[macro_export]
macro_rules! simple_enum {
    ($enum:ident, ($($field_name:ident),*$(,)*)) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $enum {
            $($field_name,)*
        }
    }
}

#[macro_export]
macro_rules! create_enum {
    ($enum:ident, $type:ident,
    ($($field_name:ident),*$(,)*)) => {

        simple_enum!($enum, ($($field_name),*));

        impl $enum {
            paste::item! {
                pub(crate) fn [<to_ $type>](&self) -> $type {
                    match self {
                        $(
                            $enum::$field_name => $type::[<PL_ $field_name>],
                        )*
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! set_struct {
    ($struct:ident, $param:ident, $type:tt) => {
        #[derive(Clone)]
        pub struct $struct {
            $param: $type,
        }
    };
}

#[macro_export]
macro_rules! default_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*)
     ) => {

        impl Default for $struct {
            fn default() -> Self {
                let $param = $type {
                    $($field_name: $field_default_expr,)*
                };
                $struct{ $param }
            }
        }
    };
    ($struct:ident, $param:ident, $field_default_expr:expr) => {

        impl Default for $struct {
            fn default() -> Self {
                let $param = $field_default_expr;
                $struct{ $param }
            }
        }
    };
}

#[macro_export]
macro_rules! set_params {
    ($struct:ident, $param:ident,
    ($($field_name:ident),*$(,)*),
    ($($field_type:ty),*$(,)*),
    ($($field_expr:expr),*$(,)*))
    => {
            impl $struct {
                paste::item! {
                    $(
                        pub fn [<set_ $field_name>](&mut self, $field_name: $field_type) {
                            self.$param.$field_name = paste::expr! { ($field_expr) }
                        }
                    )*
                }
            }
       }
}

#[macro_export]
macro_rules! implement_struct {
    (
        $struct:ident, $param:ident, $type:tt,
        ($($field_name:ident),*$(,)*),
        ($($field_type:ty),*$(,)*),
        ($($field_expr:expr),*$(,)*)
     ) => {
        impl $struct {
            pub fn new($($field_name: $field_type,)*) -> Self {
                let $param = $type {
                    $($field_name: $field_expr,)*
                };

                $struct{ $param }
            }

        }

        set_params!($struct, $param,
                    ($($field_name,)*),
                    ($($field_type,)*),
                    ($($field_expr,)*)
        );
    };
}

#[macro_export]
macro_rules! create_struct {
    ($struct:tt, $param:ident, $type:tt,
        ($($field_name:ident),*$(,)*),
        ($($field_type:ty),*$(,)*),
        ($($field_expr:expr),*$(,)*)
    ) => {

        set_struct!($struct, $param, $type);

        implement_struct!($struct, $param, $type,
                         ($($field_name,)*),
                         ($($field_type,)*),
                         ($($field_expr,)*)
        );
    };
}

#[macro_export]
macro_rules! create_complete_struct {
    ($struct:tt, $param:ident, $type:tt,
        ($($field_name:ident),*$(,)*),
        ($($field_type:ty),*$(,)*),
        ($($field_default_expr:expr),*$(,)*),
        ($($field_expr:expr),*$(,)*)
    ) => {

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*)
        );

        create_struct!($struct, $param, $type,
                     ($($field_name,)*),
                     ($($field_type,)*),
                     ($($field_expr,)*)
        );


    };
    ($struct:tt, $param:ident, $type:tt,
        ($($field_name:ident),*$(,)*),
        ($($field_type:ty),*$(,)*),
        ($($field_expr:expr),*$(,)*)
    ) => {

        create_struct!($struct, $param, $type,
                     ($($field_name,)*),
                     ($($field_type,)*),
                     ($($field_expr,)*)
        );
    };
}

#[macro_export]
macro_rules! get_ptr {
    ($struct:tt, $param:tt, $type:ty) => {
        impl $struct {
            pub(crate) fn get_ptr(&self) -> *const $type {
                &self.$param as *const $type
            }
        }
    };
}

#[macro_export]
macro_rules! internal_object {
    ($struct:tt, $param:tt, $type:ty) => {
        impl $struct {
            pub(crate) fn internal_object(&self) -> $type {
                self.$param
            }
        }
    };
}
