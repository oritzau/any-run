use run::code_file::*;

#[test]
fn file_name_works() {
    let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()], 1);
    assert_eq!(file.unwrap().name, String::from("main.py"));
}

#[test]
fn file_ending_works() {
    let file = Codefile::new(vec!["run".to_string(), "main.foo.bar.c".to_string()], 1);
    assert_eq!(file.unwrap().ending, String::from("c"));
}

#[test]
fn command_works_cross_platform() {
    let file = Codefile::new(vec!["run".to_string(), "main.py".to_string()], 1);
    match std::env::consts::OS {
        "linux" | "macos" => assert_eq!(file.unwrap().command, vec![
            String::from("python3"),
            String::from("main.py")
        ]),
        "windows" => assert_eq!(file.unwrap().command, vec![
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
    ], 1);
    assert_eq!(file.unwrap().command, vec![
        "gcc".to_string(),
        "main.c".to_string(),
        "-r".to_string(),
        "-foo".to_string(),
    ])
}

#[test]
fn file_renamed_with_arg() {
    let vec = vec![
        "run".to_string(),
        "-o".to_string(),
        "foobar".to_string(),
        "main.c".to_string(),
    ];
    assert_eq!(get_filename_index(&vec), 3);
    assert_eq!(Codefile::new(vec, 3).unwrap().target_name, "foobar".to_string());
}

#[test]
#[should_panic]
fn panics_with_bad_args() {
    let _file = Codefile::new(Vec::new(), 0).unwrap();
}

