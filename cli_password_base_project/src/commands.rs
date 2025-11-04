use crate::models::Entry;


pub enum Commands {
    Add {service: String , login: String , password: String},
    Get {service: String},
    List , 
    Delete {service: String},
}

impl Commands {
    pub fn parse_command(args: Vec<String>) -> Result<Commands , &'static str> {
        match args[0].as_str() {
            "add" => {
                if args.len() < 4 {
                    return  Err("usage: add <service> <login> <password>")
                }
                Ok(Commands::Add { 
                    service: args[1].clone(), 
                    login: args[2].clone(), 
                    password: args[3].clone()
                })
            }
            "get" => {
                if args.len() < 2 {
                    return  Err("usage: get <service>");
                }
                Ok(Commands::Get { 
                    service: args[1].clone() 
                })
            }
            "list" => {
                Ok(Commands::List)
            }
            "delete" => {
                if args.len() < 2 {
                    return  Err("usage: delete <service>")
                }
                Ok(Commands::Delete { 
                    service: args[1].clone() 
                })
            }
            _=> Err("unkown command")
        }
    }
}