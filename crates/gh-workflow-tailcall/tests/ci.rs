use gh_workflow_tailcall::Workflow;

#[test]
fn generate() {
    Workflow::default().auto_release(true).generate().unwrap();
}
