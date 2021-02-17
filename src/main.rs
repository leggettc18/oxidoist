use structopt::StructOpt;
use oxidoist_api::TodoistAPI;
use oxidoist_api::Project as ApiProject;
use oxidoist_api::TodoistAPIError;

use std::env;

#[derive(StructOpt, Debug)]
enum Cli {
    Get {
        #[structopt(subcommand)]
        category: Category,
    }
}

#[derive(StructOpt, Debug)]
enum Category {
    Projects(Projects),
    Project(Project),
}

#[derive(StructOpt, Debug)]
struct Projects {

}

#[derive(StructOpt, Debug)]
struct Project {
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), TodoistAPIError> {
    let args = Cli::from_args();
    let token = env::var("TODOIST_API_KEY").unwrap();
    let todoist_api_object = TodoistAPI::new(token.as_str()).unwrap();
    match args {
        Cli::Get { category } => {
            match category {
                Category::Projects(_) => {
                    let projects: Vec<ApiProject> = todoist_api_object.get_projects().await?;
                    println!("{:#?}", projects);
                }
                Category::Project(project) => {
                    let project: ApiProject = todoist_api_object.get_project(project.id).await?;
                    println!("{:#?}", project);
                }
            }
        }
    }
    /* if args.verb == "get" {
        if args.datatype == "projects" {
            let projects: Vec<Project> = todoist_api_object.get_projects().await?;
            println!("{:#?}", projects);
        }
    } */
    
    Ok(())
}
