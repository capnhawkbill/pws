pub type HomeworkId = String;
use chrono::naive::NaiveDate;

/// A homework assignment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Homework {
    /// Name/Title of the homework
    pub name: String,
    /// Due date of the homework
    pub date: NaiveDate,
    /// Description of the homework
    pub description: String,
    /// Points awarded when homework is finished
    pub points: i32,
}

impl Homework {
    /// Create an id from the name and the date
    /// [name]:[date]
    pub fn id(&self) -> HomeworkId {
        format!("{}:{}", self.name, self.date.to_string())
    }
}
