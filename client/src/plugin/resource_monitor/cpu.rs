use crate::plugin::utils::Plugin;

pub struct CpuPlugin {
    sysinfo: sysinfo::System,
    entries: Vec<crate::model::Entry>,
}

impl Plugin for CpuPlugin {
    fn id() -> &'static str {
        "resource_monitor_cpu"
    }

    fn priority() -> u32 {
        13
    }

    fn title() -> &'static str {
        "󰍛 CPU"
    }

    fn update_timeout() -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(2))
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        self.entries.clone()
    }

    fn set_entries(&mut self, entries: Vec<crate::model::Entry>) {
        self.entries = entries;
    }

    fn update_entries(&mut self) -> anyhow::Result<()> {
        self.sysinfo.refresh_cpu();
        self.entries.clear();

        let mut core_usages: String = format!(
            "{}% – {} Cores:",
            self.sysinfo.global_cpu_info().cpu_usage() as i32,
            self.sysinfo.cpus().len()
        );

        // Option type allows handling failures of the temperature retrieval
        let max_cpu_temp: Option<String> = {
            let new_with_refreshed_list = sysinfo::Components::new_with_refreshed_list();
            let cpu_with_max_temp = new_with_refreshed_list
                .iter()
                .filter(|component| {
                    component
                        .label()
                        // Multiple sensors may have the "cpu" in their name
                        .contains(self.sysinfo.global_cpu_info().name())
                })
                // Find the max temp
                .max_by(|left, right| {
                    left.temperature()
                        .partial_cmp(&right.temperature())
                        // temperatures come as f32, Rust only has partial ordering for them
                        // this unwrap_or handles NaN case by making it less than whatever is on the
                        // right. If two NaNs meet -- right wins.
                        .unwrap_or(std::cmp::Ordering::Less)
                });

            match cpu_with_max_temp {
                Some(cpu) => {
                    if cpu.temperature().is_nan() {
                        log::warn!("Components found, but cannot determine max temperature");
                        None
                    } else {
                        Some(format!("{}°C", cpu.temperature()))
                    }
                }
                None => {
                    log::warn!("Cannot find any CPUs");
                    None
                }
            }
        };

        for cpu_core in self.sysinfo.cpus() {
            let core_usage = match cpu_core.cpu_usage() as i32 {
                0..=12 => " ▁",
                13..=25 => " ▂",
                26..=37 => " ▃",
                38..=50 => " ▄",
                51..=62 => " ▅",
                63..=75 => " ▆",
                76..=87 => " ▇",
                _ => " █",
            };

            core_usages.push_str(core_usage);
        }

        self.entries = [Some(core_usages), max_cpu_temp]
            .iter()
            .flatten() // Remove None
            .map(|c| crate::model::Entry {
                id: "cpu".into(),
                title: c.to_string(),
                action: String::from(""),
                meta: String::from("Resource Monitor CPU"),
                command: None,
            })
            .collect();

        Ok(())
    }

    fn new() -> Self {
        Self {
            sysinfo: sysinfo::System::new_all(),
            entries: vec![],
        }
    }
}

impl Default for CpuPlugin {
    fn default() -> Self {
        Self::new()
    }
}
