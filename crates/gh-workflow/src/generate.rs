//! This module provides functionality to customize generation of the GitHub
//! Actions workflow files.

use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use derive_setters::Setters;
use indexmap::IndexMap;

use crate::error::{Error, Result};
use crate::{Job, Jobs, Workflow};

#[derive(Setters, Clone)]
#[setters(into)]
pub struct Generate {
    workflow: Workflow,
    name: String,
}

impl Generate {
    pub fn new(workflow: Workflow) -> Self {
        let workflow = organize_job_dependency(workflow);
        Self { workflow, name: "ci.yml".to_string() }
    }

    fn check_file(&self, path: &PathBuf, content: &str) -> Result<()> {
        if let Ok(prev) = std::fs::read_to_string(path) {
            if content != prev {
                Err(Error::OutdatedWorkflow)
            } else {
                Ok(())
            }
        } else {
            Err(Error::MissingWorkflowFile(path.clone()))
        }
    }

    pub fn generate(&self) -> Result<()> {
        let comment = include_str!("./comment.yml");

        let root_dir = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "--show-toplevel"])
                .output()?
                .stdout,
        )?;

        let path = PathBuf::from(root_dir.trim())
            .join(".github")
            .join("workflows")
            .join(self.name.as_str());

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        let result = self.check_file(&path, &content);

        if std::env::var("CI").is_ok() {
            result
        } else {
            match result {
                Ok(()) => {
                    println!("Workflow file is up-to-date: {}", path.display());
                    Ok(())
                }
                Err(Error::OutdatedWorkflow) => {
                    std::fs::write(path.clone(), content)?;
                    println!("Updated workflow file: {}", path.display());
                    Ok(())
                }
                Err(Error::MissingWorkflowFile(path)) => {
                    std::fs::create_dir_all(path.parent().ok_or(Error::IO(
                        std::io::Error::new(ErrorKind::Other, "Invalid parent dir(s) path"),
                    ))?)?;
                    std::fs::write(path.clone(), content)?;
                    println!("Generated workflow file: {}", path.display());
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    }
}

/// Organizes job dependencies within a given `Workflow`.
///
/// This function iterates over all jobs in the provided `Workflow` and ensures
/// that each job's dependencies are correctly set up. If a job has dependencies
/// specified in `tmp_needs`, it checks if those dependencies are already
/// defined in the workflow. If not, it creates new job IDs for the missing
/// dependencies and inserts them into the workflow. The function then updates
/// the `needs` field of each job with the appropriate job IDs.
fn organize_job_dependency(mut workflow: Workflow) -> Workflow {
    let mut job_id = 0;
    let mut new_jobs = IndexMap::<String, Job>::new();
    let empty_map = IndexMap::default();

    let old_jobs: &IndexMap<String, Job> = workflow
        .jobs
        .as_ref()
        .map(|jobs| &jobs.0)
        .unwrap_or(&empty_map);

    // Iterate over all jobs
    for (id, mut job) in workflow.jobs.clone().unwrap_or_default().0.into_iter() {
        // If job has dependencies
        if let Some(dep_jobs) = &job.tmp_needs {
            // Prepare the job_ids
            let mut job_ids = Vec::<String>::new();
            for job in dep_jobs.iter() {
                // If the job is already available
                if let Some(id) = find_value(job, &new_jobs).or(find_value(job, &old_jobs)) {
                    job_ids.push(id.to_owned());
                } else {
                    // Create a job-id for the job
                    let id = format!("job-{}", job_id);

                    // Add job id as the dependency
                    job_ids.push(id.clone());

                    // Insert the missing job into the new_jobs
                    new_jobs.insert(format!("job-{}", job_id), job.clone());

                    job_id += 1;
                }
            }
            job.needs = Some(job_ids);
        }

        new_jobs.insert(id.clone(), job.clone());
    }

    workflow.jobs = Some(Jobs(new_jobs));

    workflow
}

fn find_job<'a>(
    dep_job: &Job,
    new_jobs: &'a IndexMap<String, Job>,
    workflow: &'a Workflow,
) -> Option<&'a str> {
    let in_new_jobs: Option<&'a str> =
        new_jobs
            .iter()
            .find_map(|(k, v)| if v == dep_job { Some(k.as_str()) } else { None });

    let in_old_jobs: Option<&'a str> = workflow.jobs.as_ref().and_then(|jobs| {
        jobs.0.iter().find_map(|(id, j)| {
            if j == dep_job {
                Some(id.as_str())
            } else {
                None
            }
        })
    });

    in_new_jobs.or(in_old_jobs)
}
