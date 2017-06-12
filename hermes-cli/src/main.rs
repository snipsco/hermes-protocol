extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hermes_protocol;
use hermes_protocol::*;

fn main() {
    println!("Hello, world!");

    let slot = Slot {
        value: SlotValue::Custom("value".into()),
        range: None,
        entity: "toto".into(),
        slot_name: "toto".into(),
    };
    println!("{}", serde_json::to_string(&SlotValue::Custom("toto".into())).unwrap());

    let a = SlotValue::Builtin(BuiltinEntity::Ordinal(OrdinalValue(5)));
    println!("{}", serde_json::to_string(&a).unwrap());

}
