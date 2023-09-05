use std::convert::TryFrom;
use std::num::NonZeroU16;
use std::time::SystemTime;

pub struct HostEnv {
    time: SystemTime,
    number_cpus_physical: NonZeroU16,
    number_cpus_logical: NonZeroU16,
}

impl HostEnv {
    pub fn detect() -> Self {
        Self {
            time: SystemTime::now(),
            number_cpus_physical: NonZeroU16::new(
                u16::try_from(num_cpus::get_physical())
                    .unwrap_or(u16::MAX)
                    .max(1),
            )
            .unwrap(),
            number_cpus_logical: NonZeroU16::new(
                u16::try_from(num_cpus::get()).unwrap_or(u16::MAX).max(1),
            )
            .unwrap(),
        }
    }

    pub const fn time(&self) -> SystemTime {
        self.time
    }

    pub const fn number_cpus_physical(&self) -> NonZeroU16 {
        self.number_cpus_physical
    }

    pub const fn number_cpus_logical(&self) -> NonZeroU16 {
        self.number_cpus_logical
    }
}
