use backend::models::{NewUser, Submission, SubmissionView, CompositionTrackA, NewSubmission};
use serde_json;

#[test]
fn test_new_user_serialization() {
    let user = NewUser {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("John"));
    assert!(json.contains("Doe"));
    assert!(json.contains("john.doe@example.com"));
}

#[test]
fn test_new_user_deserialization() {
    let json = r#"{"first_name": "Jane", "last_name": "Doe", "email": "jane.doe@example.com"}"#;
    let user: NewUser = serde_json::from_str(json).unwrap();

    assert_eq!(user.first_name, "Jane");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.email, "jane.doe@example.com");
}

#[test]
fn test_submission_serialization() {
    let submission = Submission {
        submission_id: 1,
        user_id: 2,
        powersort_comp: 100,
        timsort_comp: 110,
        comp_diff: 10.0,
        mcost_diff: 5.0,
        powersort_merge_cost: 500,
        timsort_merge_cost: 550,
        combined_metric: 15.0,
        submission_size: 1000,
    };

    let json = serde_json::to_string(&submission).unwrap();
    assert!(json.contains("\"submission_id\":1"));
    assert!(json.contains("\"powersort_comp\":100"));
}

#[test]
fn test_submission_deserialization() {
    let json = r#"{
        "submission_id": 1,
        "user_id": 2,
        "powersort_comp": 100,
        "timsort_comp": 110,
        "comp_diff": 10.0,
        "mcost_diff": 5.0,
        "powersort_merge_cost": 500,
        "timsort_merge_cost": 550,
        "combined_metric": 15.0,
        "submission_size": 1000
    }"#;

    let submission: Submission = serde_json::from_str(json).unwrap();
    assert_eq!(submission.submission_id, 1);
    assert_eq!(submission.powersort_comp, 100);
}

#[test]
fn test_submission_view_serialization() {
    let view = SubmissionView {
        submission_id: 1,
        submitter: "John D.".to_string(),
        user_id: 2,
        powersort_comp: 100,
        timsort_comp: 110,
        comp_diff: 10.0,
        mcost_diff: 5.0,
        powersort_merge_cost: 500,
        timsort_merge_cost: 550,
        combined_metric: 15.0,
        submission_size: 1000,
    };

    let json = serde_json::to_string(&view).unwrap();
    assert!(json.contains("\"submitter\":\"John D.\""));
}

#[test]
fn test_composition_track_a_serialization() {
    let comp = CompositionTrackA {
        flyweight: 10,
        mediumweight: 20,
        heavyweight: 30,
    };

    let json = serde_json::to_string(&comp).unwrap();
    assert!(json.contains("\"flyweight\":10"));
    assert!(json.contains("\"mediumweight\":20"));
    assert!(json.contains("\"heavyweight\":30"));
}

#[test]
fn test_new_submission_serialization() {
    let sub = NewSubmission {
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

    let json = serde_json::to_string(&sub).unwrap();
    assert!(json.contains("\"user_id\":1"));
}
