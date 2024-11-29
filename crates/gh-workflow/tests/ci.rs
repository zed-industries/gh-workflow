use ctx::Context;
use gh_workflow::*;
use release_plz::{Command, Release};
use toolchain::Toolchain;

#[test]
fn generate() {
    let flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .permissions(Permissions::default().contents(Level::Read))
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        );

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request(
            PullRequest::default()
                .add_type(PullRequestType::Opened)
                .add_type(PullRequestType::Synchronize)
                .add_type(PullRequestType::Reopened)
                .add_branch("main"),
        );

    let permissions = Permissions::default()
        .pull_requests(Level::Write)
        .packages(Level::Write)
        .contents(Level::Write);

    let is_main = Context::github().ref_().eq("refs/heads/main".into());
    let is_push = Context::github().event_name().eq("push".into());
    let cond = is_main.and(is_push);

    let release = Job::new("Release")
        .cond(cond.clone())
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions.clone())
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::Release));

    let release_pr = Job::new("Release PR")
        .cond(cond.clone())
        .concurrency(
            Concurrency::new(Expression::new("release-${{github.ref}}")).cancel_in_progress(false),
        )
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions)
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::ReleasePR));

    Workflow::new("CI")
        .add_env(flags)
        .on(event)
        .add_job("build", build)
        .add_job("release", release)
        .add_job("release-pr", release_pr)
        .generate()
        .unwrap();
}
