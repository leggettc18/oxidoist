use oxidoist_api::TodoistAPI;
use oxidoist_api::TodoistAPIError;
use oxidoist_api::{Project, Task};
use structopt::StructOpt;

use std::env;

#[derive(StructOpt, Debug)]
enum Cli {
    /// Get data from your todoist account.
    Get {
        #[structopt(subcommand)]
        category: Category,
    },
}

#[derive(StructOpt, Debug)]
enum Category {
    /// Get all Projects on your account.
    Projects(ProjectsArgs),
    /// Get Project by its id.
    Project(ProjectArgs),
    /// Get all active tasks.
    Tasks(TasksArgs),
}

#[derive(StructOpt, Debug)]
struct ProjectsArgs {}

#[derive(StructOpt, Debug)]
struct ProjectArgs {
    /// The Project's ID. Use the `get projects` command to find the project with the id you want.
    id: u64,
}

#[derive(StructOpt, Debug)]
struct TasksArgs {
    /// Supply a project ID to filter the active tasks to those that are part of a particular project.
    #[structopt(short)]
    project_id: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), TodoistAPIError> {
    let args = Cli::from_args();
    let token = env::var("TODOIST_API_KEY").unwrap();
    let todoist_api_object = TodoistAPI::new(token.as_str()).unwrap();
    match args {
        Cli::Get { category } => match category {
            Category::Projects(_) => {
                let projects: Vec<Project> = todoist_api_object.get_projects().await?;
                println!("{:#?}", projects);
            }
            Category::Project(project) => {
                let project: Project = todoist_api_object.get_project(project.id).await?;
                println!("{:#?}", project);
            }
            Category::Tasks(args) => match args.project_id {
                Some(id) => {
                    let tasks: Vec<Task> = todoist_api_object.get_project_tasks(id).await?;
                    println!("{:#?}", tasks);
                }
                None => {
                    let tasks: Vec<Task> = todoist_api_object.get_tasks().await?;
                    println!("{:#?}", tasks);
                }
            },
        },
    }
    Ok(())
}
