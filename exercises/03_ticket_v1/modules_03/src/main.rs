mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.
  use crate::Ticket;//for absolute paths.
//   use super::*; //for relative paths to refer to items.

   pub fn  create_todo_ticket(title: String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".into())
    }
}

pub struct Ticket {
   pub title: String,
   pub description: String,
   pub status: String,
}

impl Ticket {
   pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_todo_ticket() {
        let ticket = helpers::create_todo_ticket("Test".into(), "Desc".into());
        assert_eq!(ticket.status, "To-Do");
    }
}