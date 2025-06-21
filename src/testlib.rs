#[cfg(test)]
use super::*;
#[test]
fn test_index() {
    index();
    get_file_structure(&"/media".to_string(), &"".to_string());
}
