use super::*;

use std::thread;
use std::time::Duration;

#[test]
fn with_small_integer_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| 0.into_process(process));
}

#[test]
fn with_float_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| 1.0.into_process(process));
}

#[test]
fn with_big_integer_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        (integer::small::MAX + 1).into_process(process)
    });
}

#[test]
fn with_local_reference_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        Term::next_local_reference(process)
    });
}

#[test]
fn with_external_pid_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        Term::external_pid(1, 0, 0, process).unwrap()
    });
}

#[test]
fn with_tuple_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        Term::slice_to_tuple(&[], process)
    });
}

#[test]
fn with_map_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        Term::slice_to_map(&[], process)
    });
}

#[test]
fn with_empty_list_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| list_term(process));
}

#[test]
fn with_heap_binary_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| {
        Term::slice_to_binary(&[1], process)
    });
}

#[test]
fn with_subbinary_message_sends_timeout_message_when_timer_expires() {
    with_message_sends_timeout_message_when_timer_expires(|process| bitstring!(1 :: 1, process));
}

fn with_message_sends_timeout_message_when_timer_expires<M>(message: M)
where
    M: FnOnce(&Process) -> Term,
{
    with_process_arc(|process_arc| {
        let destination = registered_name();

        assert_eq!(
            erlang::register_2(destination, process_arc.pid, process_arc.clone()),
            Ok(true.into())
        );

        let milliseconds = milliseconds();
        let time = milliseconds.into_process(&process_arc);
        let message = message(&process_arc);
        let options = options(&process_arc);

        let result =
            erlang::start_timer_4(time, destination, message, options, process_arc.clone());

        assert!(
            result.is_ok(),
            "Timer reference not returned.  Got {:?}",
            result
        );

        let timer_reference = result.unwrap();

        assert_eq!(timer_reference.tag(), Boxed);

        let unboxed_timer_reference: &Term = timer_reference.unbox_reference();

        assert_eq!(unboxed_timer_reference.tag(), LocalReference);

        let timeout_message = timeout_message(timer_reference, message, &process_arc);

        assert!(!has_message(&process_arc, timeout_message));

        thread::sleep(Duration::from_millis(milliseconds + 1));

        timer::timeout();

        assert!(has_message(&process_arc, timeout_message));
    })
}