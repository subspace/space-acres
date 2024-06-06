const SECOND: u64 = 1;
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;
const WEEK: u64 = 7 * DAY;

/// Format ETA from milliseconds to an approximate time duration
pub(crate) fn format_eta(eta_secs: u64) -> String {
    match eta_secs {
        0 => "any time now".to_string(),
        _ if eta_secs < SECOND => "any time now".to_string(),
        _ if eta_secs >= WEEK => format!("{} weeks", eta_secs / WEEK),
        _ if eta_secs >= DAY => format!("{} days", eta_secs / DAY),
        _ if eta_secs >= HOUR => format!("{} hours", eta_secs / HOUR),
        _ if eta_secs >= MINUTE => format!("{} mins", eta_secs / MINUTE),
        _ => format!("{} secs", eta_secs / SECOND),
    }
}

#[test]
fn test_format_eta() {
    assert_eq!(format_eta(0), "any time now");
    assert_eq!(format_eta(999), "any time now");
    assert_eq!(format_eta(1000), "1 secs");
    assert_eq!(format_eta(60_000), "1 mins");
    assert_eq!(format_eta(60_999), "1 mins");
    assert_eq!(format_eta(120_000), "2 mins");
    assert_eq!(format_eta(3_600_000), "1 hours");
    assert_eq!(format_eta(3_600_999), "1 hours");
    assert_eq!(format_eta(3_661_000), "1 hours");
    assert_eq!(format_eta(86_400_000), "1 days");
    assert_eq!(format_eta(86_460_000), "1 days");
    assert_eq!(format_eta(604_800_000), "1 weeks");
    assert_eq!(format_eta(604_860_000), "1 weeks");
    assert_eq!(format_eta(1_000_000), "16 mins");
    assert_eq!(format_eta(10_000_000), "2 hours");
    assert_eq!(format_eta(864_000_000), "1 weeks");
}
