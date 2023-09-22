use std::fs;

use activity_vocabulary::vocab::Travel;

#[test]
fn example4() {
    let json = fs::read_to_string("tests/example4.json").unwrap();
    let activity: Travel = serde_json::from_str(&json).unwrap();
    dbg!(activity);
}
