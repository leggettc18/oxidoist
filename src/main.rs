use oxidoist_api::{
    api::{TodoistAPI, TodoistAPIError},
    project::Project,
    section::Section,
    task::Task,
};
use structopt::clap::arg_enum;
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
    /// Get all Sections.
    Sections(SectionsArgs),
}

arg_enum! {
    #[derive(Debug)]
    enum ProjectSubCategory {
        Tasks
    }
}

#[derive(StructOpt, Debug)]
struct ProjectsArgs {}

#[derive(StructOpt, Debug)]
struct ProjectArgs {
    /// The Project's ID. Use the `get projects` command to find the project with the id you want.
    id: u64,
    /// Get subclass items belonging to the given project (sections, tasks, etc).
    #[structopt(possible_values = &ProjectSubCategory::variants(), case_insensitive = true)]
    sub_category: Option<ProjectSubCategory>,
}

#[derive(StructOpt, Debug)]
struct TasksArgs {
    /// Supply a project ID to filter the active tasks to those that are part of a particular project.
    #[structopt(short)]
    project_id: Option<u64>,
}

#[derive(StructOpt, Debug)]
struct SectionsArgs {
    /// Supply a project ID to filter the sections to those that are part of a particular project
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
                let projects: Vec<Project> = Project::get_all(&todoist_api_object).await?;
                println!("{:#?}", projects);
            }
            Category::Project(project_args) => {
                let project: Project = Project::get(project_args.id, &todoist_api_object).await?;
                match project_args.sub_category {
                    None => {
                        println!("{:#?}", project);
                    }
                    Some(sub) => match sub {
                        ProjectSubCategory::Tasks => {
                            let tasks: Vec<Task> = project.get_tasks(&todoist_api_object).await?;
                            println!("{:#?}", tasks);
                        }
                    },
                }
            }
            Category::Tasks(args) => match args.project_id {
                Some(id) => {
                    let project: Project = Project::get(id, &todoist_api_object).await?;
                    let tasks: Vec<Task> = project.get_tasks(&todoist_api_object).await?;
                    println!("{:#?}", tasks);
                }
                None => {
                    let tasks: Vec<Task> = Task::get_all(&todoist_api_object).await?;
                    println!("{:#?}", tasks);
                }
            },
            Category::Sections(args) => match args.project_id {
                Some(id) => {
                    let project: Project = Project::get(id, &todoist_api_object).await?;
                    let sections: Vec<Section> = project.get_sections(&todoist_api_object).await?;
                    println!("{:#?}", sections);
                }
                None => {
                    let sections: Vec<Section> = Section::get_all(&todoist_api_object).await?;
                    println!("{:#?}", sections);
                }
            },
        },
    }
    Ok(())
}
