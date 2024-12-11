use gh_workflow_tailcall::Workflow;

#[test]
fn generate() {
    Workflow::default()
        .auto_release(true)
        .auto_fix(true)
        .generate()
        .unwrap();
}
