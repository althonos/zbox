macro_rules! import_constant {
    ($($module:ident).+ , $name: ident, $type: ty) => {
        lazy_static! {
            pub static ref $name: $type = {

                use pyo3::ToPyObject;

                let gil = $crate::Python::acquire_gil();
                let py = gil.python();

                py.import(dot_stringify!($($module).*))
                    .expect(concat!("Can not import module: ", dot_stringify!($($module).*)))
                    .get(stringify!($name))
                    .expect(concat!("Can not load constant: ", dot_stringify!($($module).*), ".", stringify!($name)))
                    .to_object(py)
                    .extract(py)
                    .expect(concat!("Can not downcast ", stringify!($name), " as: ", stringify!($type)))
            };
        }

        impl $crate::std::fmt::Display for $name {
            fn fmt(&self, f: &mut $crate::std::fmt::Formatter) -> $crate::std::fmt::Result {
                write!(f, "{}", **self)
            }
        }

        impl ::std::cmp::PartialEq<$type> for $name {
            fn eq(&self, other: &$type) -> bool {
                **self == *other
            }
        }

        impl ::std::cmp::PartialEq<$name> for $type {
            fn eq(&self, other: &$name) -> bool {
                *self == **other
            }
        }

    }
}
