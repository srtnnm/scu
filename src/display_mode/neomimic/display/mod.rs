mod hardware;
mod misc;
pub use misc::*;
mod software;

use super::{
    color_blocks,
    config::NeomimicConfig,
    modules::run_module,
    row::{plus_n_lines, NUMBER_OF_ROWS_PRINTED},
};

use crate::{
    config::{no_colors, no_logo, simplify},
    display_mode::{
        neomimic::logo::{logo_height, logo_width},
        DisplaySender,
    },
    modules::Module,
};

use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, mpsc, Arc, Mutex},
    thread,
};

pub(super) static LAST_ROW_LENGTH: AtomicUsize = AtomicUsize::new(0);

static CURSOR_MOVER: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

pub fn cursor_mover() {
    print!("{}", CURSOR_MOVER.get_or_init(|| ""));
}

pub fn display(config: NeomimicConfig) {
    if !simplify() {
        // Display logo
        config.logo.print();
        // Return cursor to start
        println!("\x1b[{}A\x1b[9999999D", logo_height());
    }

    let logo_width = logo_width();
    if !no_logo() {
        CURSOR_MOVER
            .set(if !simplify() {
                Box::leak(
                    format!("\x1b[{}C", if logo_width == 0 { 0 } else { logo_width + 4 })
                        .into_boxed_str(),
                )
            } else {
                "".into()
            })
            .expect("attempted to set already initialized cursor mover");
    }

    run(config);

    // Display color blocks
    if !no_colors() {
        color_blocks::print();
        plus_n_lines(3);
    }

    if !no_logo() {
        let diff = logo_height() as isize
            - NUMBER_OF_ROWS_PRINTED.load(std::sync::atomic::Ordering::Relaxed) as isize;
        if diff > 0 {
            print!("{}", "\n".repeat(diff as usize));
        }
    }
}

extern "C" {
    pub fn get_nprocs_conf() -> std::ffi::c_int;
}
fn run(config: NeomimicConfig) {
    let (s, r) = mpsc::channel::<(usize, String)>();
    let s = Arc::new(s);

    let num_of_modules = config.modules.len();
    let q = Arc::new(Mutex::new(VecDeque::from(
        config
            .modules
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, Module)>>(),
    )));
    let num_of_threads = unsafe { get_nprocs_conf() } as usize;

    let printer_thread = thread::spawn(move || {
        let mut results = Vec::with_capacity(num_of_modules);
        for _ in 0..num_of_modules {
            results.push(Vec::new());
        }

        while let Ok((i, result)) = r.recv() {
            results[i].push(result);
        }

        for (i, module_results) in results.iter().enumerate() {
            if config.modules[i] == Module::Separator {
                cursor_mover();
                println!(
                    "{}",
                    "-".repeat(LAST_ROW_LENGTH.load(std::sync::atomic::Ordering::Relaxed))
                )
            } else {
                for line in module_results {
                    LAST_ROW_LENGTH.store(line.len(), std::sync::atomic::Ordering::Relaxed);
                    cursor_mover();
                    println!("{line}");
                }
            }
        }
    });

    thread::scope(|scope| {
        for _ in 0..num_of_threads - 1 {
            let q = q.clone();
            let s = s.clone();
            scope.spawn(move || {
                while let Some((i, module)) = q.lock().ok().and_then(|mut q| q.pop_front()) {
                    let sender = RowSenderT::new(i, s.clone());
                    run_module(&module, sender);
                }
            });
        }
    });

    drop(s);

    printer_thread.join().unwrap();
}

pub trait Display: crate::modules::Detection {
    fn run(&self, sender: RowSenderT) -> std::io::Result<()> {
        Self::display(self.fetch()?, &sender)
    }
    fn display(data: Self::Result, sender: &RowSenderT) -> std::io::Result<()>;
}

pub type RowSenderT = DisplaySender<usize, String>;
