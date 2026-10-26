use serde::{Serialize,Deserialize};
use chrono::Local;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
    created_at: String,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        let now = Local::now().format("%Y-%m-%d %H:%M").to_string();
        Self {
            id,
            title:String::from(title),
            done: false,
            created_at: now,
        }
    }
}

const TODO_FILE: &str = "D:\\MyProject\\CLI_todo\\tasks.json";

fn load_tasks() -> Vec<Task> {
    let path = Path::new(TODO_FILE);
    let data = std::fs::read_to_string(path).expect(&format!("cannot read {}\n", TODO_FILE));
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("error: serialize failed.");
    std::fs::write(TODO_FILE, json).expect(&format!("error: cannot write {}.\n", TODO_FILE));
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(String::from("error: CLI_todo need args to work.\ntry -h or --help for more help.\n"));
    }
    if args.len() > 3 {
        return Err(String::from("error: args are not valid.\ntry -h or --help for more help.\n"));
    }
    let mut tasks = load_tasks();
    match args[1].as_str() {
        "list" => {
            if args.len() == 3 {
                return Err(format!("error: unexpected {} found in args.\n",args[2]))
            }
            if tasks.is_empty() {
                print!("no tasks waiting to be done.\n");
                return Ok(());
            }
            for task in &tasks {
                if !task.done {
                    continue;
                }
                print!("[{}]    #{} {} ({})\n", "done", task.id, task.title, task.created_at);
            }
            for task in &tasks {
                if task.done {
                    continue;
                }
                print!("[{}] #{} {} ({})\n", "pending", task.id, task.title, task.created_at);
            }
        },
        "add" => {
            if args.len() == 2 {
                return Err(String::from("error: title can not be null.\n"));
            }
            let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            let new_task = Task::new(id, &args[2]);
            tasks.push(new_task);
            save_tasks(&tasks);
        },
        "done" => {
            if args.len() == 2 {
                return Err(String::from("error: id can not be null.\n"));
            }
            if let Ok(id) = args[2].parse::<u32>() {
                if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                    task.done = true;
                } else {
                    return Err(format!("error: no tasks own id as {}.\n", id));
                }
            } else {
                return Err(format!("error: {} is invalid id.\n", args[2]));
            }
            save_tasks(&tasks);
        },
        "remove" => {
            if args.len() == 2 {
                return Err(String::from("error: id can not be null.\n"));
            }
            if let Ok(id) = args[2].parse::<u32>() {
                let len_before = tasks.len();
                tasks.retain(|task| task.id != id);
                if tasks.len() == len_before {
                    return Err(format!("error: no tasks own id as {}.\n", id));
                } else {
                    return Ok(());
                }
            } else {
                return Err(format!("{} is invalid id.\n", args[2]));
            }
        },
        "clear" => {
            if args.len() == 3 {
                return Err(format!("error: unexpected {} found in args.\n", args[2]));
            }
            tasks.retain(|task| task.done);
            save_tasks(&tasks);
            print!("all done tasks havs been removed.\n");
        },
        _=> {

        },
    }
    return Ok(());
}
