use todolst::components::{ group::*, list::*, item::* };
use clap::{ ArgMatches };
use std::sync::{ Weak, Mutex };
use crate::utils::{ load_todolst };

pub fn show_command(sub_m: &ArgMatches) {
    match sub_m.subcommand() {
        ("group", Some(sub_m)) => { show_group_command(sub_m) },
        ("list", Some(sub_m)) => { show_list_command(sub_m) },
        ("item", Some(sub_m)) => { show_item_command(sub_m) },
        _ => (),
    }

    return;
}

fn show_group_command(sub_m: &ArgMatches) {
    let system = load_todolst();

    // construct print_options
    let mut print_options = if sub_m.is_present("all") {
        GroupPrintOptions::new(true)
    } else {
        GroupPrintOptions {
            title : sub_m.is_present("title"),
            parent : sub_m.is_present("parent"),
        }
    };

    if sub_m.is_present("GROUP") {
        if print_options.is_all_false() { print_options.set_all(true) }

        let group_expected = sub_m.values_of("GROUP").expect("GROUP title should not be empty.").collect::<Vec<&str>>();

        let groups = system.iter_groups().filter(|i| {
            let i = i.upgrade().unwrap();
            let i = i.lock().unwrap();
            let title = i.title();
            group_expected.contains(&title)
        }).collect();

        print_groups(groups, print_options);

    } else {
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
                        parent.title() == sub_m.value_of("with-parent").expect("with-parent should not be empty.")
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

#[derive(Copy, Clone)]
pub struct GroupPrintOptions {
    title: bool,
    parent: bool,
}

impl GroupPrintOptions {
    pub fn new(init_value: bool) -> Self {
        Self {
            title: init_value,
            parent: init_value,
        }
    }
    pub fn is_all_false(&self) -> bool {
        !(self.title || self.parent)
    }
    pub fn set_all(&mut self, value: bool) {
        self.title = value;
        self.parent = value;
    }
}

fn show_list_command(sub_m: &ArgMatches) {
    let system = load_todolst();

    // construct print_options
    let mut print_options = if sub_m.is_present("all") {
        ListPrintOptions::new(true)
    } else {
        ListPrintOptions {
            title : sub_m.is_present("title"),
            group : sub_m.is_present("group"),
        }
    };
    
    if sub_m.is_present("LIST") {
        if print_options.is_all_false() { print_options.set_all(true); }

        let list_expected = sub_m.values_of("LIST").expect("LIST title should not be empty.").collect::<Vec<&str>>();
        let lists = system.iter_lists().filter(|i| {
            let i = i.upgrade().unwrap();
            let i = i.lock().unwrap();
            let title = i.title();
            list_expected.contains(&title)
        }).collect();

        print_lists(lists, print_options);

    } else {
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
                        group.title() == sub_m.value_of("with-group").expect("with-group should not be empty.")
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

#[derive(Copy, Clone)]
pub struct ListPrintOptions {
    title: bool,
    group: bool,
}

impl ListPrintOptions {
    pub fn new(init_value: bool) -> Self {
        Self {
            title: init_value,
            group: init_value,
        }
    }
    pub fn is_all_false(&self) -> bool {
        !(self.title || self.group)
    }
    pub fn set_all(&mut self, value: bool) {
        self.title = value;
        self.group = value;
    }
}

fn show_item_command(sub_m: &ArgMatches) {
    let system = load_todolst();

    // construct print_options
    let mut print_options = if sub_m.is_present("all") {
        ItemPrintOptions::new(true)
    } else {
        ItemPrintOptions {
            id: sub_m.is_present("id"),
            message: sub_m.is_present("message"),
            level: sub_m.is_present("level"),
            style: sub_m.is_present("style"),
            today: sub_m.is_present("today"),
            notice: sub_m.is_present("notice"),
            deadline: sub_m.is_present("deadline"),
            plan: sub_m.is_present("plan"),
            repeat: sub_m.is_present("repeat"),
            list: sub_m.is_present("list"),
            finished: sub_m.is_present("finished"),
            note: sub_m.is_present("note"),
        }
    };
    
    if sub_m.is_present("ITEM") {
        if print_options.is_all_false() { print_options.set_all(true); }

        let item_expected = sub_m.values_of("ITEM").expect("ITEM title should not be empty.").map(|i|{
            i.parse::<u32>().expect("ITEM should be a number.")
        }).collect::<Vec<u32>>();
        let items = system.iter_items().filter(|i| {
            let i = i.upgrade().unwrap();
            let i = i.lock().unwrap();
            let id = i.id();
            item_expected.contains(&id)
        }).collect();

        print_items(items, print_options);

    } else {
        if print_options.is_all_false() { 
            print_options.id = true;
            print_options.message = true; 
        }

        let mut items = system.iter_items().collect::<Vec<Weak<Mutex<Item>>>>();
        if sub_m.is_present("with-list") { 
            items = items.into_iter().filter(|i| {
                let i = i.upgrade().unwrap();
                let i = i.lock().unwrap();
                let list = i.list();
                match list {
                    None => false,
                    Some(list) => {
                        let list = list.upgrade().unwrap();
                        let list = list.lock().unwrap();
                        list.title() == sub_m.value_of("with-list").expect("with-list should not be empty.")
                    }
                }
            }).collect()
        };
        if sub_m.is_present("with-group") { 
            items = items.into_iter().filter(|i| {
                let i = i.upgrade().unwrap();
                let i = i.lock().unwrap();
                let list = i.list();
                match list {
                    None => false,
                    Some(list) => {
                        let list = list.upgrade().unwrap();
                        let list = list.lock().unwrap();
                        let group = list.group();
                        match group {
                            None => false,
                            Some(group) => {
                                let group = group.upgrade().unwrap();
                                let group = group.lock().unwrap();
                                group.title() == sub_m.value_of("with-group").expect("with-group should not be empty.")
                            }
                        }
                    }
                }
            }).collect()
        };
        print_items(items, print_options);
        
    }
}

fn print_items(items: Vec<Weak<Mutex<Item>>>, print_options: ItemPrintOptions) {
    let mut headers = vec![];
    if print_options.id { headers.push("id"); }
    if print_options.message { headers.push("message"); }
    if print_options.level { headers.push("level"); }
    if print_options.style { headers.push("style"); }
    if print_options.today { headers.push("today"); }
    if print_options.notice { headers.push("notice"); }
    if print_options.deadline { headers.push("deadline"); }
    if print_options.plan { headers.push("plan"); }
    if print_options.repeat { headers.push("repeat"); }
    if print_options.list { headers.push("list"); }
    if print_options.finished { headers.push("finished"); }
    if print_options.note { headers.push("note"); }
    
    println!("====");
    println!("{}", headers.join("\t"));
    println!("----");

    for item in items.iter() {
        let item = item.upgrade().unwrap();
        let item = item.lock().unwrap();
        let mut details = vec![];
        if print_options.id { details.push(item.id().to_string()); }
        if print_options.message { details.push(item.message().to_string()); }
        if print_options.level { details.push(item.level().to_string()); }
        if print_options.style { details.push(item.style().to_string()); }
        if print_options.today { details.push(item.today().to_string()); }
        if print_options.notice { details.push(match item.notice() { None => "[null]".to_string(), Some(t) => t.format("%Y-%m-%dT%H:%M:%S").to_string() } ); }
        if print_options.deadline { details.push(match item.deadline() { None => "[null]".to_string(), Some(t) => t.format("%Y-%m-%d").to_string() } ); }
        if print_options.plan { details.push(match item.plan() { None => "[null]".to_string(), Some(t) => t.format("%Y-%m-%d").to_string() } ); }
        if print_options.repeat { details.push(item.repeat().to_string_or("[null]".to_string())); }
        if print_options.list { 
            let list = item.list();
            match list {
                None => { details.push("[null]".to_string()); }
                Some(list) => {
                    let list = list.upgrade().unwrap();
                    let list = list.lock().unwrap();
                    details.push(list.title().to_string());
                }
            }
        }
        if print_options.finished { details.push(item.finished().to_string()); }
        if print_options.note { details.push(item.note().to_string()); }
        println!("{}", details.join("\t"));
    }

    println!("====");

}

#[derive(Copy, Clone)]
pub struct ItemPrintOptions {
    id: bool,
    message: bool,
    level: bool,
    style: bool,
    today: bool,
    notice: bool,
    deadline: bool,
    plan: bool,
    repeat: bool,
    list: bool,
    finished: bool,
    note: bool,
}

impl ItemPrintOptions {
    pub fn new(init_value: bool) -> Self {
        Self {
            id: init_value,
            message: init_value,
            level: init_value,
            style: init_value,
            today: init_value,
            notice: init_value,
            deadline: init_value,
            plan: init_value,
            repeat: init_value,
            list: init_value,
            finished: init_value,
            note: init_value,
        }
    }
    pub fn is_all_false(&self) -> bool {
        !(self.id || self.message || self.level || self.style || self.today || self.notice || self.deadline || self.plan || self.repeat || self.list || self.finished || self.note)
    }
    pub fn set_all(&mut self, value: bool) {
        self.id = value;
        self.message = value;
        self.level = value;
        self.style = value;
        self.today = value;
        self.notice = value;
        self.deadline = value;
        self.plan = value;
        self.repeat = value;
        self.list = value;
        self.finished = value;
        self.note = value;
    }
}

trait OptionToString {
    fn to_string_or(&self, none_string: String) -> String;
    fn to_string(&self) -> String {
        self.to_string_or(String::default())
    }
}

impl<T> OptionToString for Option<T>
    where T: ToString
{
    fn to_string_or(&self, none_string: String) -> String {
        match self {
            None => none_string,
            Some(s) => s.to_string(),
        }
    }
}