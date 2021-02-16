use structopt::StructOpt;
use oxidoist_api::TodoistAPI;
use oxidoist_api::Project;
use oxidoist_api::TodoistAPIError;

use std::env;

#[derive(StructOpt, Debug)]
struct Cli {
    verb: String, //get, add, complete, etc.
    datatype: String, //project, task, section, etc.
}

#[tokio::main]
async fn main() -> Result<(), TodoistAPIError> {
    let args = Cli::from_args();
    let token = env::var("TODOIST_API_KEY").unwrap();
    let todoist_api_object = TodoistAPI::new(token.as_str()).unwrap();
    if args.verb == "get" {
        if args.datatype == "projects" {
            let projects: Vec<Project> = todoist_api_object.get_projects().await?;
            println!("{:#?}", projects);
        }
    }
    
    Ok(())
}
