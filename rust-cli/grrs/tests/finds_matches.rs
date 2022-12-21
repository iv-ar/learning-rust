use std::fs::File;

#[test]
fn finds_matches() {
    let mut test_file = File::create("finds_matches.txt").unwrap();
    std::io::Write::write_all(&mut test_file, b"asdf\ntesting").unwrap();
    std::io::Write::flush(&mut test_file).unwrap();
    let file_to_test = File::open("finds_matches.txt").expect("could not read test file");
    let matches = grrs::find_matches(&file_to_test, "asdf").unwrap();
    assert_eq!(matches.len(), 1);
    std::fs::remove_file("finds_matches.txt").unwrap();
}