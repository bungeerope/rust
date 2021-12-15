#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Online,
    Offline,
    Unknown,
}

#[allow(dead_code)]
enum Week {
    Mon,
    Tues,
    Wed,
    Thur,
    Fri,
    Sat,
    Sun,
}

#[test]
fn test_enum() {
    let status = Status::Online;
    match status {
        Status::Online => println!("current value is online"),
        _ => {}
    }
}