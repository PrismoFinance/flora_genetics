use std::env;

pub struct Cli {
    pub search_type: String,
    pub query: String,
}

impl Cli {
    pub fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("Usage: <search_type> <query>\nSearch types: 'genus' or 'author'");
        }
        Ok(Self {
            search_type: args[1].clone(),
            query: args[2].clone(),
        })
    }
}