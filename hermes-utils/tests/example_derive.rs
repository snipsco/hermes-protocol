use hermes_utils::Example;

#[derive(Example, Debug, PartialEq)]
struct DemoExampleDerive {
    value_from_name_string: String,
    #[example_value("hello world")]
    overridden_string: String,
    optional_string: Option<String>,
    zeroed_i32: i32,
    #[example_value(5)]
    overridden_i32: i32,
    false_boolean: bool,
    #[example_value(true)]
    overridden_boolean: bool,
    struct_implementing_example: Option<NumericTypesSupported>,
    #[example_value(DummyStruct { fizz: Some("buzz".into()) })]
    struct_implementing_not_example: DummyStruct,
    vec: Vec<SimpleStruct>,
    #[example_value(get_example_vec())]
    overridden_vec: Vec<SimpleStruct>,
}

#[derive(Example, Debug, PartialEq)]
struct NumericTypesSupported {
    default_i8: i8,
    default_i16: i16,
    default_i32: i32,
    default_i64: i64,
    default_i128: i128,
    default_u8: u8,
    default_u16: u16,
    default_u32: u32,
    default_u64: u64,
    default_u128: u128,
    default_usize: usize,
    default_f32: f32,
    default_f64: f64,
}

#[derive(Example, Debug, PartialEq)]
struct SimpleStruct {
    name: String,
}

#[derive(Debug, PartialEq)]
struct DummyStruct {
    fizz: Option<String>,
}

fn get_example_vec() -> Vec<SimpleStruct> {
    vec![
        SimpleStruct { name: "hello".into() },
        SimpleStruct { name: "world".into() },
    ]
}

#[test]
fn full_example_works() {
    assert_eq!(
        DemoExampleDerive::full_example(),
        DemoExampleDerive {
            value_from_name_string: "<value from name string>".into(),
            overridden_string: "hello world".into(),
            optional_string: Some("<optional string>".into()),
            struct_implementing_example: Some(NumericTypesSupported {
                default_i8: 0,
                default_i16: 0,
                default_i32: 0,
                default_i64: 0,
                default_i128: 0,
                default_u8: 0,
                default_u16: 0,
                default_u32: 0,
                default_u64: 0,
                default_u128: 0,
                default_usize: 0,
                default_f32: 0.0,
                default_f64: 0.0,
            }),
            zeroed_i32: 0,
            overridden_i32: 5,
            false_boolean: false,
            overridden_boolean: true,
            struct_implementing_not_example: DummyStruct {
                fizz: Some("buzz".into())
            },
            vec: vec![
                SimpleStruct {
                    name: "<name 1>".into()
                },
                SimpleStruct {
                    name: "<name 2>".into()
                },
                SimpleStruct {
                    name: "<name 3>".into()
                }
            ],
            overridden_vec: get_example_vec(),
        }
    )
}

#[test]
fn minimal_example_works() {
    assert_eq!(
        DemoExampleDerive::minimal_example(),
        DemoExampleDerive {
            value_from_name_string: "<value from name string>".into(),
            overridden_string: "hello world".into(),
            optional_string: None,
            struct_implementing_example: None,
            zeroed_i32: 0,
            overridden_i32: 5,
            false_boolean: false,
            overridden_boolean: true,
            struct_implementing_not_example: DummyStruct {
                fizz: Some("buzz".into())
            },
            vec: vec![],
            overridden_vec: get_example_vec(),
        }
    )
}
