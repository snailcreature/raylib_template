//! Implementation of the Command design pattern

/// A command to issue.
pub trait Command<T = ()> {
    /// Action to perform.
    fn execute(_target: T) -> ();
}

/// A command that can be undone.
pub trait CommandUndo {
    /// Action to perform.
    fn execute(&mut self) -> ();

    /// Undo the action that was done.
    fn undo(&mut self) -> ();
}

/// Stack of executed commands that can be undone and redone.
pub struct CommandStack {
    /// Commands that have been executed.
    commands: Vec<Box<dyn CommandUndo>>,
    /// Current index of the command to redo.
    head: usize,
    /// Maximum number of commands to store.
    history_size: usize,
}

impl CommandStack {
    pub fn new(history_size: Option<usize>) -> Self {
        Self {
            commands: Vec::new(),
            head: 0,
            history_size: history_size.unwrap_or(50),
        }
    }

    /// Do a command and add it to the stack.
    /// If there are commands in the stack that can be redone, clear them first.
    pub fn execute<T: CommandUndo + 'static>(&mut self, mut command: T) -> bool {
        if self.head < self.commands.len() {
            self.commands.truncate(self.head - 1);
        }

        if self.commands.len() == self.history_size {
            self.commands.remove(0);
        }

        command.execute();

        self.commands.push(Box::new(command));
        self.head += 1;

        true
    }

    /// Undo the last command that was executed or redone.
    pub fn undo(&mut self) -> bool {
        if self.head == 0 {
            return false;
        }

        self.commands[self.head - 1].undo();

        self.head -= 1;

        true
    }

    /// Redo the last command that was undone.
    pub fn redo(&mut self) -> bool {
        if self.head == self.commands.len() {
            return false;
        }

        self.commands[self.head].execute();

        self.head += 1;

        true
    }
}
