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
