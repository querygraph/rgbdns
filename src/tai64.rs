//! TAI64N timestamp generation, parsing, and stream filters.

use chrono::{Datelike, Local, TimeZone};
use std::{
    io::{self, BufRead, Write},
    time::{SystemTime, UNIX_EPOCH},
};

const TAI64_BIAS: u64 = 1_u64 << 62;
const INITIAL_TAI_UTC_OFFSET: i64 = 10;

// UTC instants immediately after each positive leap second since 1972.
const LEAP_TRANSITIONS: [i64; 27] = [
    78_796_800,
    94_694_400,
    126_230_400,
    157_766_400,
    189_302_400,
    220_924_800,
    252_460_800,
    283_996_800,
    315_532_800,
    362_793_600,
    394_329_600,
    425_865_600,
    489_024_000,
    567_993_600,
    631_152_000,
    662_688_000,
    709_948_800,
    741_484_800,
    773_020_800,
    820_454_400,
    867_715_200,
    915_148_800,
    1_136_073_600,
    1_230_768_000,
    1_341_100_800,
    1_435_708_800,
    1_483_228_800,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timestamp {
    pub unix_seconds: i64,
    pub nanoseconds: u32,
}

pub fn label(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let unix_seconds = i64::try_from(duration.as_secs()).unwrap_or(i64::MAX);
    let tai_seconds = unix_seconds.saturating_add(offset_at_utc(unix_seconds));
    format!(
        "@{:016x}{:08x}",
        TAI64_BIAS.saturating_add(tai_seconds.max(0) as u64),
        duration.subsec_nanos()
    )
}

pub fn parse_label(value: &str) -> Option<Timestamp> {
    if value.len() != 25 || !value.starts_with('@') {
        return None;
    }
    let seconds = u64::from_str_radix(&value[1..17], 16).ok()?;
    let nanoseconds = u32::from_str_radix(&value[17..25], 16).ok()?;
    if seconds < TAI64_BIAS || nanoseconds >= 1_000_000_000 {
        return None;
    }
    let tai_seconds = i64::try_from(seconds - TAI64_BIAS).ok()?;
    let unix_seconds = tai_to_unix(tai_seconds);
    Some(Timestamp {
        unix_seconds,
        nanoseconds,
    })
}

pub fn local(value: Timestamp) -> Option<String> {
    let date = Local
        .timestamp_opt(value.unix_seconds, value.nanoseconds)
        .single()?;
    if !(0..=9999).contains(&date.year()) {
        return None;
    }
    Some(format!(
        "{}.{:09}",
        date.format("%Y-%m-%d %H:%M:%S"),
        value.nanoseconds
    ))
}

pub fn stamp<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut line_start = true;
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            break;
        }
        let length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let segment = &available[..length];
        if line_start {
            output.write_all(label(SystemTime::now()).as_bytes())?;
            output.write_all(b" ")?;
        }
        output.write_all(segment)?;
        line_start = segment.last() == Some(&b'\n');
        input.consume(length);
    }
    output.flush()
}

pub fn localize<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut prefix = Vec::with_capacity(25);
    loop {
        prefix.clear();
        while prefix.len() < 25 {
            let available = input.fill_buf()?;
            if available.is_empty() {
                break;
            }
            let remaining = 25 - prefix.len();
            let length = available
                .iter()
                .take(remaining)
                .position(|byte| *byte == b'\n')
                .map_or(available.len().min(remaining), |position| position + 1);
            prefix.extend_from_slice(&available[..length]);
            input.consume(length);
            if prefix.last() == Some(&b'\n') {
                break;
            }
        }
        if prefix.is_empty() {
            break;
        }
        if prefix.len() == 25
            && let Ok(value) = std::str::from_utf8(&prefix)
            && let Some(timestamp) = parse_label(value).and_then(local)
        {
            output.write_all(timestamp.as_bytes())?;
        } else {
            output.write_all(&prefix)?;
        }
        if prefix.last() != Some(&b'\n') {
            copy_line_remainder(&mut input, &mut output)?;
        }
    }
    output.flush()
}

fn copy_line_remainder<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            return Ok(());
        }
        let length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let line_ended = available[length - 1] == b'\n';
        output.write_all(&available[..length])?;
        input.consume(length);
        if line_ended {
            return Ok(());
        }
    }
}

fn offset_at_utc(unix_seconds: i64) -> i64 {
    INITIAL_TAI_UTC_OFFSET
        + LEAP_TRANSITIONS.partition_point(|transition| *transition <= unix_seconds) as i64
}

fn tai_to_unix(tai_seconds: i64) -> i64 {
    let mut offset = INITIAL_TAI_UTC_OFFSET;
    for transition in LEAP_TRANSITIONS {
        let next_offset = offset + 1;
        if tai_seconds < transition + next_offset {
            break;
        }
        offset = next_offset;
    }
    tai_seconds - offset
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Cursor, time::Duration};

    #[test]
    fn matches_the_published_daemontools_example() {
        let parsed = parse_label("@4000000037c219bf2ef02e94").unwrap();
        assert_eq!(parsed.unix_seconds, 935_467_423);
        assert_eq!(parsed.nanoseconds, 787_492_500);
    }

    #[test]
    fn current_labels_include_the_post_2017_offset() {
        let time = UNIX_EPOCH + Duration::from_secs(1_483_228_800);
        assert_eq!(&label(time)[..17], "@40000000586846a5");
        assert_eq!(
            parse_label(&label(time)).unwrap(),
            Timestamp {
                unix_seconds: 1_483_228_800,
                nanoseconds: 0,
            }
        );
    }

    #[test]
    fn stream_filters_preserve_lines_and_invalid_prefixes() {
        let mut stamped = Vec::new();
        stamp(Cursor::new(b"one\ntwo"), &mut stamped).unwrap();
        let text = String::from_utf8(stamped).unwrap();
        assert_eq!(text.lines().count(), 2);
        assert!(text.lines().all(|line| line.starts_with("@4")));

        let mut localized = Vec::new();
        localize(
            Cursor::new(b"not-a-stamp\n@4000000037c219bf2ef02e94 mark\n"),
            &mut localized,
        )
        .unwrap();
        let localized = String::from_utf8(localized).unwrap();
        assert!(localized.starts_with("not-a-stamp\n"));
        assert!(localized.ends_with(".787492500 mark\n"));
    }

    #[test]
    fn rejects_malformed_and_out_of_range_labels() {
        assert!(parse_label("4000000037c219bf2ef02e94").is_none());
        assert!(parse_label("@4000000037c219bfzzzzzzzz").is_none());
        assert!(parse_label("@4000000037c219bf3b9aca00").is_none());
    }
}
