use crate::models::NewSubmission;

pub fn construct_receipt_body(
    submission: &NewSubmission,
    track: &str,
    hash: &str,
) -> String {
    if track == "A" {
        format!(
            "Hello! Your submission for track A has been recorded successfully. \
                        \n\n<br> <br> Powersort Comparison: {} \
                        \n<br> Timsort Comparison: {} \
                        \n<br> Difference in comparisons (%): {} \
                        \n<br> Difference in merge costs (%): {} \
                        \n<br> Powersort Merge Cost: {} \
                        \n<br> Timsort Merge Cost: {} \
                        \n<br> Overall Score: {} \
                        \n<br> Submission Size: {} \
                         \n\n<br> <br> <br> <br> <br> {} ",
            submission.powersort_comp,
            submission.timsort_comp,
            submission.comp_diff,
            submission.mcost_diff,
            submission.powersort_merge_cost,
            submission.timsort_merge_cost,
            submission.combined_metric,
            submission.submission_size,
            hash
        )
    } else {
        // Track B.
        format!(
            "Hello! Your submission for track B has been recorded successfully. \
                        \n\n<br> <br> <br> <br> <br> {} \
            ",
            hash
        )
    }
}
