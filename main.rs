use std::collections::HashMap;

use crate::taskmanager::TaskManager::Print_Tasks;

mod task;
mod taskmanager;

fn main() {
    use crate::task::Task::*;
    use crate::taskmanager::TaskManager::Storage;

    let mut new_task_manager = Storage::new_storage();

    let new_task1 = new_task {
        id : 1,
        title : "Rust Programming".to_string(),
        description : "Rusts Trait Bounds and generics".to_string(),
        status : TaskStatus::Completed,
    };
    new_task1.print_details();

    let new_task2 = new_task {
        id : 2,
        title : "Rust Traits Bounds".to_string(),
        description : "Diving into Rust trait Bounds".to_string(),
        status : TaskStatus::ToDo,
    };

    new_task_manager.add_task(1, new_task1);
    new_task_manager.add_task(2, new_task2);

    new_task_manager.print_info();

    new_task_manager.remove_task(1);

    let searched_task = new_task_manager.search_task_by_id(1);
    println!("{:?}" , searched_task);
}

