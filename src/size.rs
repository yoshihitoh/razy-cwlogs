use std::convert::TryFrom;
use std::fmt;

use thiserror::Error;

mod byte {
    pub const MULTIPLIER: u32 = 0;
    pub const SHORT_NAME: &str = "B";
    pub const MIN: u128 = 0;
    pub const MAX: u128 = 1_023;
}

mod kibi_byte {
    pub const MULTIPLIER: u32 = 1;
    pub const SHORT_NAME: &str = "KiB";
    pub const MIN: u128 = 1_024;
    pub const MAX: u128 = 1_048_575;
}

mod mebi_byte {
    pub const MULTIPLIER: u32 = 2;
    pub const SHORT_NAME: &str = "MiB";
    pub const MIN: u128 = 1_048_576;
    pub const MAX: u128 = 1_073_741_823;
}

mod gibi_byte {
    pub const MULTIPLIER: u32 = 3;
    pub const SHORT_NAME: &str = "GiB";
    pub const MIN: u128 = 1_073_741_824;
    pub const MAX: u128 = 1_099_511_627_775;
}

mod tebi_byte {
    pub const MULTIPLIER: u32 = 4;
    pub const SHORT_NAME: &str = "TiB";
    pub const MIN: u128 = 1_099_511_627_776;
    pub const MAX: u128 = 1_125_899_906_842_623;
}

mod pebi_byte {
    pub const MULTIPLIER: u32 = 5;
    pub const SHORT_NAME: &str = "PiB";
    pub const MIN: u128 = 1_125_899_906_842_624;
    pub const MAX: u128 = 1_152_921_504_606_846_975;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SizeUnit {
    Byte,
    KibiByte,
    MebiByte,
    GibiByte,
    TebiByte,
    PebiByte,
}

impl SizeUnit {
    pub fn short_name(&self) -> &'static str {
        use SizeUnit::*;
        match *self {
            Byte => byte::SHORT_NAME,
            KibiByte => kibi_byte::SHORT_NAME,
            MebiByte => mebi_byte::SHORT_NAME,
            GibiByte => gibi_byte::SHORT_NAME,
            TebiByte => tebi_byte::SHORT_NAME,
            PebiByte => pebi_byte::SHORT_NAME,
        }
    }

    pub fn max(&self) -> u128 {
        use SizeUnit::*;
        match *self {
            Byte => byte::MAX,
            KibiByte => kibi_byte::MAX,
            MebiByte => mebi_byte::MAX,
            GibiByte => gibi_byte::MAX,
            TebiByte => tebi_byte::MAX,
            PebiByte => pebi_byte::MAX,
        }
    }

    pub fn min(&self) -> u128 {
        use SizeUnit::*;
        match *self {
            Byte => byte::MIN,
            KibiByte => kibi_byte::MIN,
            MebiByte => mebi_byte::MIN,
            GibiByte => gibi_byte::MIN,
            TebiByte => tebi_byte::MIN,
            PebiByte => pebi_byte::MIN,
        }
    }

    pub fn multiplier(&self) -> u32 {
        use SizeUnit::*;
        match *self {
            Byte => byte::MULTIPLIER,
            KibiByte => kibi_byte::MULTIPLIER,
            MebiByte => mebi_byte::MULTIPLIER,
            GibiByte => gibi_byte::MULTIPLIER,
            TebiByte => tebi_byte::MULTIPLIER,
            PebiByte => pebi_byte::MULTIPLIER,
        }
    }
}

fn preferred_size_unit(n: u128) -> Option<SizeUnit> {
    use SizeUnit::*;

    if n == 0 {
        Some(Byte)
    } else {
        // too calculate number of digits correctly, cut off too big number.
        let (offset, n) = if n > gibi_byte::MAX {
            (4, n / (1024 * 1024 * 1024 * 1024))
        } else {
            (0, n)
        };

        let multiplier = (n as f64).log(1024_f64) as u32;
        match multiplier + offset {
            byte::MULTIPLIER => Some(Byte),
            kibi_byte::MULTIPLIER => Some(KibiByte),
            mebi_byte::MULTIPLIER => Some(MebiByte),
            gibi_byte::MULTIPLIER => Some(GibiByte),
            tebi_byte::MULTIPLIER => Some(TebiByte),
            pebi_byte::MULTIPLIER => Some(PebiByte),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum SizeError {
    #[error("size cannot be negative")]
    Negative,

    #[error("size must be within max value of pebi byte")]
    TooLarge,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size {
    in_bytes: u128,
}

impl Size {
    pub fn new(in_bytes: u128) -> Size {
        Size { in_bytes }
    }
}

impl TryFrom<i64> for Size {
    type Error = SizeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Size::new(value as u128))
        } else {
            Err(SizeError::Negative)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HumanReadableSize {
    pub size: u128,
    pub unit: SizeUnit,
}

impl HumanReadableSize {
    pub fn new(size: u128, unit: SizeUnit) -> HumanReadableSize {
        HumanReadableSize { size, unit }
    }
}

// FIXME: Replace to Display macro  from derive_more
impl fmt::Display for HumanReadableSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.size, self.unit.short_name())
    }
}

impl TryFrom<Size> for HumanReadableSize {
    type Error = SizeError;

    fn try_from(value: Size) -> Result<Self, Self::Error> {
        let unit = preferred_size_unit(value.in_bytes)
            .map(Ok)
            .unwrap_or(Err(SizeError::TooLarge))?;
        let size = value.in_bytes / 1024_u128.pow(unit.multiplier());
        Ok(HumanReadableSize::new(size, unit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_unit_short_name() {
        use SizeUnit::*;

        assert_eq!("B", Byte.short_name());
        assert_eq!("KiB", KibiByte.short_name());
        assert_eq!("MiB", MebiByte.short_name());
        assert_eq!("GiB", GibiByte.short_name());
        assert_eq!("TiB", TebiByte.short_name());
        assert_eq!("PiB", PebiByte.short_name());
    }

    #[test]
    fn size_unit_min_max() {
        use SizeUnit::*;

        assert_eq!(0, Byte.min());
        assert_eq!(1_023, Byte.max());

        assert_eq!(1_024, KibiByte.min());
        assert_eq!(1_048_575, KibiByte.max());

        assert_eq!(1_048_576, MebiByte.min());
        assert_eq!(1_073_741_823, MebiByte.max());

        assert_eq!(1_073_741_824, GibiByte.min());
        assert_eq!(1_099_511_627_775, GibiByte.max());

        assert_eq!(1_099_511_627_776, TebiByte.min());
        assert_eq!(1_125_899_906_842_623, TebiByte.max());

        assert_eq!(1_125_899_906_842_624, PebiByte.min());
        assert_eq!(1_152_921_504_606_846_975, PebiByte.max());
    }

    #[test]
    fn size_try_from() {
        assert_eq!(Ok(Size::new(0)), Size::try_from(0));
        assert_eq!(
            Ok(Size::new(1_152_921_504_606_846_975)),
            Size::try_from(1_152_921_504_606_846_975)
        );

        // unsupported size
        assert_eq!(Err(SizeError::Negative), Size::try_from(-1));
    }

    #[test]
    fn human_readable_size_try_from() {
        use SizeUnit::*;

        // bytes
        assert_eq!(
            Ok(HumanReadableSize::new(0, Byte)),
            HumanReadableSize::try_from(Size::new(0))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, Byte)),
            HumanReadableSize::try_from(Size::new(1023))
        );

        // kibi bytes
        assert_eq!(
            Ok(HumanReadableSize::new(1, KibiByte)),
            HumanReadableSize::try_from(Size::new(1024))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1, KibiByte)),
            HumanReadableSize::try_from(Size::new(2047))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(2, KibiByte)),
            HumanReadableSize::try_from(Size::new(2048))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, KibiByte)),
            HumanReadableSize::try_from(Size::new(1_048_575))
        );

        // mebi bytes
        assert_eq!(
            Ok(HumanReadableSize::new(1, MebiByte)),
            HumanReadableSize::try_from(Size::new(1_048_576))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, MebiByte)),
            HumanReadableSize::try_from(Size::new(1_073_741_823))
        );

        // gibi bytes
        assert_eq!(
            Ok(HumanReadableSize::new(1, GibiByte)),
            HumanReadableSize::try_from(Size::new(1_073_741_824))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, GibiByte)),
            HumanReadableSize::try_from(Size::new(1_099_511_627_775))
        );

        // tebi bytes
        assert_eq!(
            Ok(HumanReadableSize::new(1, TebiByte)),
            HumanReadableSize::try_from(Size::new(1_099_511_627_776))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, TebiByte)),
            HumanReadableSize::try_from(Size::new(1_125_899_906_842_623))
        );

        // pebi bytes
        assert_eq!(
            Ok(HumanReadableSize::new(1, PebiByte)),
            HumanReadableSize::try_from(Size::new(1_125_899_906_842_624))
        );
        assert_eq!(
            Ok(HumanReadableSize::new(1023, PebiByte)),
            HumanReadableSize::try_from(Size::new(1_152_921_504_606_846_975))
        );

        // unsupported size
        assert_eq!(
            Err(SizeError::TooLarge),
            HumanReadableSize::try_from(Size::new(1_152_921_504_606_846_976))
        );
    }
}
