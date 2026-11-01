use xsyd_json::Json;
use xsyd_command_parser::Args;
use std::collections::HashMap;
use std::io::ErrorKind;

const HELPMSG: &str = "options:\n    -v  --version: get edition informaiton\n    -h     --help: get informaiton about more operations\nsubcommand:\n    list   + task_id: list all tasks\n    add    +   title: add a task on your todo list\n    remove + task_id: remove a task from your todo list\n        use -c option to clear tasks marked as done\n    done   + task_id: mark a task as done\n";

#[derive(Clone)]
struct Task {
    id:i32,
    title:String,
    done:bool,
    created_time:String,
}

impl Task {
    pub fn from_json(json: &Json) -> Result<Task, String> {
        let id: i32;
        let title: String;
        let done: bool;
        let created_time: String;
        if let Json::Object(map) = json {
            if !map.contains_key("id") {
                return Err(String::from("Field id has been lost."));
            }
            if let Json::Int(inner) = map["id"] {
                if inner < 0 {
                    return Err(String::from("The `id` field must be a non-negative number."))
                } else {
                    id = inner;
                }
            } else {
                return Err(String::from("Field id is not a Integer."));
            }
            if !map.contains_key("title") {
                return Err(String::from("Field title has been lost."));
            }
            if let Json::String(inner) = &map["title"] {
                title = String::from(inner);
            } else {
                return Err(String::from("Field title is not String."));
            }
            if !map.contains_key("done") {
                return Err(String::from("Field done has been lost."));
            }
            if let Json::Bool(inner) = &map["done"] {
                done = inner.to_owned();
            } else {
                return Err(String::from("Field done is not bool."));
            }
            if !map.contains_key("created_time") {
                return Err(String::from("Field created_time has been lost."));
            }
            if let Json::String(inner) = &map["created_time"] {
                created_time = String::from(inner);
            } else {
                return Err(String::from("Field created_time is not String."));
            }
            return Ok(Task{id, title, done ,created_time});
        } else {
            return Err(String::from("Attempt to construct Task with an object that is not a Json::Object."));
        }
    }

    pub fn to_json(&self) -> Json {
        let json_id = Json::Int(self.id);
        let json_title = Json::String(String::from(&self.title));
        let json_done = Json::Bool(self.done);
        let json_created_time = Json::String(String::from(&self.created_time));
        let mut map = HashMap::<String, Json>::new();
        map.insert(String::from("id"), json_id);
        map.insert(String::from("title"), json_title);
        map.insert(String::from("done"), json_done);
        map.insert(String::from("created_time"), json_created_time);
        Json::Object(map)
        }
}

fn load_tasks(path: &str) -> Result<Vec<Task>, String>{
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let mut tasks = Vec::<Task>::new();
            if let Json::Array(vec) = Json::from(&content) {
                for json in &vec {
                    tasks.push(Task::from_json(json)?);
                }
                return Ok(tasks);
            } else {
                return Err(format!("{} is not an array.", path));
            }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    return Result::Ok(Vec::<Task>::new());
                },
                _ => {
                    return Result::Err(format!("can not read {}.", path));
                }
            }
        }
    }
}

fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), std::io::Error> {
    let mut vec = Vec::<Json>::new();
    for task in tasks {
        vec.push(task.to_json());
    }
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    Json::Array(vec).write(path)
}

fn main() {
    let args = match Args::from(std::env::args().collect(), &[]) {
        Some(inner) => inner,
        None => {
            eprint!("\x1b[31merror\x1b[0m: Args can not be null.\n");
            return;
        }
    };
    let tasks_path = match std::env::var("APPDATA") {
        Ok(appdata) => {
            appdata+"/xsyd_todo/tasks.json"
        },
        _ => {
            eprint!("\x1b[31merror\x1b[0m: APPDATA has not been set.\n");
            return;
        }
    };
    let mut root_command = false;
    if  args.short_options.iter().any(|s| s == "-v") ||
        args.long_options.iter().any(|s| s == "--version") {
        print!("\x1b[32mversion\x1b[0m: 0.1.2");
        root_command = true;
    }
    if  args.short_options.iter().any(|s| s == "-h") ||
        args.long_options.iter().any(|s| s == "--help") {
        print!("{}", HELPMSG);
        root_command = true;
    }
    if root_command {
        return;
    }
    let mut tasks = match load_tasks(&tasks_path) {
        Ok(inner) => inner,
        Err(s) => {
            eprint!("\x1b[31merror\x1b[0m: {}\n", s);
            return;
        }
    };
    if let Some(subcommand) = args.subcommand {
        match subcommand.as_str() {
            "add" => {
                let id;
                let title = args.configs.join("");
                let created_time = chrono::Local::now().to_string();
                id = match tasks.iter().map(|t| t.id).max() {
                    Some(inner) => inner,
                    _ => 0,
                };
                let task = Task{id, title, done: false, created_time};
                tasks.push(task);
                match save_tasks(&tasks_path, &tasks) {
                    Ok(_) => {
                        print!("\x1b[32mnew task has been created.\x1b[0m\n");
                    },
                    Err(e) => {
                        eprint!("{}", e.to_string());
                    }
                }
                return;
            },
            "remove" => {
                let mut clear = false;
                if  args.short_options.iter().any(|s| s == "-c") ||
                    args.long_options.iter().any(|s| s== "--clear") {
                    clear = true;
                }
                if clear {
                    tasks.retain(|t| t.done == false);
                    match save_tasks(&tasks_path, &tasks) {
                        Ok(_) => {
                            print!("\x1b[32mall done tasks have been removed.\x1b[0m\n");
                            return;
                        },
                        Err(e) => {
                            eprint!("\x1b[31merror\x1b[0m:{}", e.to_string());
                            return;
                        }
                    }
                }
                if args.configs.len() == 0 {
                    eprint!("\x1b[31merror\x1b[0m:remove need a task id to work.\n", );
                    return;
                }
                if args.configs.len() > 1 {
                    eprint!("\x1b[31merror\x1b[0m:too many configs.\n");
                    return;
                }
                let aim_id = match args.configs[0].parse::<u32>() {
                    Ok(inner) => inner as i32,
                    _ => {
                        eprint!("\x1b[31merror\x1b[0m:{} is not an valid id.\n", args.configs[0]);
                        return;
                    }
                };
                let prev_len = tasks.len();
                tasks.retain(|t| t.id != aim_id);
                if tasks.len() != prev_len {
                    match save_tasks(&tasks_path, &tasks){
                        Ok(_) => print!("\x1b[31mremoved #{}\x1b[0m\n", aim_id),
                        Err(e) => {
                            eprint!("\x1b[31merror\x1b[0m:{}", e.to_string());
                            return;
                        }
                    }
                } else {
                    eprint!("\x1b[31merror\x1b[0m:no such task.\n");
                    return;
                }
            },
            "list" => {
                if tasks.is_empty() {
                    print!("no task has been created.\n");
                    return;
                }
                for task in &tasks {
                    if task.done {
                        print!("\x1b[32m#{} [{}] ({})\n{}\x1b[0m", task.id, task.done, task.created_time, task.title);
                    } else {
                        print!("\x1b[31m#{} [{}] ({})\n{}\x1b[0m", task.id, task.done, task.created_time, task.title);
                    }
                }
                return;
            },
            "done" => {
                if args.configs.len() == 0 {
                    eprint!("\x1b[31merror\x1b[0m:done need a task id to work.\n", );
                    return;
                }
                if args.configs.len() > 1 {
                    eprint!("\x1b[31merror\x1b[0m:too many configs.\n");
                    return;
                }
                let aim_id = match args.configs[0].parse::<u32>() {
                    Ok(inner) => inner as i32,
                    _ => {
                        eprint!("\x1b[31merror\x1b[0m:{} is not an valid id.\n", args.configs[0]);
                        return
                    }
                };
                for i in 0..tasks.len() {
                    if tasks[i].id == aim_id {
                        match tasks[i].done {
                            true => {
                                print!("#{} has already been done.\n", tasks[i].id);
                                return;
                            },
                            false => {
                                tasks[i].done = true;
                                match save_tasks(&tasks_path, &tasks) {
                                    Ok(_) => {
                                        print!("\x1b[32mmarked #{} as done.\x1b[0m\n", tasks[i].id);
                                        return;
                                    },
                                    Err(e) => {
                                        eprint!("\x1b31merror\x1b[0m:{}\n", e.to_string());
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                eprint!("\x1b[31merror\x1b[0m:no such task.\n");
                return;
            },
            _ => {}
        }
    } else {
        eprint!("\x1b[31merror\x1b[0m: args are not valid.\ntry -h or --help for more information.\n");
    }
}