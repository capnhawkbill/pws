mod badge;
mod class;
mod student;
mod teacher;

pub use badge::*;
pub use class::*;
pub use student::*;
pub use teacher::*;

/// Id for types in the database
/// `TEXT NOT NULL PRIMARY KEY`
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
