pub mod TaskManager {

    pub trait Print_Tasks {
        fn print_info(&self);
    }

    use std::collections::HashMap;

    use crate::task::Task::*;

    #[derive(Debug)]
    pub struct Storage {
        pub tasks : HashMap<u32 , new_task>,
    }

    impl Print_Tasks for Storage {
        fn print_info(&self) {
            println!("Tasks : {:?}" , self.tasks);
        }
    }

    impl Storage {
        pub fn new_storage() -> Storage {
            Storage {
                tasks : HashMap::new(),
            }
        }

        pub fn add_task(&mut self , task_id : u32 , task_details : new_task) -> Option<String> {
            self.tasks.insert(task_id, task_details);
            Some("Added successfully".to_string())
        }

        pub fn remove_task(&mut self , task_id : u32) {
            self.tasks.remove(&task_id);
        }

        pub fn search_task_by_id(&self , id : u32) -> Result<() , String>{
            for (_ , task) in &self.tasks {
                if task.id == id {
                    println!("{:?}" , task);
                    return Ok(());
                }else {
                    return Err("No such task found!".to_string());
                }
            }
            Err("No such task found".to_string())
        }
    }
}