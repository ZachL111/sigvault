use sigvault::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 66, capacity: 76, latency: 23, risk: 16, weight: 5 };
    assert_eq!(score(signal), 57);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 66, capacity: 74, latency: 23, risk: 6, weight: 5 };
    assert_eq!(score(signal), 125);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 84, capacity: 77, latency: 14, risk: 21, weight: 7 };
    assert_eq!(score(signal), 98);
    assert_eq!(classify(signal), "review");
}
