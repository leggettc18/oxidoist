# Oxidoist

A CLI client for Todoist written in Rust. Utilizes the [oxidoist-api](https://github.com/leggettc18/oxidoist-api)
also made by me. Neither of these projects is complete or anywhere near production ready at the moment.

## Usage

If you want to use it, you will need to pull the repos for this and the corresponding `oxidoist-api` crate and build them from source. Since both projects are under active
development and not ready to be posted to crates.io, the API crate is listed as a file
dependency such that if both projects' folders are in the same parent folder, building
this project will also build the api crate. So do the following:

```bash
mkdir oxidoist && cd oxidoist
git clone https://github.com/leggettc18/oxidoist-api
git clone https://github.com/leggettc18/oxidoist
cd oxidoist
cargo build
```

At this point the `target/debug` folder will have an oxidoist executable file you can
either run directly from there or move to a more desireable location. While in the same directory as this executable, you can run something like the following.

Obviously as the API crate gets more feature complete it will eventually be uploaded to crates.io and these steps will no longer apply.

```bash
TODOIST_API_KEY='<todoist-api-key>' ./oxidoist get projects
```

This will return all the projects in your todoist account.

The basic structure of the commands will be `oxidoist <verb> <category> <arguments> <subcategory-if-applicable>`.

- `verb`: get, post, update, delete, complete, etc.
- `category`: project, section, task, label, etc. Also plurals for all fo those.
- `arguments`: id numbers, flags for filtering options (i.e. `oxidoist get tasks -p <project_id>` to get tasks belonging to a specific project), etc.
- `subcategory`: mostly same options as category, if one category "contains" items of another category (i.e. `oxidoist get project <id> tasks` for getting tasks attached to a specific project).

Currently not all commands have been implemented. At the time of writing this, `get` and `create` are the only initial commands that have any subcommands implemented. In the case of `get`, for all categories other than comments, at least the plural subcommands have been implemented (which basically means you can get items of a category on your account). Some more specific queries can be made as well (such as all tasks or sections of a specific project, either by filtering arguments or by specifying a subcategory after getting a project). Watch this space, as things are progressing quite quickly. In the case of `create`, you can currently create a task and optionally supply a project id and a due string (i.e. "tomorrow at noon", or "every day").
