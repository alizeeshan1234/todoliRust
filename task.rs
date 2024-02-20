pub mod Task {
    pub trait PrintInfo{
        fn print_details(&self);
    }

    #[derive(Debug)]
    pub struct new_task {
        pub id : u32,
        pub title : String,
        pub description : String,
        pub status : TaskStatus,
    }

    impl new_task {
        pub fn change_task_status(&mut self , new_status : TaskStatus){
            self.status = new_status;
        }
    }

    #[derive(Debug)]
    pub enum TaskStatus {
        ToDo,
        InProgress,
        Completed,
    }

    impl PrintInfo for new_task {
        fn print_details(&self) {
            println!("Task Id : {:?}" , self.id);
            println!("Task Title : {:?}" , self.title);
            println!("Task Description : {:?}" , self.description);
            println!("Task Status : {:?}" , self.status);
        }
    }
}