use std::sync::{Mutex, mpsc};
use std::time::{Duration, Instant};

use tracelogging as tlg;
use uniproc_etw::{Enable, Options, Session, Timestamps, Trace, running, stop};

tlg::define_provider!(PROVIDER, "Uniproc.Etw.Test.Live");

const DISK_IO: u32 = 0x0000_0100;
const NETWORK_TCPIP: u32 = 0x0001_0000;
const PROFILE: u32 = 0x0100_0000;

static KERNEL: Mutex<()> = Mutex::new(());

fn one_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    KERNEL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
#[ignore = "requires admin"]
fn the_flush_timer_changes_and_the_kernel_flags_stay() {
    let _one = one_at_a_time();
    let name = "Uniproc-Etw-Test-Flush";
    stop(name);
    let session = Session::start(name, &Options::system_logger(DISK_IO | NETWORK_TCPIP));
    assert!(session.is_ok(), "{:?}", session.err());
    let session = session.unwrap();
    assert_eq!(session.query().map(|now| now.flush_timer_ms), Some(50));

    let set = session.set_flush_timer(1000);
    let after = session.query();
    drop(session);

    assert!(set.is_ok(), "{set:?}");
    assert_eq!(after.map(|now| now.flush_timer_ms), Some(1000));
    assert_eq!(after.map(|now| now.kernel_flags), Some(DISK_IO | NETWORK_TCPIP), "the update cleared the kernel flags");
}

#[test]
#[ignore = "requires admin"]
fn a_leftover_session_is_restarted_with_its_flags() {
    let _one = one_at_a_time();
    let name = "Uniproc-Etw-Test-Restart";
    let flags = DISK_IO | PROFILE | NETWORK_TCPIP;
    stop(name);
    let left = Session::start(name, &Options::system_logger(flags));
    assert!(left.is_ok(), "{:?}", left.err());
    std::mem::forget(left);

    let restarted = Session::start(name, &Options::system_logger(flags));
    assert!(restarted.is_ok(), "{:?}", restarted.err());
    let restarted = restarted.unwrap();
    assert_eq!(restarted.query().map(|now| now.kernel_flags), Some(flags), "the restarted session lost its kernel flags");
}

#[test]
#[ignore = "requires admin"]
fn a_running_session_is_listed_and_stopped_by_name() {
    let name = "Uniproc-Etw-Test-List";
    stop(name);
    let session = Session::start(name, &Options::real_time());
    assert!(session.is_ok(), "{:?}", session.err());
    let session = session.unwrap();

    assert!(running().iter().any(|running| running == name), "{:?}", running());
    assert!(stop(name));
    assert!(!running().iter().any(|running| running == name));
    assert!(!stop(name), "nothing left to stop");
    assert_eq!(session.query(), None);
}

const KERNEL_PROCESS: u128 = 0x22FB2CD6_0E7B_422B_A0C7_2FAD1FD0E716;
const PROCESS_KEYWORD: u64 = 0x10;

#[test]
#[ignore = "requires admin"]
fn a_manifest_event_tells_its_task_and_opcode_by_name() {
    let name = "Uniproc-Etw-Test-Names";
    stop(name);
    let session = Session::start(name, &Options::real_time());
    assert!(session.is_ok(), "{:?}", session.err());
    let session = session.unwrap();
    let enable = Enable {
        keywords: PROCESS_KEYWORD,
        level: 4,
    };
    let enabled = session.enable(KERNEL_PROCESS, enable);
    assert!(enabled.is_ok(), "{enabled:?}");

    let (sender, received) = mpsc::channel();
    let trace = Trace::real_time(name, Timestamps::SystemTime, move |event| {
        if event.provider() == KERNEL_PROCESS && event.id() == 1 {
            let _ = sender.send((
                event.number("ProcessID"),
                event.task_name().map(str::to_owned),
                event.opcode_name().map(str::to_owned),
            ));
        }
    });
    assert!(trace.is_ok(), "{:?}", trace.err());

    let child = std::process::Command::new("cmd.exe").args(["/c", "exit"]).spawn();
    assert!(child.is_ok(), "{:?}", child.err());
    let mut child = child.unwrap();
    let pid = u64::from(child.id());
    let _ = child.wait();

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut told = None;
    while let Ok(start) = received.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
        if start.0 == Some(pid) {
            told = Some(start);
            break;
        }
    }
    drop(trace);
    drop(session);
    let told = told.map(|(_, task, opcode)| (task, opcode));
    assert_eq!(told, Some((Some("ProcessStart".to_string()), Some("Start".to_string()))));
}

#[test]
#[ignore = "requires admin"]
fn a_real_time_trace_hands_over_events_until_its_session_stops() {
    unsafe { PROVIDER.register() };
    let provider = PROVIDER.id().to_u128();
    let name = "Uniproc-Etw-Test-RealTime";
    stop(name);
    let session = Session::start(name, &Options::real_time());
    assert!(session.is_ok(), "{:?}", session.err());
    let session = session.unwrap();
    let enabled = session.enable(provider, Enable::ALL);
    assert!(enabled.is_ok(), "{enabled:?}");

    let (sender, received) = mpsc::channel();
    let trace = Trace::real_time(name, Timestamps::SystemTime, move |event| {
        if event.provider() == provider {
            let _ = sender.send(event.number("Number"));
        }
    });
    assert!(trace.is_ok(), "{:?}", trace.err());
    let trace = trace.unwrap();

    tlg::write_event!(PROVIDER, "Written", u32("Number", &7));
    let told = received.recv_timeout(Duration::from_secs(5));
    assert_eq!(told, Ok(Some(7)));
    assert!(trace.pumping());

    drop(session);
    let stopped = Instant::now();
    while trace.pumping() && stopped.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(10));
    }
    PROVIDER.unregister();
    assert!(!trace.pumping(), "the trace went on after its session stopped");
}
