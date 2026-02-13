#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Keys,
    Exit,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    Value(String),
    Int(i64),
    Keys(Vec<String>),
    Null,
    HelpText(String),
    Error(String),
}