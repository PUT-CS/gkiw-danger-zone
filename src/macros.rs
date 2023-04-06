#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
#[macro_export]
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}

#[macro_export]
/// Generate getter methods for references to fields of a struct
macro_rules! gen_ref_getters {
    {$t:ty, $($field:ident -> $type:ty,)+} => {
        impl $t {
            $(
                pub fn $field(&self) -> $type {
                    &self.$field
                }
            )+
        }
    };
}

#[macro_export]
/// Generate getter methods to fields of a struct
macro_rules! gen_getters {
    {$t:ty, $($field:ident -> $type:ty,)+} => {
        impl $t {
            $(
                pub fn $field(&self) -> $type {
                    self.$field
                }
            )+
        }
    };
}
/// Generate getter methods to fields of a struct
macro_rules! gen_mut_getters {
    {$t:ty, $($field:ident -> $type:ty,)+} => {
        impl $t {
            $(
                pub fn $field(&self) -> $type {
                    &mut self.$field
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! key_pressed {
    ($window:expr, $key:expr, $action:expr) => {
        if $window.get_key($key) == Action::Press {
            $action
        }
    };
}
