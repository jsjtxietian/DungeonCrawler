#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action : VisitorAction,
    age : i8,
}

#[derive(Debug)]
enum VisitorAction{
    Accept,
    AcceptWithNote {note:String},
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action:VisitorAction, age:i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self){
        match &self.action {
            VisitorAction::Accept => println!("Welcome!"),
            VisitorAction::AcceptWithNote{note} => println!("Welcome {}!",note),
            _ => println!("FUCK OFF"),
        }
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept,45),
        Visitor::new("damn", VisitorAction::AcceptWithNote{note:String::from("ddd")}, 15)
    ];
    // for i in &visitor_list {
    //     if i.name == name {
    //         i.greet_visitor();
    //     } else {
    //         println!("Wow");
    //     }
    // }

    // println!("{:?}",visitor_list[0]);

    loop {
        println!("Whats your name");
        let name = get_name();

        let known_visitor = visitor_list.iter().find(|vistor| vistor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    visitor_list.push(Visitor::new(&name, VisitorAction::Accept,88));
                }
            }
        }
    }
}
