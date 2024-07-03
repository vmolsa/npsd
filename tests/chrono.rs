#[cfg(feature = "chrono")]
use npsd::{Payload, Schema, Info};

#[cfg(feature = "chrono")]
use chrono::{/* NaiveDate, NaiveDateTime, NaiveTime, */ Utc, Local, DateTime, FixedOffset, TimeZone};

#[cfg(feature = "chrono")]
#[derive(Schema, Info, Clone, PartialEq, Debug)]
struct ChronoTime {
    date0: DateTime<Utc>,
    date1: DateTime<Local>,
    date2: DateTime<FixedOffset>,
    // date3: NaiveDate, // TODO(): Not implemented
    // date4: NaiveTime, // TODO(): Not implemented
    // date5: NaiveDateTime, // TODO(): Not implemented
}

#[cfg(feature = "chrono")]
#[test]
fn test_chrono_payload() {
    use pretty_hex::PrettyHex;

    #[cfg(feature = "info")]
    use npsd::NextTrace;

    #[cfg(not(feature = "info"))]
    use npsd::Next;

    let mut ctx = ();

    // Create Middleware
    #[cfg(not(feature = "info"))]
    let mut next = Next::default();

    #[cfg(feature = "info")]
    let mut next = NextTrace::default();

    let instance = ChronoTime {
        date0: Utc.timestamp_opt(61, 0).unwrap(),
        date1: Utc::now().with_timezone(&Local),
        date2: FixedOffset::east_opt(5 * 12).unwrap().with_ymd_and_hms(2016, 11, 08, 0, 0, 0).unwrap()
        // date3: NaiveDate::from_ymd_opt(2024, 6, 29).unwrap(), // TODO(): Not implemented
        // date4: NaiveTime::from_hms_micro_opt(12, 34, 56, 789_000).unwrap(), // TODO(): Not implemented
        // date5: NaiveDateTime::new(NaiveDate::from_ymd_opt(2024, 6, 29).unwrap(), NaiveTime::from_hms_opt(12, 34, 56).unwrap()), // TODO(): Not implemented
    };

    instance.into_packet(&mut ctx, &mut next).unwrap();

    println!("Encoded: {:?}", next.serialized().hex_dump());

    let deserialized = ChronoTime::from_packet(&mut ctx, &mut next).unwrap();

    assert_eq!(instance, deserialized);
}
