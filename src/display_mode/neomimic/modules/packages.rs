use super::{DataRow, ModuleTrait};

use crate::modules::get_vec;

pub struct Packages;

impl ModuleTrait for Packages {
    const NAME: &'static str = "packages";

    fn run(info: &crate::modules::SystemInformation) -> std::io::Result<usize> {
        let mut package_managers = get_vec("package managers", &info.packages)?;
        // sort by number of packages
        package_managers.sort_by(|pm1, pm2| pm2.number_of_packages.cmp(&pm1.number_of_packages));

        let mut value = String::default();
        for pm in package_managers {
            value.push_str(&format!(
                "{packages} ({manager}), ",
                packages = pm.number_of_packages,
                manager = pm.name
            ));
        }
        value.pop();
        value.pop();
        Ok(DataRow::info("Packages", &value))
    }
}
