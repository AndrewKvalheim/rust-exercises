use two_bucket::{solve, Bucket, BucketStats};

#[test]
fn test_case_1() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::One),
        BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        }
    );
}

#[test]
fn test_case_2() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::Two),
        BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        }
    );
}

#[test]
fn test_case_3() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::One),
        BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        }
    );
}

#[test]
fn test_case_4() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::Two),
        BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        }
    );
}

#[test]
fn goal_equal_to_start_bucket() {
    assert_eq!(
        solve(1, 3, 3, &Bucket::Two),
        BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        }
    );
}

#[test]
fn goal_equal_to_other_bucket() {
    assert_eq!(
        solve(2, 3, 3, &Bucket::One),
        BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        }
    );
}
