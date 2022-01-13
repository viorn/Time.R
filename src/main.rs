
mod screenshot;
mod task_manager;

use crate::{task_manager::{Task, TaskManager}};

fn main() {
    println!("Hello, world!");
    let task_manager = TaskManager::new();
    task_manager.set_current_task(Task{name: String::from("sda"), description: String::from("sda2112")});
    screenshot::take("image.png");
}
