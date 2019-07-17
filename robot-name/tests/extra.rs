use robot_name::Robot;
use std::collections::HashSet;
use std::panic;

const NAMESPACE_LENGTH: usize = 26 * 26 * 10 * 10 * 10;

#[test]
fn all_names_valid_unique_and_exhausted() {
    let robots = (1..=NAMESPACE_LENGTH)
        .map(|_| Robot::new())
        .collect::<Vec<_>>();

    // Name validity and uniqueness
    let mut names = HashSet::with_capacity(NAMESPACE_LENGTH);
    robots.iter().for_each(|robot| {
        let mut chars = robot.name().chars();
        assert!(chars.next().unwrap().is_ascii_uppercase());
        assert!(chars.next().unwrap().is_ascii_uppercase());
        assert!(chars.next().unwrap().is_ascii_digit());
        assert!(chars.next().unwrap().is_ascii_digit());
        assert!(chars.next().unwrap().is_ascii_digit());
        assert!(chars.next().is_none());

        assert!(names.insert(robot.name().to_owned()));
    });

    // Namespace exhaustion
    assert!(panic::catch_unwind(Robot::new).is_err());
}
