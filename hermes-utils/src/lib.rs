pub use hermes_utils_derive::Example;

#[derive(Default, Clone)]
pub struct ExampleConfig {
    pub field_name: Option<String>,
    pub minimal: bool,
    pub index: Option<usize>,
}

/// A trait used to generate example values of the implementing struct.
pub trait Example: Sized {
    /// Generate a minimal example (Options are set to None and Vecs are empty)
    fn minimal_example() -> Self {
        Self::example(ExampleConfig {
            minimal: true,
            ..Default::default()
        })
    }

    /// Generate a full example (Options are set to Some and Vecs contain values)
    fn full_example() -> Self {
        Self::example(Default::default())
    }

    /// Generate an example using the given config
    fn example(config: ExampleConfig) -> Self;
}

impl Example for String {
    fn example(config: ExampleConfig) -> Self {
        match (config.field_name, config.index) {
            (Some(field_name), Some(index)) => format!("<{} {}>", field_name.replace("_", " "), index),
            (Some(field_name), None) => format!("<{}>", field_name.replace("_", " ")),
            (None, Some(index)) => format!("string {}", index),
            (None, None) => "a string".into(),
        }
    }
}

impl<T: Example> Example for Option<T> {
    fn example(config: ExampleConfig) -> Self {
        if config.minimal {
            None
        } else {
            Some(T::example(config))
        }
    }
}

impl<T: Example> Example for Vec<T> {
    fn example(config: ExampleConfig) -> Self {
        if config.minimal {
            vec![]
        } else {
            (1..=3)
                .map(|index| {
                    T::example(ExampleConfig {
                        index: Some(index),
                        ..config.clone()
                    })
                })
                .collect()
        }
    }
}

macro_rules! example_from_default_for {
    ($typ:ty) => {
        impl Example for $typ {
            fn example(_: ExampleConfig) -> Self {
                Default::default()
            }
        }
    };
}

example_from_default_for!(i8);
example_from_default_for!(i16);
example_from_default_for!(i32);
example_from_default_for!(i64);
example_from_default_for!(i128);

example_from_default_for!(u8);
example_from_default_for!(u16);
example_from_default_for!(u32);
example_from_default_for!(u64);
example_from_default_for!(u128);
example_from_default_for!(usize);

example_from_default_for!(f32);
example_from_default_for!(f64);

example_from_default_for!(bool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_string() {
        assert_eq!("a string", String::example(Default::default()));
        assert_eq!(
            "<foo bar>",
            String::example(ExampleConfig {
                field_name: Some("foo_bar".into()),
                ..Default::default()
            })
        );
    }
}
