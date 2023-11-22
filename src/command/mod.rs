use anyhow::{anyhow, Result};

pub trait Command<Target> {
    fn execute(&mut self, target: &mut Target);
    fn undo(&mut self, target: &mut Target);
}

pub struct CommandManager<'a, Target: 'a> {
    queue_stack: Vec<Box<dyn Command<Target> + 'a>>,
    target: &'a mut Target,
    index: usize,
}

impl<'a, Target> CommandManager<'a, Target> {
    pub fn new(target: &'a mut Target) -> Self {
        CommandManager {
            queue_stack: Vec::new(),
            target,
            index: 0,
        }
    }
    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn execute(&mut self) -> Result<()> {
        if self.queue_stack.len() <= self.index {
            Err(anyhow!("Done"))
        } else {
            let cmd = &mut self.queue_stack[self.index];
            cmd.execute(&mut self.target);
            self.index += 1;
            Ok(())
        }
    }
    pub fn execute_all(&mut self) {
        loop {
            if let Err(_) = self.execute() {
                break;
            }
        }
    }
    pub fn undo(&mut self) -> Result<()> {
        if 0 == self.index {
            return Err(anyhow!("Couldn't go back"));
        } else {
            self.index -= 1;
            let cmd = &mut self.queue_stack[self.index];
            cmd.undo(&mut self.target);
            Ok(())
        }
    }
    pub fn append<Cmd: Command<Target> + 'a>(&mut self, command: Cmd) {
        self.queue_stack.push(Box::new(command));
    }
    pub fn clear(&mut self) {
        self.queue_stack.clear();
        self.index = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Debug, PartialEq)]
    struct Actor {
        name: &'static str,
        x: i64,
        y: i64,
        dx: i64,
        dy: i64,
    }
    impl Actor {
        pub fn new() -> Self {
            Actor {
                name: "",
                x: 0,
                y: 0,
                dx: 1,
                dy: 0,
            }
        }
        pub fn go_forward(&mut self) {
            self.x += self.dx;
            self.y += self.dy;
        }
        pub fn set_velocity(&mut self, v: (i64, i64)) {
            self.dx = v.0;
            self.dy = v.1;
        }
        pub fn get_velocity(&mut self) -> (i64, i64) {
            (self.dx, self.dy)
        }
    }

    //enumの中に構造体入れた方が動的ディスパッチより多分速い

    struct GoForward;
    impl Command<Actor> for GoForward {
        fn execute(&mut self, target: &mut Actor) {
            target.go_forward();
        }
        fn undo(&mut self, target: &mut Actor) {
            let mut cmd = TurnRight;
            cmd.execute(target);
            cmd.execute(target);
            self.execute(target);
            cmd.execute(target);
            cmd.execute(target);
        }
    }
    struct TurnRight;
    impl Command<Actor> for TurnRight {
        fn execute(&mut self, target: &mut Actor) {
            let (dx, dy) = target.get_velocity();
            target.set_velocity((dy, -dx));
        }
        fn undo(&mut self, target: &mut Actor) {
            let mut cmd = TurnLeft;
            cmd.execute(target);
        }
    }
    struct TurnLeft;
    impl Command<Actor> for TurnLeft {
        fn execute(&mut self, target: &mut Actor) {
            let (dx, dy) = target.get_velocity();
            target.set_velocity((-dy, dx));
        }
        fn undo(&mut self, target: &mut Actor) {
            let mut cmd = TurnRight;
            cmd.execute(target);
        }
    }
    struct Teleport {
        position: (i64, i64),
        prev: Option<(i64, i64)>,
    }
    impl Teleport {
        pub fn new(position: (i64, i64)) -> Self {
            Self {
                position,
                prev: None,
            }
        }
    }
    impl Command<Actor> for Teleport {
        fn execute(&mut self, target: &mut Actor) {
            self.prev = Some((target.x, target.y));
            target.x = self.position.0;
            target.y = self.position.1;
        }
        fn undo(&mut self, target: &mut Actor) {
            let pos = match self.prev {
                Some(v) => v,
                None => (0, 0),
            };
            let mut cmd = Teleport::new(pos);
            cmd.execute(target);
        }
    }
    struct Rename {
        name: &'static str,
        prev: Option<&'static str>,
    }
    impl Rename {
        pub fn new(name: &'static str) -> Self {
            Self { name, prev: None }
        }
    }
    impl Command<Actor> for Rename {
        fn execute(&mut self, target: &mut Actor) {
            self.prev = Some(target.name);
            target.name = self.name;
        }
        fn undo(&mut self, target: &mut Actor) {
            let name = match self.prev {
                Some(v) => v,
                None => "",
            };
            let mut cmd = Rename::new(name);
            cmd.execute(target);
        }
    }
    #[test]
    fn main() {
        let mut actor = Actor::new();
        let mut invoker = CommandManager::new(&mut actor);
        {
            invoker.append(Rename::new("Tom"));
            invoker.append(GoForward);
            invoker.append(Teleport::new((10, 10)));
            invoker.append(TurnRight);
            invoker.append(GoForward);
        }
        invoker.execute().unwrap();
        assert_eq!(
            *invoker.target,
            Actor {
                name: "Tom",
                x: 0,
                y: 0,
                dx: 1,
                dy: 0
            }
        );
        invoker.execute_all();
        assert_eq!(
            *invoker.target,
            Actor {
                name: "Tom",
                x: 10,
                y: 9,
                dx: 0,
                dy: -1
            }
        );
        invoker.undo().unwrap();
        assert_eq!(
            *invoker.target,
            Actor {
                name: "Tom",
                x: 10,
                y: 10,
                dx: 0,
                dy: -1
            }
        );
    }
}
