use anyhow::{anyhow, Result};

pub trait Command<Target> {
    fn execute(&self, target: &mut Target);
    fn undo(&self,target:&mut Target);
}

pub struct CommandManager<'a, Cmd, Target>
where
    Cmd: Command<Target>,
{
    queue_stack: Vec<Cmd>,
    target: &'a mut Target,
    index: usize,
}

impl<'a, Cmd, Target> CommandManager<'a, Cmd, Target>
where
    Cmd: Command<Target>,
{
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
            let command = &self.queue_stack[self.index];
            let target = &mut self.target;
            self.index += 1;
            command.execute(target);
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
            return Err(anyhow!("Cannot go back"))
        } else {
            self.index -= 1;
            let c = &self.queue_stack[self.index];
            let t = &mut *self.target;
            c.undo(t);
            Ok(())
        }
    }
    pub fn append(&mut self, command: Cmd) {
        self.queue_stack.push(command);
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
        x:i64,
        y:i64,
        dx:i64,
        dy:i64
    }
    impl Actor {
        pub fn new() -> Self{
            Actor {x: 0, y: 0, dx: 1, dy: 0 }
        }
        pub fn go_forward(&mut self) {
            self.x += self.dx;
            self.y += self.dy;
        }
        pub fn set_velocity(&mut self,v:(i64,i64)) {
            self.dx = v.0;
            self.dy = v.1;
        }
        pub fn get_velocity(&mut self) -> (i64,i64){
            (self.dx,self.dy)
        }
    }
    enum ActorCommand {
        GoForward,
        TurnRight,
        TurnLeft,
    }
    impl Command<Actor> for ActorCommand {
        fn execute(&self, target: &mut Actor) {
            use ActorCommand::*;
            match self {
                GoForward => {
                    target.go_forward();
                }
                TurnRight => {
                    let (dx,dy) = target.get_velocity();
                    target.set_velocity((dy,-dx));
                },
                TurnLeft => {
                    let (dx,dy) = target.get_velocity();
                    target.set_velocity((-dy,dx));
                }
            }
        }
        fn undo(&self,target:&mut Actor) {
            use ActorCommand::*;
            match self {
                GoForward => {
                    let cmd = TurnRight;
                    cmd.execute(target);
                    cmd.execute(target);
                    self.execute(target);
                    cmd.execute(target);
                    cmd.execute(target);
                }
                TurnRight => {
                    let cmd = TurnLeft;
                    cmd.execute(target);
                },
                TurnLeft => {
                    let cmd = TurnRight;
                    cmd.execute(target);
                }
            }
        }
    }
    #[test]
    fn main() {
        let mut actor = Actor::new();
        let mut invoker = CommandManager::new(&mut actor);
        {
            use ActorCommand::*;
            invoker.append(GoForward);
            invoker.append(TurnRight);
            invoker.append(GoForward);
        }
        invoker.execute().unwrap();
        assert_eq!(
            *invoker.target,
            Actor {
                x:1,
                y:0,
                dx:1,
                dy:0
            }
        );
        invoker.execute_all();
        assert_eq!(
            *invoker.target,
            Actor {
                x:1,
                y:-1,
                dx:0,
                dy:-1
            }
        );
        invoker.undo().unwrap();
        assert_eq!(
            *invoker.target,
            Actor {
                x:1,
                y:0,
                dx:0,
                dy:-1
            }
        );
    }
}
