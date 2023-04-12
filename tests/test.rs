use run::code_file;

#[test]
fn it_works() {
    assert!(2 == 2);
#[test]
fn file_name_works() {
    let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()]);
    assert_eq!(file.name, String::from("main.py"));
}

#[test]
fn file_ending_works() {
    let file = Codefile::new(vec!["run".to_string(), "main.foo.bar.c".to_string()]);
    assert_eq!(file.ending, String::from("c"));
}

#[test]
fn command_works_cross_platform() {
    let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()]);
    match std::env::consts::OS {
        "linux" | "macos" => assert_eq!(file.command, vec![
            String::from("python3"),
            String::from("main.py")
        ]),
        "windows" => assert_eq!(file.command, vec![
            String::from("python"),
            String::from("main.py")
        ]),
        _ => panic!("Invalid OS detected"),
    }
}

#[test]
fn command_works_with_args() {
    let file = Codefile::new(vec![
        "run".to_string(), 
        "main.c".to_string(), 
        "-r".to_string(), 
        "-foo".to_string()
    ]);
    assert_eq!(file.command, vec![
        "gcc".to_string(),
        "main.c".to_string(),
        "-r".to_string(),
        "-foo".to_string(),
    ])
}

#[test]
#[should_panic]
fn panics_with_bad_args() {
    let _file = Codefile::new(Vec::new());
}
}
