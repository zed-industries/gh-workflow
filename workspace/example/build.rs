use gh_workflow_rs::*;
fn main() {
    let workflow = Workflow::default();
    std::fs::write("workflow.yml", workflow.to_string().unwrap()).unwrap();
}
