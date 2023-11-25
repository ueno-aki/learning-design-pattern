use anyhow::{anyhow, Result};

pub struct Switch<'a, Cmd, Receiver> {
    queue_stack: Vec<Cmd>,
    receiver: &'a mut Receiver,
    current_index: usize,
}
impl<'a, Cmd, Receiver> Switch<'a, Cmd, Receiver> {
    pub fn new(receiver: &'a mut Receiver) -> Self {
        Switch {
            queue_stack: Vec::new(),
            receiver,
            current_index: 0,
        }
    }
    pub fn append(&mut self, command: Cmd) {
        self.queue_stack.push(command);
    }
    pub fn clear(&mut self) {
        self.queue_stack.clear();
        self.current_index = 0;
    }
}
impl<'a, Cmd, Receiver> Switch<'a, Cmd, Receiver>
where
    Cmd: Command<Receiver>,
{
    pub fn execute(&mut self) -> Result<()> {
        if self.queue_stack.len() <= self.current_index {
            Err(anyhow!("Done"))
        } else {
            let cmd = &mut self.queue_stack[self.current_index];
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
}

pub trait Command<R> {
    fn execute(&mut self, receiver: &mut R);
}
pub enum SwitchCommand {
    TurnOn,
    TurnOff,
}
impl Command<Light> for SwitchCommand {
    fn execute(&mut self, receiver: &mut Light) {
        use SwitchCommand::*;
        match self {
            TurnOn => receiver.turn_on(),
            TurnOff => receiver.turn_off(),
        }
    }
}

pub struct Light;
impl Light {
    pub fn turn_on(&mut self) {
        println!("Turn ON");
    }
    pub fn turn_off(&mut self) {
        println!("Turn OFF");
    }
}

#[test]
fn flashing() {
    let mut my_light = Light;
    let mut my_switch = Switch::new(&mut my_light);
    {
        use SwitchCommand::*;
        my_switch.append(TurnOn);
        my_switch.append(TurnOff);
        my_switch.append(TurnOn);
        my_switch.append(TurnOff);
        my_switch.append(TurnOn);
        my_switch.append(TurnOff);
    }
    my_switch.execute_all();
}
