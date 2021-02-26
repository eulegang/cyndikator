use super::Dispatch;
use crate::Action;
use crate::Event;

const TEST_TEXT: &str = "
# we like rust
\"rust\" in categories and feed_title is \"Lobsters\" {
  notify
  record
}

title matches /Crust of Rust/ {
    notify
    record
}

\"osdev\" in categories {
  record
}

";

#[test]
fn parse() {
    Dispatch::parse(TEST_TEXT).unwrap();
}

#[test]
fn dispatch() {
    let dispatch = Dispatch::parse("'rust' in categories { record }").unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("foobar".to_string()),
        categories: vec!["rust".to_string()],
        feed_title: Some("foobar".to_string()),
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![Action::Record]);

    let actions = dispatch.dispatch(&Event {
        title: Some("foobar".to_string()),
        categories: vec![],
        feed_title: Some("foobar".to_string()),
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![]);

    let dispatch = Dispatch::parse("title matches /rust/i { notify }").unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("Curst of Rust".to_string()),
        categories: vec![],
        feed_title: None,
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![Action::Notify]);
}

#[test]
fn action_deduplication() {
    let dispatch = Dispatch::parse("title matches /rust/i { notify record notify }").unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("Curst of Rust".to_string()),
        categories: vec![],
        feed_title: None,
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![Action::Notify, Action::Record]);

    let dispatch = Dispatch::parse(
        "title matches /rust/i { exec \"echo '${title}'\" exec \"echo '${feed_title}'\" }",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec![],
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
    });

    assert_eq!(
        actions,
        vec![
            Action::Exec("echo 'Crust of Rust: Subtyping and Variance'".to_string()),
            Action::Exec("echo 'Youtube'".to_string())
        ]
    );
}

#[test]
fn multiaction() {
    let dispatch = Dispatch::parse(
        "
title matches /rust/i { 
    record
}

'rust' in categories { 
    notify
}

",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec!["rust".to_string()],
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![Action::Record, Action::Notify,]);
}

#[test]
fn drop() {
    let dispatch = Dispatch::parse(
        "
title matches /rust/i { 
    record
    drop
}

'rust' in categories { 
    notify
}

",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec!["rust".to_string()],
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
    });

    assert_eq!(actions, vec![Action::Record]);
}
