use sigvault::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 50, slack: 54, drag: 30, confidence: 89 };
    assert_eq!(review_score(case), 153);
    assert_eq!(review_lane(case), "ship");
}
