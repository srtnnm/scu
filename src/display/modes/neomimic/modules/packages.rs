use crate::{display::modes::neomimic::row::DataRow, info::get_vec};

use super::ModuleTrait;

pub struct Packages;

impl ModuleTrait for Packages {
    const NAME: &'static str = "packages";

    fn get(
        info: &crate::info::SystemInformation,
    ) -> std::io::Result<crate::display::modes::neomimic::row::DataRow> {
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
        Ok(DataRow::info("Packages", value))
    }
}
