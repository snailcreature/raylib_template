pub mod command;
pub mod event;
pub mod observe;

pub mod prelude {
    pub use crate::command::*;
    pub use crate::event::*;
    pub use crate::observe::*;
}

#[cfg(test)]
mod tests {
    use crate::command::{Command, CommandStack, CommandUndo};

    struct Test {
        pub value: String,
    }

    impl Test {
        pub fn new(value: &str) -> Self {
            Self {
                value: value.to_string(),
            }
        }
    }

    impl Command for Test {
        fn execute(&mut self, _target: ()) -> () {
            self.value = "Goodbye".to_string();
        }
    }

    struct Test2 {
        pub value: String,
        last_value: String,
    }

    impl Test2 {
        pub fn new(value: &str) -> Self {
            Self {
                value: value.to_string(),
                last_value: "".to_string(),
            }
        }
    }

    impl CommandUndo for Test2 {
        fn execute(&mut self) -> () {
            self.last_value = self.value.clone();
            self.value = "Goodbye".to_string();
        }

        fn undo(&mut self) -> () {
            self.value = self.last_value.clone();
        }
    }

    #[test]
    fn command_test() {
        let mut test0 = Test::new("Hello!");

        test0.execute(());

        assert_eq!(test0.value, "Goodbye".to_string())
    }

    #[test]
    fn command_undo_test() {
        let mut cmd_stack = CommandStack::new(None);

        let test1 = Test2::new("Hello");

        cmd_stack.execute(test1);

        cmd_stack.undo();

        cmd_stack.redo();
    }
}
