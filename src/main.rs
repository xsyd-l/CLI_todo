use xsyd_json::Json;
use std::io::ErrorKind;

const TASKSPATH: &str = "./task.json";

fn load_tasks() -> Result<Json, std::io::Error>{
    match std::fs::read_to_string(TASKSPATH) {
        Ok(content) => {
            Result::Ok(Json::from(&content))
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    return Result::Ok(Json::Null);
                },
                _ => {
                    return Result::Err(e);
                }
            }
        }
    }
}

fn save_tasks(json: &Json) -> Result<(), std::io::Error> {
    json.write(TASKSPATH)
}

fn main() {
    let args: Vec<String>= std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "list" => {
                match load_tasks() {
                    Ok(tasks) => {
                        print!("{}",tasks.to_string());
                    },
                    _ => {
                        eprint!("\x1b[31merror\x1b[0m: can not read \x1b[33m{}\x1b[0m\n", TASKSPATH);
                        panic!();
                    },
                }
            },
            "-v" | "--version" => {
                print!("\x1b[32mversion\x1b[0m: 0.1.1");
            },
            "-h" | "--help" => {
                let a   = "    -v  --version: get edition informaiton\n";
                let b   = "    -h     --help: get informaiton about more operations\n";
                let c   = "    list   + task_id: list all tasks\n";
                let d   = "    add    +   title: add a task on your todo list\n";
                let e   = "    remove + task_id: remove a task from your todo list\n";
                let e1  = "        use -c option to clear tasks marked as done\n";
                let f   = "    done   + task_id: mark a task as done\n";
                print!("options:\n{}{}subcommand:\n{}{}{}{}{}", a, b, c, d, e, e1, f);
            },
            _ => {
                eprint!("\x1b[31merror\x1b[0m: invalid option \x1b[33m{}\x1b[0m.\ntry -h or --help for more information.\n", args[1].as_str());
            }
        }
        return;
    }

    if args.len() == 3 {
        match args[1].as_str() {
            "add" => {
                match load_tasks() {
                    Ok(mut tasks) => {
                        let time: String = chrono::Local::now().to_string();
                        let id: i32 = match &tasks {
                            Json::Array(vec) => {
                                let mut max = 0;
                                for json in vec {
                                    if let Json::Object(map) = json {
                                        if let Some(json) = map.get("id") {
                                            if let Json::Int(id) = json {
                                                if id > &max {
                                                    max = id.clone();
                                                }
                                            } else {
                                                eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id is not a valid value (it must be in the range 0–255)\n")
                                            }
                                        } else {
                                            eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id doesn't exist.\n");
                                            panic!();
                                        }
                                    } else {
                                        eprint!("\x1b[31merror\x1b[0m: \nThe Array in \x1b[33m{}\x1b[0m is only allowed to contain JSON objects of type Object.\n", TASKSPATH);
                                        panic!();
                                    }
                                }
                                max
                            },
                            Json::Null => {
                                0
                            },
                            _ => {
                                eprint!("{} must be an array.\n", TASKSPATH);
                                panic!();
                            }
                        } + 1;
                        let title = args[2].as_str();
                        let new_task = format!("{{\n\"id\":{},\n\"title\":\"{}\",\n\"done\":false,\n\"time\":\"{}\"\n}}", id, title, time);
                        
                        match &mut tasks {
                            Json::Null => {
                                tasks = Json::from(&format!("[\n{}\n]", new_task));
                            },
                            Json::Array(vec) => {
                                vec.push(Json::from(&new_task));
                            },
                            _ => {}
                        }
                        match save_tasks(&tasks) {
                            Ok(_) => {
                                print!("\x1b[32mnew task has already been saved.\x1b[0m")
                            },
                            Err(e)=>{
                                eprint!("{}", e);
                            }
                        }
                    },
                    _ => {
                        eprint!("\x1b[31merror\x1b[0m: can not read \x1b[33m{}\x1b[0m\n", TASKSPATH);
                        panic!();
                    }
                }
            },
            "remove" => {
                if let Ok(mut tasks) = load_tasks() {
                    let aim: i32;
                    if let Ok(aim_id) = args[2].parse::<i32>() {
                        aim = aim_id;
                    } else {
                        eprint!("\x1b[31merror\x1b[0m: {} is not a valid id.\n", args[2]);
                        panic!();
                    }
                    if let Json::Array(vec) = &mut tasks {
                        vec.retain(|json| {
                            let mut result = false;
                                if let Json::Object(map) = json {
                                    if let Some(json) = map.get("id") {
                                        if let Json::Int(id) = json {
                                            if id != &aim {
                                                result = true;
                                            }
                                        } else {
                                            eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id is not a valid value (it must be in the range 0–255)\n")
                                        }
                                    } else {
                                        eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id doesn't exist.\n");
                                        panic!();
                                    }
                                } else {
                                    eprint!("\x1b[31merror\x1b[0m: \nThe Array in \x1b[33m{}\x1b[0m is only allowed to contain JSON objects of type Object.\n", TASKSPATH);
                                    panic!();
                                }
                                result
                            }
                        );
                    } else {
                        eprint!("{} must be an array.\n", TASKSPATH);
                        panic!();
                    }
                    match save_tasks(&tasks) {
                        Ok(_) => {
                            print!("\x1b[32mnew task has already been saved.\x1b[0m")
                        },
                        Err(e)=>{
                            eprint!("{}", e);
                        }
                    }
                } else {
                        eprint!("\x1b[31merror\x1b[0m: can not read \x1b[33m{}\x1b[0m\n", TASKSPATH);
                        panic!();
                }
            },
            "done" =>{
                if let Ok(mut tasks) = load_tasks() {
                    let aim: i32;
                    if let Ok(aim_id) = args[2].parse::<i32>() {
                        aim = aim_id;
                    } else {
                        eprint!("\x1b[31merror\x1b[0m: {} is not a valid id.\n", args[2]);
                        panic!();
                    }
                    if let Json::Array(vec) = &mut tasks {
                        for json in vec {
                            if let Json::Object(map) = json {
                                if let Some(json) = map.get("id") {
                                    if let Json::Int(id) = json {
                                        if id == &aim {
                                            map.insert(String::from("done"), Json::Bool(true));
                                        }
                                    } else {
                                        eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id is not a valid value (it must be in the range 0–255)\n")
                                    }
                                } else {
                                    eprint!("\x1b[31merror\x1b[0m:\nParsing failed because a task's id doesn't exist.\n");
                                    panic!();
                                }
                            } else {
                                eprint!("\x1b[31merror\x1b[0m: \nThe Array in \x1b[33m{}\x1b[0m is only allowed to contain JSON objects of type Object.\n", TASKSPATH);
                                panic!();
                            }
                        }
                    } else {
                        eprint!("{} must be an array.\n", TASKSPATH);
                        panic!();
                    }
                    match save_tasks(&tasks) {
                        Ok(_) => {
                            print!("\x1b[32mnew task has already been saved.\x1b[0m")
                        },
                        Err(e)=>{
                            eprint!("{}", e);
                        }
                    }
                } else {
                        eprint!("\x1b[31merror\x1b[0m: can not read \x1b[33m{}\x1b[0m\n", TASKSPATH);
                        panic!();
                }
            },
            _ => {
                eprint!("\x1b[31merror\x1b[0m: invalid option \x1b[33m{}\x1b[0m.\ntry -h or --help for more information.\n", args[1].as_str());
            },
        }
        return;
    }

    eprint!("\x1b[31merror\x1b[0m: too many args!\ntry -h or --help for more information.\n");
}