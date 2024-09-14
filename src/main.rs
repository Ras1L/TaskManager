use chrono::{
    DateTime,
    Local
};
use serde::{
    Serialize,
    Deserialize
};
use std::{
    fs::File,
    io::{
        self, 
        BufReader, 
        Write
    },
    path::Path
};


#[derive(PartialEq, PartialOrd, Serialize, Deserialize)]
enum Priority
{
    None,

    Low,
    Medium,
    High,
    VeryHigh,
}

impl Priority
{
    fn to_string(&self) -> String
    {
        match self 
        {
            Priority::Low      => "Low".to_string(),
            Priority::Medium   => "Medium".to_string(),
            Priority::High     => "High".to_string(),
            Priority::VeryHigh => "Very High".to_string(),
            Priority::None     => "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task
{
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>
}

impl Task
{
    fn new(name: String, description: String, priority: Priority)
    -> Self
    {
        return Self
        {
            name,
            description,
            priority,
            add_time: Local::now()
        };
    }

    fn print(&self)
    {
        println!("{} | {} | {}\n\"{}\"",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y  %H:%M:%S"),
            self.description
        );
    }
}


struct TaskManager
{
    tasks: Vec<Task>
}

impl TaskManager
{
    fn new() -> Self
    {
        return Self { tasks: Vec::new() };
    }

    fn print(&self)
    {
        for task in self.tasks.iter()
        {
            task.print();
            print!("\n");
        }
    }

    fn sort(&mut self)
    {
        // self.tasks.sort();
    }

    fn push(&mut self, task: Task)
    {
        self.tasks.push(task);    
    }

    fn pop(&mut self) -> Option<Task>
    {
        return self.tasks.pop();
    }

    fn remove(&mut self, name: &str) -> Result<Task, String>
    {
        if let Some(index) = self.find(name)
        {
            return Ok(self.tasks.remove(index))
        }
        else
        {
            return Err(format!("Task {} not found", name))   
        }
    }

    fn find(&self, name: &str) -> Option<usize>
    {
        return self.tasks
            .iter()
            .position(|task: &Task| task.name.to_lowercase() == name.to_lowercase());
    }

    fn clear(&mut self)
    {
        self.tasks.clear();
    }

    fn store_to_file(&self, path: &str) -> Result<(), String>
    {
        if !Path::new(path).exists()
        {
            let file: File = match File::create(path)
            {
                Ok(file) => file,
                Err(e) => return Err(format!("Error to create file \"{}\": {}", path, e))
            };

            match serde_json::to_writer(&file, &self.tasks)
            {
                Ok(_) => {},
                Err(_) => {}
            }
        }
        Ok(())
    }
    
    fn read_from_file(&mut self, path: &str) -> Result<(), String>
    {
        if Path::new(path).exists()
        {
            let file: File = match File::open(path)
            {
                Ok(file) => file,
                Err(e) => return Err(format!("Error to open file: {}", e))
            };

            let reader: BufReader<File> = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader)
            {
                Ok(data) => data,
                Err(e)       => return Err(format!("Error to read file: {}", e))
            };
        }
        Ok(())
    }
}


struct ConsoleForTask
{
    my_tasks: TaskManager
}

impl ConsoleForTask
{
    fn new() -> Self
    {
        Self {
            my_tasks: TaskManager::new()
        }
    }

    fn print_menu()
    {
        println!("\nh - for help \n\n1. Add Task \n2. Pop Task \n3. Remove Task \n4. Find Task");
        println!("5. List of Tasks \n6. Remove all Tasks \n7. Store Tasks to file \n8. Read Tasks from file \n9. Exit")
    }

    fn input(query: &str) -> io::Result<String>
    {
        print!("{}", query);
        io::stdout().flush()?;

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;
        
        return Ok(buffer.to_string());
    }

    fn process_input(&mut self) -> bool
    {
        match Self::input("\nEnter command index: ")
        {
            Ok(command) =>
            {
                match command.trim()
                {
                    "h" => Self::print_menu(),
                    "1" => {
                        let name: String = Self::input("Enter name of new task: ").unwrap().trim().to_string();
                        let description: String = Self::input("Enter description: ").unwrap().trim().to_string();
                        let mut priority: Priority = Priority::None;
                        while priority == Priority::None
                        {
                            priority = match Self::input("Enter index of priority (1. Low, 2. Medium, 3. High, 4. Very High): ")
                                .unwrap()
                                .trim()
                            {
                                "1" => Priority::Low,
                                "2" => Priority::Medium,
                                "3" => Priority::High,
                                "4" => Priority::VeryHigh,
                                _   => Priority::None,
                            }
                        }
                        self.my_tasks.push(Task::new(name, description, priority));
                    },
                    "2" => {
                        match self.my_tasks.pop()
                        {
                            Some(task) => println!("Task \"{}\" removed", task.name),
                            None             => println!("List of tasks is empty"),
                        }
                    },
                    "3" => {
                        let name: String = Self::input("Enter name of task that you wanna remove: ").unwrap();
                        println!("Task \"{}\" removed", self.my_tasks.remove(name.trim()).unwrap().name);

                    },
                    "4" => {
                        let name: String = Self::input("Enter name of task that you wanna find: ").unwrap();
                        match self.my_tasks.find(name.trim())
                        {
                            Some(index) => self.my_tasks.tasks[index].print(),
                            None => println!("Task \"{}\" not found", name)
                        }
                    },
                    "5" => {
                        self.my_tasks.sort();
                        self.my_tasks.print();
                    },
                    "6" => {
                        self.my_tasks.clear();
                        println!("All tasks removed");
                    },
                    "7" => {
                        let path: String = Self::input("Enter path to file where to store tasks: ").unwrap();
                        self.my_tasks.store_to_file(path.trim()).expect("Error to store to file");
                    },
                    "8" => {
                        let path: String = Self::input("Enter path to file that store tasks: ").unwrap();
                        self.my_tasks.read_from_file(path.trim()).expect("Error to read from file");
                    }
                    "9" => return false,

                    _ => println!("Invalid input")
                }
            },
            Err(e) => println!("Error user input: {e}")
        };
        return true;
    }
}

fn main()
{
    let mut console: ConsoleForTask = ConsoleForTask::new();
    println!("Task Manager 1.0");
    ConsoleForTask::print_menu();

    while console.process_input() {}
}
