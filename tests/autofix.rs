use gh_workflow::generate::Generate;
use gh_workflow::toolchain::Toolchain;
use gh_workflow::*;

#[test]
fn autofix() {
    let lint_mode_condition =
        "contains(github.event.pull_request.labels.*.name, 'ci: lintfix') && 'fix' || 'check'";
    let permissions = Permissions::default()
        .pull_requests(Level::Write)
        .packages(Level::Write)
        .contents(Level::Write);

    let lint_job = Job::new("Run Formatter and Lint Check")
        .runs_on("ubuntu-latest")
        .permissions(permissions)
        .add_env(("LINT_MODE", format!("${{{{{}}}}}", lint_mode_condition)))
        .add_step(Step::checkout())
        .add_step(
            Step::run("echo $LINT_MODE")
                .add_env(("LINT_MODE", "${{ env.LINT_MODE }}"))
                .name("Print $LINT_MODE"),
        )
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all --all-targets --all-features --fix --allow-staged --allow-dirty")
                .if_condition("env.LINT_MODE == 'fix'")
                .name("Cargo Clippy Fix"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--all")
                .if_condition("env.LINT_MODE == 'fix'")
                .name("Cargo Fmt fix"),
        )
        .add_step(
            Step::uses(
                "autofix-ci",
                "action",
                "ff86a557419858bb967097bfc916833f5647fa8c",
            )
            .if_condition(Expression::new("env.LINT_MODE == 'fix'"))
            .name("Commit and push if changed"),
        );

    let concurrency = Concurrency::default()
        .group("${{ github.workflow }}-${{ github.ref }}")
        .cancel_in_progress(true);

    let permissions = Permissions::default()
        .pull_requests(Level::Read)
        .packages(Level::Read)
        .contents(Level::Read);

    let workflow = Workflow::new("autofix.ci")
        .on(Event::default()
            .push(Push::default().add_branch("main"))
            .pull_request(
                PullRequest::default()
                    .add_branch("main")
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Reopened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Labeled),
            ))
        .permissions(permissions)
        .concurrency(concurrency)
        .add_job("lint", lint_job);

    Generate::new(workflow)
        .name("autofix.yml")
        .generate()
        .unwrap()
}
