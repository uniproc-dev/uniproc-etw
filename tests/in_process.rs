use std::path::PathBuf;
use std::sync::Mutex;

use tracelogging as tlg;
use uniproc_etw::{Clock, ClockKind, Enable, Event, Options, OwnedEvent, Session, Timestamps, read_file};

tlg::define_provider!(PROVIDER, "Uniproc.Etw.Test.InProcess");

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
    fn GetCurrentProcessId() -> u32;
    fn QueryPerformanceCounter(count: *mut i64) -> i32;
    fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
}

fn qpc() -> i64 {
    let mut count = 0;
    unsafe { QueryPerformanceCounter(&mut count) };
    count
}

fn ring(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("{name}.etl"))
}

/// Records what `write` writes to PROVIDER in a private session, hands each
/// of its events to `on_event`, and tells the clock the file was read with.
fn record(test: &str, write: impl FnOnce(), mut on_event: impl FnMut(&Event<'_>)) -> Clock {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    unsafe { PROVIDER.register() };
    let provider = PROVIDER.id().to_u128();
    let name = format!("Uniproc-Etw-Test-{test}-{}", std::process::id());
    let file = ring(&name);

    let session = Session::start(&name, &Options::private_in_proc(&file, 4));
    assert!(session.is_ok(), "the session did not start: {:?}", session.err());
    let session = session.unwrap();
    let enabled = session.enable(provider, Enable::ALL);
    assert!(enabled.is_ok(), "the provider was not enabled: {enabled:?}");
    write();
    drop(session);

    let read = read_file(&file, Timestamps::Raw, |event| {
        if event.provider() == provider {
            on_event(event);
        }
    });
    let _ = std::fs::remove_file(&file);
    PROVIDER.unregister();
    assert!(read.is_ok(), "the file was not read: {read:?}");
    read.unwrap()
}

#[test]
fn a_raw_file_tells_the_qpc_frequency_its_times_count_in() {
    let clock = record(
        "Clock",
        || {
            tlg::write_event!(PROVIDER, "Written", u32("Number", &7));
        },
        |_| {},
    );

    let mut frequency = 0;
    unsafe { QueryPerformanceFrequency(&mut frequency) };
    assert_eq!(clock.kind, ClockKind::Qpc);
    assert_eq!(clock.frequency, frequency);
}

#[test]
fn an_in_process_session_records_this_thread_s_event_at_its_raw_qpc_time() {
    let mut told = Vec::new();
    let (mut before, mut after) = (0, 0);
    record(
        "Time",
        || {
            before = qpc();
            tlg::write_event!(PROVIDER, "Written", u32("Number", &7));
            after = qpc();
        },
        |event| told.push((event.thread_id(), event.timestamp())),
    );

    assert_eq!(told.len(), 1, "{told:?}");
    let (thread, time) = told[0];
    assert_eq!(thread, unsafe { GetCurrentThreadId() });
    assert!(before <= time && time <= after, "{time} is not between {before} and {after}");
}

#[test]
fn an_event_tells_its_header_and_its_bytes_as_written() {
    let mut told = Vec::new();
    record(
        "Header",
        || {
            tlg::write_event!(
                PROVIDER,
                "Header",
                level(Warning),
                keyword(0x30),
                opcode(Start),
                u32("Number", &7),
            );
        },
        |event| {
            told.push((
                event.level(),
                event.keywords() & 0xFFFF,
                event.opcode(),
                event.process_id(),
                event.is_64_bit(),
                event.user_data().to_vec(),
            ))
        },
    );

    assert_eq!(told.len(), 1, "{told:?}");
    let (level, keywords, opcode, process, is_64_bit, data) = &told[0];
    assert_eq!(*level, 3);
    assert_eq!(*keywords, 0x30);
    assert_eq!(*opcode, 1);
    assert_eq!(*process, unsafe { GetCurrentProcessId() });
    assert_eq!(*is_64_bit, cfg!(target_pointer_width = "64"));
    assert_eq!(data, &[7, 0, 0, 0]);
}

const SID: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];

fn write_fields() {
    let text: Vec<u16> = "Привет".encode_utf16().collect();
    tlg::write_event!(
        PROVIDER,
        "Fields",
        u64("Big", &0x1122_3344_5566_7788),
        u16("Small", &513),
        str16("Text", &text),
        str8_cp1252("Ansi", b"image.exe"),
        win_sid("User", &SID),
    );
}

#[derive(Debug)]
struct Fields {
    big: Option<u64>,
    small: Option<u64>,
    text: Option<String>,
    ansi: Option<String>,
    sid: Option<Vec<u8>>,
    missing: Option<u64>,
}

fn fields(event: &Event<'_>) -> Fields {
    Fields {
        big: event.number("Big"),
        small: event.number("Small"),
        text: event.text("Text"),
        ansi: event.ansi("Ansi"),
        sid: event.sid("User"),
        missing: event.number("Missing"),
    }
}

fn assert_fields(told: &[Fields]) {
    assert_eq!(told.len(), 1, "{told:?}");
    let told = &told[0];
    assert_eq!(told.big, Some(0x1122_3344_5566_7788));
    assert_eq!(told.small, Some(513));
    assert_eq!(told.text.as_deref(), Some("Привет"));
    assert_eq!(told.ansi.as_deref(), Some("image.exe"));
    assert_eq!(told.sid.as_deref(), Some(&SID[..]));
    assert_eq!(told.missing, None);
}

#[test]
fn an_event_s_fields_are_read_by_their_names() {
    let mut told = Vec::new();
    record("Fields", write_fields, |event| told.push(fields(event)));
    assert_fields(&told);
}

#[test]
fn a_copied_event_reads_as_the_event_on_another_thread() {
    let mut copies: Vec<OwnedEvent> = Vec::new();
    record("Copy", write_fields, |event| copies.push(event.to_owned()));

    let told = std::thread::spawn(move || copies.iter().map(|copy| fields(&copy.event())).collect::<Vec<_>>())
        .join()
        .unwrap();
    assert_fields(&told);
}
