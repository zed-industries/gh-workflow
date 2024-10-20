use gh_workflow_rs::*;
fn main() {
    let workflow = Workflow::new("CI".to_string());
    std::fs::write("workflow.yml", workflow.to_string().unwrap()).unwrap();
}
