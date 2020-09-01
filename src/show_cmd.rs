use todolst::components::{ group::*, list::*, item::*, todolst::* };
use clap::{ App, Arg, SubCommand, ArgMatches };
use futures::executor::block_on;
use std::sync::{ Weak, Mutex };
use crate::utils::{ load_todolst, save_todolst };

pub fn show_command(sub_m: &ArgMatches) {
    match sub_m.subcommand() {
        ("group", Some(sub_m)) => { show_group_command(sub_m) },
        ("list", Some(sub_m)) => { show_list_command(sub_m) },
        ("item", Some(sub_m)) => { show_item_command(sub_m) },
        _ => (),
    }

    return;

    let system = load_todolst();
    if sub_m.is_present("lists") { // show lists
        for list in system.iter_lists() {
            let to_be_printed = 
            if let Some(value) = sub_m.value_of("group")
            {
                let list = list.upgrade().unwrap();
                let list = list.lock().unwrap();
                let group = list.group();
                match group {
                    None => { false },
                    Some(group) => {
                        group.upgrade().unwrap().lock().unwrap().title() == value
                    }
                }
            } else { true };
            
            if to_be_printed {
                println!("{}", list.upgrade().unwrap().lock().unwrap().title());
            }
        }

    }
}

fn show_group_command(sub_m: &ArgMatches) {
    let system = load_todolst();
    if sub_m.is_present("GROUP") {

    } else {
        let mut print_options = GroupPrintOptions {
            title: false, parent: false
        };
        if sub_m.is_present("title") { print_options.title = true; }
        if sub_m.is_present("parent") { print_options.parent = true; }
        if print_options.is_all_false() { print_options.title = true; }
        let groups = if sub_m.is_present("with-parent") { 
            system.iter_groups().filter(|i| {
                let i = i.upgrade().unwrap();
                let i = i.lock().unwrap();
                let parent = i.parent();
                match parent {
                    None => false,
                    Some(parent) => {
                        let parent = parent.upgrade().unwrap();
                        let parent = parent.lock().unwrap();
                        parent.title() == sub_m.value_of("with-parent").expect("with-parent should not empty.")
                    }
                }
            }).collect()
        } else {
            system.iter_groups().collect()
        };
        print_groups(groups, print_options);
        
    }

}

fn print_groups(groups: Vec<Weak<Mutex<Group>>>, print_options: GroupPrintOptions) {
    let mut headers = vec![];
    if print_options.title { headers.push("title"); }
    if print_options.parent { headers.push("parent"); }
    
    println!("====");
    println!("{}", headers.join("\t"));
    println!("----");

    for group in groups.iter() {
        let group = group.upgrade().unwrap();
        let group = group.lock().unwrap();
        let mut details = vec![];
        if print_options.title { details.push(group.title().to_string()); }
        if print_options.parent { 
            let parent = group.parent();
            match parent {
                None => { details.push("[null]".to_string()); }
                Some(parent) => {
                    let parent = parent.upgrade().unwrap();
                    let parent = parent.lock().unwrap();
                    details.push(parent.title().to_string());
                }
            }
        }
        println!("{}", details.join("\t"));
    }

    println!("====");

}

pub struct GroupPrintOptions {
    title: bool,
    parent: bool,
}

impl GroupPrintOptions {
    pub fn is_all_false(&self) -> bool {
        !(self.title || self.parent)
    }
}

fn show_list_command(sub_m: &ArgMatches) {
    let system = load_todolst();
    if sub_m.is_present("LIST") {

    } else {
        let mut print_options = ListPrintOptions {
            title: false, group: false
        };
        if sub_m.is_present("title") { print_options.title = true; }
        if sub_m.is_present("group") { print_options.group = true; }
        if print_options.is_all_false() { print_options.title = true; }
        let lists = if sub_m.is_present("with-group") { 
            system.iter_lists().filter(|i| {
                let i = i.upgrade().unwrap();
                let i = i.lock().unwrap();
                let group = i.group();
                match group {
                    None => false,
                    Some(group) => {
                        let group = group.upgrade().unwrap();
                        let group = group.lock().unwrap();
                        group.title() == sub_m.value_of("with-group").expect("with-group should not empty.")
                    }
                }
            }).collect()
        } else {
            system.iter_lists().collect()
        };
        print_lists(lists, print_options);
        
    }

}

fn print_lists(lists: Vec<Weak<Mutex<List>>>, print_options: ListPrintOptions) {
    let mut headers = vec![];
    if print_options.title { headers.push("title"); }
    if print_options.group { headers.push("group"); }
    
    println!("====");
    println!("{}", headers.join("\t"));
    println!("----");

    for list in lists.iter() {
        let list = list.upgrade().unwrap();
        let list = list.lock().unwrap();
        let mut details = vec![];
        if print_options.title { details.push(list.title().to_string()); }
        if print_options.group { 
            let group = list.group();
            match group {
                None => { details.push("[null]".to_string()); }
                Some(group) => {
                    let group = group.upgrade().unwrap();
                    let group = group.lock().unwrap();
                    details.push(group.title().to_string());
                }
            }
        }
        println!("{}", details.join("\t"));
    }

    println!("====");

}

pub struct ListPrintOptions {
    title: bool,
    group: bool,
}

impl ListPrintOptions {
    pub fn is_all_false(&self) -> bool {
        !(self.title || self.group)
    }
}

fn show_item_command(sub_m: &ArgMatches) {

}