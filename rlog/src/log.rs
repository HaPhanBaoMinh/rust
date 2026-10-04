use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}
