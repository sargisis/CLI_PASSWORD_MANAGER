use crate::models::Entry;
use crate::commands::Commands;

pub struct DataBase {
    entries: Vec<Entry>,
}


impl DataBase {
    pub fn new() -> Self {
        Self { entries: Vec::new()}
    }

    pub fn execute(&mut self , cmd: Commands) {
        match cmd {
            Commands::Add { service, login, password } => {
                self.entries.push( Entry { service, login, password } );
                println!("Added Success ✅");
            }
            Commands::Get { service } => {
                for e in &self.entries {
                    if e.service == service {
                        println!("{} | {} | {}" , e.service , e.login , e.password)
                    }
                }
            }
            Commands::List => {
                for e in &self.entries {
                    println!("{} | {} | {}" , e.service , e.login , e.password)
                }
            }

            Commands::Delete { service } => {
                self.entries.retain(|e| e.service != service);
                println!("Deleted");
            }
        }
    }
}