extern crate todolst;
extern crate clap;

use todolst::components::{ group::*, list::*, item::*, todolst::* };
use clap::{ App, Arg, SubCommand };

fn main() {
    println!("Hello, world!");
    let _matches = App::new("TodoLst-cli")
                            .version("0.1")
                            .author("Hx Xiu")
                            .about("TodoLst Command Interface")
                            .subcommand(SubCommand::with_name("show")
                                    .about("print todo items.")
                                    .arg(Arg::with_name("group")
                                            .short("g")
                                            .long("group")
                                            .value_name("GROUP")
                                            .help("specify group in which items or lists showed."))
                                    .arg(Arg::with_name("list")
                                            .short("l")
                                            .long("list")
                                            .value_name("LIST")
                                            .help("specify list in which items showed."))
                                    .arg(Arg::with_name("groups")
                                            .long("groups")
                                            .conflicts_with("lists")
                                            .help("list groups instead of items"))
                                    .arg(Arg::with_name("lists")
                                            .long("lists")
                                            .help("print list instead of items")))
                            .subcommand(SubCommand::with_name("add")
                                    .about("add item, list or group.")
                                    .subcommand(SubCommand::with_name("item")
                                            .about("add item.")
                                            .arg(Arg::with_name("MESSAGE")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify item message"))
                                            .arg(Arg::with_name("LIST")
                                                    .index(2)
                                                    .required(true)
                                                    .help("specify list to which item added."))
                                            .arg(Arg::with_name("level")
                                                    .long("level")
                                                    .value_name("LEVEL")
                                                    .help("specify the item level, 0 indicates normal, higher the number, more important the item is."))
                                            .arg(Arg::with_name("today")
                                                    .long("today")
                                                    .value_name("TODAY")
                                                    .help("specify the item today."))
                                            .arg(Arg::with_name("notice")
                                                    .long("notice")
                                                    .value_name("NOTICE")
                                                    .help("specify the item notice."))
                                            .arg(Arg::with_name("deadline")
                                                    .long("deadline")
                                                    .value_name("DEADLINE")
                                                    .help("specify the item deadline."))
                                            .arg(Arg::with_name("plan")
                                                    .long("plan")
                                                    .value_name("PLAN")
                                                    .help("specify the item plan."))
                                            .arg(Arg::with_name("repeat")
                                                    .long("repeat")
                                                    .value_name("REPEAT")
                                                    .help("specify the item repeat span, format with {a number}(y|m|w|d), like \"1w\" means 1 week."))
                                            .arg(Arg::with_name("finished")
                                                    .long("finished")
                                                    .value_name("FINISHED")
                                                    .help("specify the item finished."))
                                            .arg(Arg::with_name("note")
                                                    .long("note")
                                                    .value_name("NOTE")
                                                    .help("specify the item note.")))
                                    .subcommand(SubCommand::with_name("list")
                                            .about("add list.")
                                            .arg(Arg::with_name("LIST")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify list title."))
                                            .arg(Arg::with_name("GROUP")
                                                    .index(2)
                                                    .help("specify group to which list attached.")))
                                    .subcommand(SubCommand::with_name("group")
                                            .about("add group.")
                                            .arg(Arg::with_name("GROUP")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify group title"))
                                            .arg(Arg::with_name("PARENT")
                                                    .index(2)
                                                    .help("specify parent group to which the new group attached."))))
                            .subcommand(SubCommand::with_name("modify")
                                    .about("modify an item, list or group.")
                                    .subcommand(SubCommand::with_name("item")
                                            .about("modify an item.")
                                            .arg(Arg::with_name("ID")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify item to modified."))
                                            .arg(Arg::with_name("message")
                                                    .long("message")
                                                    .value_name("MESSAGE")
                                                    .help("specify item message"))
                                            .arg(Arg::with_name("list")
                                                    .long("list")
                                                    .value_name("LIST")
                                                    .help("specify list to which item modified."))
                                            .arg(Arg::with_name("level")
                                                    .long("level")
                                                    .value_name("LEVEL")
                                                    .help("specify the item level, 0 indicates normal, higher the number, more important the item is."))
                                            .arg(Arg::with_name("today")
                                                    .long("today")
                                                    .value_name("TODAY")
                                                    .help("specify the item today."))
                                            .arg(Arg::with_name("notice")
                                                    .long("notice")
                                                    .value_name("NOTICE")
                                                    .help("specify the item notice."))
                                            .arg(Arg::with_name("deadline")
                                                    .long("deadline")
                                                    .value_name("DEADLINE")
                                                    .help("specify the item deadline."))
                                            .arg(Arg::with_name("plan")
                                                    .long("plan")
                                                    .value_name("PLAN")
                                                    .help("specify the item plan."))
                                            .arg(Arg::with_name("repeat")
                                                    .long("repeat")
                                                    .value_name("REPEAT")
                                                    .help("specify the item repeat span, format with {a number}(y|m|w|d), like \"1w\" means 1 week."))
                                            .arg(Arg::with_name("finished")
                                                    .long("finished")
                                                    .value_name("FINISHED")
                                                    .help("specify the item finished."))
                                            .arg(Arg::with_name("note")
                                                    .long("note")
                                                    .value_name("NOTE")
                                                    .help("specify the item note.")))
                                    .subcommand(SubCommand::with_name("list")
                                            .about("modify an list.")
                                            .arg(Arg::with_name("LIST")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify list title."))
                                            .arg(Arg::with_name("group")
                                                    .long("group")
                                                    .value_name("GROUP")
                                                    .help("specify group to which list attached."))
                                            .arg(Arg::with_name("title")
                                                    .long("title")
                                                    .value_name("TITLE")
                                                    .help("specify new title.")))
                                    .subcommand(SubCommand::with_name("group")
                                            .about("modify an group.")
                                            .arg(Arg::with_name("GROUP")
                                                    .index(1)
                                                    .required(true)
                                                    .help("specify group title"))
                                            .arg(Arg::with_name("parent")
                                                    .long("parent")
                                                    .value_name("PARENT")
                                                    .help("specify parent group to which the new group attached."))
                                            .arg(Arg::with_name("TITLE")
                                                    .long("title")
                                                    .value_name("TITLE")
                                                    .help("specify new title."))))
                            .subcommand(SubCommand::with_name("delete")
                                    .about("delete an item, list or group.")
                                    .subcommand(SubCommand::with_name("item")
                                            .about("delete an item.")
                                            .arg(Arg::with_name("ID")
                                                    .index(1)
                                                    .help("specify item to be deleted.")))
                                    .subcommand(SubCommand::with_name("list")
                                            .about("delete an list.")
                                            .arg(Arg::with_name("LIST")
                                                    .index(1)
                                                    .help("specify list to be deleted.")))
                                    .subcommand(SubCommand::with_name("group")
                                            .about("delete an group.")
                                            .arg(Arg::with_name("GROUP")
                                                    .index(1)
                                                    .help("specify group to be deleted."))))
                            .get_matches();

    
    
}
