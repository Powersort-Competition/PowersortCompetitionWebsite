use backend::utils::construct_receipt_body;
use backend::models::NewSubmission;

#[test]
fn test_construct_receipt_body_track_a() {
    let submission = NewSubmission {
        user_id: 1,
        powersort_comp: 100,
        timsort_comp: 110,
        comp_diff: 10.0,
        mcost_diff: 5.0,
        powersort_merge_cost: 500,
        timsort_merge_cost: 550,
        combined_metric: 15.0,
        submission_size: 1000,
    };

    let hash = "abc123hash";
    let body = construct_receipt_body(&submission, "A", hash);

    assert!(body.contains("Powersort Comparison: 100"));
    assert!(body.contains("Timsort Comparison: 110"));
    assert!(body.contains(hash));
}

#[test]
fn test_construct_receipt_body_track_b() {
    let submission = NewSubmission {
        user_id: 1,
        powersort_comp: 0,
        timsort_comp: 0,
        comp_diff: 0.0,
        mcost_diff: 0.0,
        powersort_merge_cost: 0,
        timsort_merge_cost: 0,
        combined_metric: 0.0,
        submission_size: 0,
    };

    let hash = "xyz789hash";
    let body = construct_receipt_body(&submission, "B", hash);

    assert!(body.contains("track B"));
    assert!(body.contains(hash));
    assert!(!body.contains("Powersort Comparison")); // Should not contain Track A specific info
}
