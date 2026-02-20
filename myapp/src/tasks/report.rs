use loco_rs::prelude::*;

pub struct Report;
#[async_trait]
impl Task for Report {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "report".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task Report generated");
        Ok(())
    }
}
