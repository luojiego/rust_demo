#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Environment {
    Development,
    Staging,
    Production,
}