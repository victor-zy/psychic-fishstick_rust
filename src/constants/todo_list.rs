struct TodoItem {
    name: String,
    completed: char,
}

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {

    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String) {
        let item: TodoItem = TodoItem {
            name,
            completed: ' ',
        };
        self.items.push(item);
    }

    pub fn print(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{}: [{}] - {}", index + 1, item.completed, item.name);
        }
    }

    pub fn mark_done(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.completed = 'x';
        }
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }
}