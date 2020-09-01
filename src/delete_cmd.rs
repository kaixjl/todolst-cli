use todolst::components::{ group::*, list::*, item::*, todolst::* };
use clap::{ App, Arg, SubCommand, ArgMatches };
use futures::executor::block_on;
use crate::utils::{ load_todolst, save_todolst };

pub fn delete_command(sub_m: &ArgMatches) {
    
}