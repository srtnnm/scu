use super::{config, gen_table::*};

use crate::{
    data::table::{self, Table},
    modules::*,
};

pub(super) fn collect_tables(config: &config::TableConfig) -> Vec<table::Table> {
    let mut result: Vec<table::Table> = Vec::new();

    for config_table in config.categories.iter() {
        let mut table = Table::new(&config_table.title);

        gen_entries_for_modules(&mut table, &config_table.modules);

        result.push(table);
    }

    result
}

macro_rules! gen_entries_for_module_func {
    ($($module:tt,)*) => {
        fn gen_entries_for_modules(table: &mut Table, modules: &[Module]) -> std::io::Result<()> {
            for module in modules {
                match module {
                    $(
                        &Module::$module => { $module.gen_entries(table); },
                    )*
                    _ => {}
                }
            }
            Ok(())
        }
    };
}

// TODO: maybe add Arch,Header and Separator modules for Table ?
gen_entries_for_module_func!(
    // Arch,
    Battery,
    Brightness,
    CPU,
    DE,
    Device,
    Disks,
    DisplayServer,
    GPU,
    // Header,
    Hostname,
    Init,
    Kernel,
    Locale,
    Memory,
    OS,
    Packages,
    RootFS,
    // Separator,
    Shell,
    Terminal,
    Uptime,
    Username,
    WM,
);
