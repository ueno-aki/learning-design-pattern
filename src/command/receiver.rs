#![allow(unused)]
use anyhow::{anyhow, Result};

pub struct Controller<'a, Cmd, Receiver> {
    command_queue: Vec<Cmd>,
    receiver: &'a mut Receiver,
    current_index: usize,
}
impl<'a, Cmd, Receiver> Controller<'a, Cmd, Receiver> {
    pub fn new(receiver: &'a mut Receiver) -> Self {
        Controller {
            command_queue: Vec::new(),
            receiver,
            current_index: 0,
        }
    }
    pub fn append(&mut self, command: Cmd) {
        self.command_queue.push(command);
    }
    pub fn clear(&mut self) {
        self.command_queue.clear();
        self.current_index = 0;
    }
}
impl<'a, Cmd, Receiver> Controller<'a, Cmd, Receiver>
where
    Cmd: Command<Receiver>,
{
    pub fn execute(&mut self) -> Result<()> {
        if self.command_queue.len() <= self.current_index {
            Err(anyhow!("Done"))
        } else {
            let cmd = &mut self.command_queue[self.current_index];
            cmd.execute(&mut self.receiver);
            self.current_index += 1;
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
        if 0 == self.current_index {
            return Err(anyhow!("Couldn't go back"));
        } else {
            self.current_index -= 1;
            let cmd = &mut self.command_queue[self.current_index];
            cmd.undo(&mut self.receiver);
            Ok(())
        }
    }
}

pub trait Command<Receiver> {
    fn execute(&mut self, receiver: &mut Receiver);
    fn undo(&mut self, receiver: &mut Receiver);
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}
impl Robot {
    pub fn new() -> Self {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }
    pub fn go_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
    pub fn turn_right(&mut self) {
        let (dx, dy) = (self.dx, self.dy);
        self.dx = dy;
        self.dy = -dx;
    }
    pub fn turn_left(&mut self) {
        let (dx, dy) = (self.dx, self.dy);
        self.dx = -dy;
        self.dy = dx;
    }
}

pub enum ControllerCommand {
    GoForward,
    TurnRight,
    TurnLeft,
}
impl Command<Robot> for ControllerCommand {
    fn execute(&mut self, receiver: &mut Robot) {
        use ControllerCommand::*;
        match self {
            GoForward => receiver.go_forward(),
            TurnRight => receiver.turn_right(),
            TurnLeft => receiver.turn_left(),
        }
    }
    fn undo(&mut self, receiver: &mut Robot) {
        use ControllerCommand::*;
        match self {
            GoForward => {
                receiver.turn_right();
                receiver.turn_right();
                receiver.go_forward();
                receiver.turn_right();
                receiver.turn_right();
            }
            TurnRight => receiver.turn_left(),
            TurnLeft => receiver.turn_right(),
        }
    }
}

#[test]
fn test() {
    let mut my_robot = Robot::new();
    let mut my_controller = Controller::new(&mut my_robot);
    assert_eq!(
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1
        },
        *my_controller.receiver
    );
    {
        use ControllerCommand::*;
        my_controller.append(TurnRight);
        my_controller.append(GoForward);
        my_controller.append(TurnLeft);
        my_controller.append(GoForward);
        my_controller.append(GoForward);
    }
    my_controller.execute_all();
    assert_eq!(
        Robot {
            x: 1,
            y: 2,
            dx: 0,
            dy: 1
        },
        *my_controller.receiver
    );
    my_controller.undo().unwrap();
    my_controller.undo().unwrap();
    assert_eq!(
        Robot {
            x: 1,
            y: 0,
            dx: 0,
            dy: 1
        },
        *my_controller.receiver
    )
}
