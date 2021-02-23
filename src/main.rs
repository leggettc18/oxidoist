use oxidoist_api::{
    api::{TodoistAPI, TodoistAPIError},
    label::Label,
    project::Project,
    section::Section,
    task::{CreateTaskParamsBuilder, Task},
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
    /// Create a task, project, section, etc.
    Create {
        #[structopt(subcommand)]
        category: CreateCategory,
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
    /// Get all Lablels.
    Labels(LabelsArgs),
}

#[derive(StructOpt, Debug)]
enum CreateCategory {
    /// Create a new task.
    Task(CreateTaskArgs),
}

#[derive(StructOpt, Debug)]
struct CreateTaskArgs {
    /// Task content.
    #[structopt(short)]
    content: String,
    #[structopt(short)]
    project_id: Option<u64>,
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

#[derive(StructOpt, Debug)]
struct LabelsArgs {}

#[tokio::main]
async fn main() -> Result<(), TodoistAPIError> {
    let args = Cli::from_args();
    let token = env::var("TODOIST_API_KEY")
        .expect("Please provide a TODOIST_API_KEY environment variable.)");
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
            Category::Labels(_) => {
                let labels: Vec<Label> = Label::get_all(&todoist_api_object).await?;
                println!("{:#?}", labels);
            }
        },
        Cli::Create { category } => match category {
            CreateCategory::Task(args) => {
                let new_task = CreateTaskParamsBuilder::default()
                    .content(args.content)
                    .project_id(args.project_id)
                    .build()
                    .map_err(TodoistAPIError::ParamsBuilderError)?;
                let task = Task::create(&new_task, &todoist_api_object).await?;
                println!("{:#?}", task);
            }
        },
    }
    Ok(())
}
