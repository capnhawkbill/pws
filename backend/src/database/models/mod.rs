mod class;
mod student;
mod teacher;

pub use class::*;
pub use student::*;
pub use teacher::*;

/// Id for types in the database
/// `varchar(50)`
pub type Id = String;
