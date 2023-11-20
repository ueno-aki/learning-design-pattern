use anyhow::{Result, anyhow};

pub trait Command<Target> {
    fn execute(&self,target: &mut Target);
}

pub struct Invoker<'a,Cmd,Target> 
where
    Cmd: Command<Target>
{
    commands: Vec<Cmd>,
    target:&'a mut Target,
    index: usize
}

impl<'a,Cmd,Target> Invoker<'a,Cmd,Target> 
where
    Cmd: Command<Target>
{
    pub fn new(target:&'a mut Target) -> Self {
        Invoker { commands: Vec::new(), target, index: 0 }
    }
    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn execute(&mut self) -> Result<()>{
        if self.commands.len() <= self.index {
            Err(anyhow!("Commands Stack was done."))
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
    pub fn append(&mut self,command:Cmd) {
        self.commands.push(command);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug,PartialEq)]
    struct Document {
        text:String,
        amount:usize
    }

    enum PrinterCommand {
        PrintJob(String,usize),
        Alert(String)
    }
    impl Command<Document> for PrinterCommand {
        fn execute(&self,target: &mut Document) {
            use PrinterCommand::*;
            match self {
                PrintJob(doc,size) => {
                    target.text = doc.clone();
                    target.amount = *size;
                    println!("{} documents[{}] was printed ",doc,size)
                }
                Alert(str) => {
                    println!("[Alert]'{str}'")
                }
            }
        }
    }

    #[test]
    fn main() {
        let mut docment = Document {
            text:"".to_owned(),
            amount:30
        };
        let mut printer = Invoker::new(&mut docment);
        {
            use PrinterCommand::*;
            printer.append(PrintJob("Glory to Arstotzka".to_owned(),30));
            printer.append(Alert("Confirmication".to_owned()));
            printer.append(PrintJob("GOOD-JOB".to_owned(),1));
        }
        printer.execute().unwrap();
        assert_eq!(*printer.target(),Document {
            text:"Glory to Arstotzka".to_owned(),
            amount:30
        });
        printer.execute().unwrap();
        printer.execute().unwrap();
        assert_eq!(*printer.target(),Document {
            text:"GOOD-JOB".to_owned(),
            amount:1
        });
    }
}