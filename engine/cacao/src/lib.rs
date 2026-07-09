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
    use std::thread;

    use serde::{Deserialize, Serialize};

    use crate::{
        command::{Command, CommandStack, CommandUndo},
        event::{Event, EventBroker, Module, ModuleCtx, Payload, ron_deserialise},
        observe::{Observer, Subject},
    };

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

    #[test]
    fn observe_test() {
        #[derive(Debug, Clone, Copy)]
        struct TestSubject(i32);

        struct TestObserver {}

        impl TestObserver {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl Observer<TestSubject> for TestObserver {
            fn update(&self, state: &TestSubject) {
                assert_eq!(state.0, 50);
            }
        }

        let mut test_subject = Subject::new(TestSubject(30));

        let o0 = TestObserver::new();
        let o1 = TestObserver::new();
        let o2 = TestObserver::new();
        let o3 = TestObserver::new();

        test_subject.attach(Box::new(o0));
        test_subject.attach(Box::new(o1));
        test_subject.attach(Box::new(o2));
        test_subject.attach(Box::new(o3));

        test_subject.update_state(TestSubject(50));
    }

    #[test]
    fn event_test() {
        #[derive(Serialize, Deserialize)]
        struct TestEvent(i32);

        struct TestHandler {
            ctx: ModuleCtx,
        }

        impl Module for TestHandler {
            type Response = ();
            fn new(ctx: crate::prelude::ModuleCtx) -> Self {
                Self { ctx }
            }

            fn run(&mut self) -> std::thread::JoinHandle<()> {
                let rx = self.ctx.receiver.clone();
                thread::spawn(move || {
                    loop {
                        let Ok(event) = rx.recv() else {
                            continue;
                        };

                        let Payload::Post(data) = event.payload else {
                            continue;
                        };
                        let event: TestEvent = ron_deserialise(&data);

                        assert_eq!(event.0, 50);

                        break;
                    }
                })
            }
        }

        let mut broker = EventBroker::new();

        let mut test_handler0 = TestHandler::new(ModuleCtx::new("test", &mut broker));

        broker.init();

        let mut handles: Vec<_> = Vec::new();
        handles.push(test_handler0.run());

        broker.publish(Event::new(
            "test".to_string(),
            Payload::new_post(TestEvent(50)),
        ));

        for handle in handles {
            handle.join().unwrap()
        }
    }
}
