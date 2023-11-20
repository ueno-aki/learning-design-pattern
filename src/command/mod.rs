use anyhow::{Result, anyhow};

pub trait Command<Target> {
    fn execute(&self,target: &mut Target);
}

///Invoker
pub struct MacroCommand<'a,Cmd,Target> {
    commands: Vec<Cmd>,
    target:&'a mut Target,
    index: usize
}

impl<'a,Cmd,Target> MacroCommand<'a,Cmd,Target> {
    pub fn new(target:&'a mut Target) -> Self {
        MacroCommand { commands: Vec::new(), target, index: 0 }
    }
    pub fn target(&self) -> &Target {
        &self.target
    }
    pub fn append(&mut self,command:Cmd) {
        self.commands.push(command);
    }
}

impl<'a,Cmd,Target> MacroCommand<'a,Cmd,Target> 
where
    Cmd: Command<Target>
{
    pub fn execute(&mut self) -> Result<()>{
        if self.commands.len() <= self.index {
            Err(anyhow!("Commands Stack was done"))
        }
        else {
            let command = &self.commands[self.index];
            let target = &mut self.target;
            command.execute(target);
            self.index += 1;
            Ok(())
        }

    }
    pub fn execute_all(&mut self) {
        for _ in self.index .. self.commands.len() {
        self.execute().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug,PartialEq)]
    struct Printer {
        document:String,
        amount:usize
    }

    enum PrinterCommand {
        PrintJob(String,usize),
    }
    impl Command<Printer> for PrinterCommand {
        fn execute(&self,target: &mut Printer) {
            use PrinterCommand::*;
            match self {
                PrintJob(doc,size) => {
                    target.document = doc.clone();
                    target.amount = *size;
                    println!("{} documents[{}] was printed ",doc,size)
                }
            }
        }
    }

    #[test]
    fn main() {
        let mut printer = Printer {
            document:"".to_owned(),
            amount:30
        };
        let mut invoker = MacroCommand::new(&mut printer);
        {
            use PrinterCommand::*;
            invoker.append(PrintJob("Glory to Arstotzka".to_owned(),30));
            invoker.append(PrintJob("GOOD-JOB".to_owned(),1));
        }
        invoker.execute().unwrap();
        assert_eq!(*invoker.target(),Printer {
            document:"Glory to Arstotzka".to_owned(),
            amount:30
        });
        invoker.execute().unwrap();
        assert_eq!(*invoker.target(),Printer {
            document:"GOOD-JOB".to_owned(),
            amount:1
        });
    }
}