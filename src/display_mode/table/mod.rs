mod collect;
pub mod config;
mod gen_table;
mod print;

pub(crate) fn run(config: &config::TableConfig) {
    print::print(config);
}
