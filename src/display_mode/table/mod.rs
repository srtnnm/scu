mod collect;
pub mod config;
mod display;
mod print;

pub(crate) fn run(config: &config::TableConfig) {
    print::print(config);
}
