use errors::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content="value")]
pub enum BuiltinEntity {
    Number(NumberValue),
    Ordinal(OrdinalValue),
    Time(TimeValue),
    AmountOfMoney(AmountOfMoneyValue),
    Temperature(TemperatureValue),
    Duration(DurationValue),
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct NumberValue(pub f64);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct OrdinalValue(pub i64);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "kind", content="value")]
pub enum TimeValue {
    InstantTime(InstantTimeValue),
    TimeInterval(TimeIntervalValue)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InstantTimeValue {
    pub value: String,
    pub grain: Grain,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeIntervalValue {
    from: Option<String>,
    to: Option<String>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AmountOfMoneyValue {
    pub value: f32,
    pub precision: Precision,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TemperatureValue {
    pub value: f32,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DurationValue {
    pub years: i64,
    pub quarters: i64,
    pub months: i64,
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Grain {
    Year = 0,
    Quarter = 1,
    Month = 2,
    Week = 3,
    Day = 4,
    Hour = 5,
    Minute = 6,
    Second = 7,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinEntityKind {
    AmountOfMoney,
    Duration,
    Number,
    Ordinal,
    Temperature,
    Time,
}

impl BuiltinEntityKind {
    pub fn all() -> Vec<BuiltinEntityKind> {
        vec![
            BuiltinEntityKind::AmountOfMoney,
            BuiltinEntityKind::Duration,
            BuiltinEntityKind::Number,
            BuiltinEntityKind::Ordinal,
            BuiltinEntityKind::Temperature,
            BuiltinEntityKind::Time,
        ]
    }
}

impl BuiltinEntityKind {
    pub fn identifier(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "snips/amountOfMoney",
            BuiltinEntityKind::Duration => "snips/duration",
            BuiltinEntityKind::Number => "snips/number",
            BuiltinEntityKind::Ordinal => "snips/ordinal",
            BuiltinEntityKind::Temperature => "snips/temperature",
            BuiltinEntityKind::Time => "snips/datetime",
        }
    }

    pub fn from_identifier(identifier: &str) -> Result<BuiltinEntityKind> {
        Self::all()
            .into_iter()
            .find(|kind| kind.identifier() == identifier)
            .ok_or(format!("Unknown EntityKind identifier: {}", identifier).into())
    }

    pub fn from_builtin_entity(entity: &BuiltinEntity) -> BuiltinEntityKind {
        match entity {
            &BuiltinEntity::AmountOfMoney(_) => BuiltinEntityKind::AmountOfMoney,
            &BuiltinEntity::Duration(_) => BuiltinEntityKind::Duration,
            &BuiltinEntity::Number(_) => BuiltinEntityKind::Number,
            &BuiltinEntity::Ordinal(_) => BuiltinEntityKind::Ordinal,
            &BuiltinEntity::Temperature(_) => BuiltinEntityKind::Temperature,
            &BuiltinEntity::Time(_) => BuiltinEntityKind::Time,
        }
    }
}
