use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub uuid: String,
    pub id: Option<i64>,
    pub description: String,
    pub entry: String,
    pub modified: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub udas: Map<String, Value>
}

impl Task {
    pub fn sort_order(&self) -> Result<f64> {
        let sort_order = match self.udas.get("sortOrder") {
            Some(v) => serde_json::to_string(v)?,
            None => "0.0".to_string()
        };
        Ok(sort_order.parse::<f64>()?)
    }
}

#[cfg(test)]
pub mod tests {
    use anyhow::Result;
    use serde_json::Value;
    use crate::model::Task;

    #[test]
    pub fn can_deserialize_basic_add() -> Result<()> {
        // if you just run `t add foo`, it'll look like this
        let task: Task = serde_json::from_str(r#"{"description":"foo","entry":"20230214T053646Z","modified":"20230214T053646Z","status":"pending","uuid":"e237cf7e-298f-4941-9fbc-f4df6de523c8"}"#)?;
        assert_eq!(task.description, "foo");
        assert_eq!(task.entry, "20230214T053646Z");
        assert_eq!(task.modified, "20230214T053646Z");
        assert_eq!(task.status, "pending");
        assert_eq!(task.uuid, "e237cf7e-298f-4941-9fbc-f4df6de523c8");
        Ok(())
    }

    #[test]
    pub fn can_deserialize_with_tags() -> Result<()> {
        // t add +personal foo
        let task: Task = serde_json::from_str(r#"{"description":"foo","entry":"20230214T053852Z","modified":"20230214T053852Z","status":"pending","uuid":"07dec074-9502-4035-9750-96c25983200e","tags":["personal"]}"#)?;
        assert_eq!(task.tags, vec!["personal"]);
        Ok(())
    }

    #[test]
    pub fn collects_unmapped_fields() -> Result<()> {
        // t add url:foo foo
        let task: Task = serde_json::from_str(r#"{"description":"foo","entry":"20230214T055231Z","modified":"20230214T055231Z","status":"pending","url":"foo","uuid":"39ecff7f-75d8-4194-9759-f7415508a203"}"#)?;
        assert_eq!(task.udas.get("url"), Some(&Value::String("foo".to_string())));
        println!("{:?}", task);
        println!("{}", serde_json::to_string(&task)?);
        Ok(())
    }
}