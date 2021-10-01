//! Conversions between the XRP Ledger's 'Ripple Epoch' time and native time
//! data types.

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use std::fmt::Display;
use std::fmt::Formatter;

/// The "Ripple Epoch" of 2000-01-01T00:00:00 UTC
pub const RIPPLE_EPOCH: i64 = 946684800;
/// The maximum time that can be expressed on the XRPL
pub const MAX_XRPL_TIME: i64 = i64::pow(2, 32);

#[derive(Debug)]
/// Exception for invalid XRP Ledger time data.
pub struct XRPLTimeRangeException {
    time: i64,
}

impl Display for XRPLTimeRangeException {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.time < 0 {
            write!(f, "{} is before the Ripple Epoch.", self.time)
        } else if self.time > MAX_XRPL_TIME {
            write!(
                f,
                "{} is larger than any time that can be expressed on the XRP Ledger.",
                self.time
            )
        } else {
            write!(f, "Unknown error for {}", self.time)
        }
    }
}

/// Convert from XRP Ledger 'Ripple Epoch' time to a UTC datetime
/// See [`chrono::DateTime`]
///
/// [`chrono::DateTime`]: chrono::DateTime
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use chrono::DateTime;
/// use xrpl_rust::utils::time_conversion::ripple_time_to_datetime;
///
/// let date_time = ripple_time_to_datetime(946684801);
/// ```
pub fn ripple_time_to_datetime(ripple_time: i64) -> Result<DateTime<Utc>, XRPLTimeRangeException> {
    if ripple_time < 0 || ripple_time > MAX_XRPL_TIME {
        Err(XRPLTimeRangeException { time: ripple_time })
    } else {
        Ok(Utc.timestamp(ripple_time + RIPPLE_EPOCH, 0))
    }
}

/// Convert from a [`chrono::DateTime`] object to an XRP Ledger
/// 'Ripple Epoch' time.
///
/// [`chrono::DateTime`]: chrono::DateTime
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use xrpl_rust::utils::time_conversion::datetime_to_ripple_time;
///
/// let timestamp = datetime_to_ripple_time(Utc.timestamp(946684801, 0));
/// ```
pub fn datetime_to_ripple_time(dt: DateTime<Utc>) -> Result<i64, XRPLTimeRangeException> {
    let ripple_time = dt.timestamp() - RIPPLE_EPOCH;

    if ripple_time < 0 || ripple_time > MAX_XRPL_TIME {
        Err(XRPLTimeRangeException { time: ripple_time })
    } else {
        Ok(ripple_time)
    }
}

/// Convert from XRP Ledger 'Ripple Epoch' time to a POSIX-like
/// integer timestamp.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use xrpl_rust::utils::time_conversion::ripple_time_to_posix;
///
/// let timestamp = ripple_time_to_posix(946684801);
/// ```
pub fn ripple_time_to_posix(ripple_time: i64) -> Result<i64, XRPLTimeRangeException> {
    if ripple_time < 0 || ripple_time > MAX_XRPL_TIME {
        Err(XRPLTimeRangeException { time: ripple_time })
    } else {
        Ok(ripple_time + RIPPLE_EPOCH)
    }
}

/// Convert from a POSIX-like timestamp to an XRP Ledger
/// 'Ripple Epoch' time.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use xrpl_rust::utils::time_conversion::posix_to_ripple_time;
///
/// let timestamp = posix_to_ripple_time(946684801);
/// ```
pub fn posix_to_ripple_time(timestamp: i64) -> Result<i64, XRPLTimeRangeException> {
    let ripple_time = timestamp - RIPPLE_EPOCH;

    if ripple_time < 0 || ripple_time > MAX_XRPL_TIME {
        Err(XRPLTimeRangeException { time: timestamp })
    } else {
        Ok(ripple_time)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ripple_time_to_datetime() {
        let success: DateTime<Utc> = ripple_time_to_datetime(RIPPLE_EPOCH).unwrap();
        assert_eq!(success.timestamp(), RIPPLE_EPOCH + RIPPLE_EPOCH);
    }

    #[test]
    fn test_datetime_to_ripple_time() {
        let success: i64 = datetime_to_ripple_time(Utc.timestamp(RIPPLE_EPOCH, 0)).unwrap();
        assert_eq!(success, RIPPLE_EPOCH - RIPPLE_EPOCH);
    }

    #[test]
    fn test_ripple_time_to_posix() {
        let success: i64 = ripple_time_to_posix(RIPPLE_EPOCH).unwrap();
        assert_eq!(success, RIPPLE_EPOCH + RIPPLE_EPOCH);
    }

    #[test]
    fn test_posix_to_ripple_time() {
        let success: i64 = posix_to_ripple_time(RIPPLE_EPOCH).unwrap();
        assert_eq!(success, RIPPLE_EPOCH - RIPPLE_EPOCH);
    }
}
