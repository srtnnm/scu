use super::{config, display::*};

use crate::{
    data::table::{self, TableEntry},
    modules::*,
    util::libc::get_nprocs_conf,
};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub(super) fn collect_tables(config: &config::TableConfig) -> Vec<table::Table> {
    use std::collections::VecDeque;

    let categories_count = config.categories.len();
    let modules_queue: Arc<Mutex<VecDeque<(usize, Module)>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    let mut modules_queue_lock = modules_queue.lock().unwrap();
    for (category_id, category) in config.categories.iter().enumerate() {
        for module in &category.modules {
            modules_queue_lock.push_back((category_id, *module));
        }
    }

    let titles: Vec<String> = config.categories.iter().map(|c| c.title.clone()).collect();

    let (s, r) = mpsc::channel::<(usize, TableEntry)>();
    let collector_thread = thread::spawn(move || {
        let mut result: Vec<table::Table> = Vec::with_capacity(categories_count);
        for title in titles {
            result.push(table::Table::new(&title))
        }

        while let Ok((i, entry)) = r.recv() {
            result[i].push_entry(entry);
        }

        result
    });

    let number_of_threads = unsafe { get_nprocs_conf() };
    let s = Arc::new(s);

    thread::scope(|scope| {
        for _ in 0..number_of_threads - 1 {
            let q = modules_queue.clone();
            let s = s.clone();
            scope.spawn(move || {
                while let Some((i, module)) = q.lock().ok().and_then(|mut q| q.pop_front()) {
                    let sender = DisplaySenderT::new(i, s.clone());
                    let _ = run_module(module, sender);
                }
            });
        }
    });

    drop(s);

    collector_thread
        .join()
        .expect("failed to join data collector thread")
}

macro_rules! gen_entries_for_module_func {
    ($($module:tt,)*) => {
        fn run_module(module: Module, sender: DisplaySenderT) -> std::io::Result<()> {
            match module {
                $(
                    Module::$module => { let _ = $module.run(sender); },
                )*
                _ => {}
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
