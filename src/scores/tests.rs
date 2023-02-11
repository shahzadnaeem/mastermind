use crate::scores;

#[test]
fn test_crate_root() {}

#[test]
fn create_winning_scored() {
    let scorer = scores::Scored::new("Input", "Input");

    assert_eq!(scorer.valid, true);
    assert_eq!(scorer.done, true);
}
