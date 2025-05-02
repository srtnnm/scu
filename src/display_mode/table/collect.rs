use super::{config, display::*, table};

use crate::{modules::*, util::libc::get_nprocs_conf};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub(super) fn collect_tables(config: &config::TableConfig) -> Vec<table::Table> {
    use std::collections::VecDeque;

    let modules_queue: Arc<Mutex<VecDeque<((usize, usize), Module)>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let mut category_titles: Vec<String> = Vec::new();

    // dropping modules_queue lock to prevent deadlock
    {
        let mut modules_queue_lock = modules_queue.lock().unwrap();
        for (category_id, category) in config.categories.iter().enumerate() {
            category_titles.push(category.title.clone());

            for (module_id, module) in category.modules.iter().enumerate() {
                modules_queue_lock.push_back(((category_id, module_id), *module));
            }
        }
    }

    let (s, r) = mpsc::channel::<((usize, usize), table::TableEntry)>();
    let collector_thread = thread::spawn(move || collector(&category_titles, r));

    let number_of_threads = unsafe { get_nprocs_conf() };
    let s = Arc::new(s);

    // collect results of each module
    thread::scope(|scope| {
        for _ in 0..number_of_threads - 1 {
            let q = modules_queue.clone();
            let s = s.clone();
            scope.spawn(move || {
                while let Some((i, module)) = q.lock().ok().and_then(|mut q| q.pop_front()) {
                    let sender = DisplaySenderT::new(i, s.clone());
                    let _ = run_module(module, &sender);
                }
            });
        }
    });

    // manually drop mpsc::Sender to stop collector_thread on collecting results
    drop(s);

    collector_thread
        .join()
        .expect("failed to join data collector thread")
}

fn collector(
    titles: &Vec<String>,
    results_receiver: mpsc::Receiver<((usize, usize), table::TableEntry)>,
) -> Vec<table::Table> {
    let mut result: Vec<table::Table> = titles
        .iter()
        .map(|title| table::Table::new(&title))
        .collect();

    // buffer for unsorted results
    let mut category_entries: Vec<Vec<(usize, table::TableEntry)>> = vec![Vec::new(); titles.len()];

    // collect results of all modules
    while let Ok(((category_id, module_id), entry)) = results_receiver.recv() {
        if category_id < category_entries.len() {
            category_entries[category_id].push((module_id, entry));
        }
    }

    // sort entries in each table
    for (category_id, mut entries) in category_entries.into_iter().enumerate() {
        if category_id > result.len() {
            continue;
        }

        entries.sort_by_key(|(module_id, _)| *module_id);

        for (_, entry) in entries {
            result[category_id].push_entry(entry);
        }
    }

    result
}

macro_rules! generate_run_module_func {
    ($($module:tt,)*) => {
        fn run_module(module: Module, sender: &DisplaySenderT) -> std::io::Result<()> {
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
generate_run_module_func!(
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
