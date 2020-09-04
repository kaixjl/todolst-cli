use clap::{ ArgMatches };
use crate::utils::{ load_todolst, save_todolst };

pub fn delete_command(sub_m: &ArgMatches) {
    match sub_m.subcommand() {
        ("group", Some(sub_m)) => { delete_group_command(sub_m) },
        ("list", Some(sub_m)) => { delete_list_command(sub_m) },
        ("item", Some(sub_m)) => { delete_item_command(sub_m) },
        _ => (),
    }
}

fn delete_group_command(sub_m: &ArgMatches) {
    let title = sub_m.values_of("GROUP").expect("GROUP should not be empty.").collect::<Vec<&str>>();
    let mut system = load_todolst();
    for title in title {
        system.remove_group(title);
    }
    save_todolst(&system);


}

fn delete_list_command(sub_m: &ArgMatches) {
    let title = sub_m.values_of("LIST").expect("LIST should not be empty.").collect::<Vec<&str>>();
    let mut system = load_todolst();
    for title in title {
        system.remove_list(title);
    }
    save_todolst(&system);
}

fn delete_item_command(sub_m: &ArgMatches) {
    let id = sub_m.values_of("ID").expect("ID should not be empty.").map(|i| {
        i.parse::<u32>().expect("ID should be parsed to a number(u32).")
    }).collect::<Vec<u32>>();
    let mut system = load_todolst();
    for id in id {
        let item = system.item(id).expect("Can not found the item with that ID.");
        system.remove_item(item);
    }
    save_todolst(&system);

}