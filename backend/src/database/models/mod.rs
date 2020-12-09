mod class;
mod student;
mod teacher;
mod badge;

pub use class::*;
pub use student::*;
pub use teacher::*;
pub use badge::*;

/// Id for types in the database
/// `varchar(50)`
pub type Id = String;

#[macro_export]
macro_rules! params {
    () => {
        rocket_contrib::databases::rusqlite::NO_PARAMS
    };
    ($($param:expr),+ $(,)?) => {
        &[$(&$param as &dyn rocket_contrib::databases::rusqlite::ToSql),+] as &[&dyn rocket_contrib::databases::rusqlite::ToSql]
    };
}
