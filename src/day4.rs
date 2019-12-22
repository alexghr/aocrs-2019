use tramp::{tramp, Rec};

pub fn brute_force(lo: u32, hi: u32) -> u32 {
    fn recurse(n: u32, hi: u32, cnt: u32) -> Rec<u32> {
        if n > hi {
            rec_ret!(cnt)
        }

        if rule_duplicate_digit(n) && rule_increasing_digits(n) {
            rec_call!(recurse(n + 1, hi, cnt + 1))
        } else {
            rec_call!(recurse(n + 1, hi, cnt))
        }
    }

    tramp(recurse(lo, hi, 0))
}

fn rule_duplicate_digit(n: u32) -> bool {
    fn rec_rule(n: u32) -> Rec<bool> {
        if n < 10 {
            rec_ret!(false)
        }

        let digit_a = n % 10;
        let digit_b = (n / 10) % 10;
        if digit_a == digit_b {
            rec_ret!(true)
        }

        rec_call!(rec_rule(n / 10))
    }

    tramp(rec_rule(n))
}

fn rule_increasing_digits(n: u32) -> bool {
    fn rec_rule(n: u32) -> Rec<bool> {
        if n < 10 {
            rec_ret!(true)
        }

        let digit_hi = n % 10;
        let digit_lo = (n / 10) % 10;

        if digit_lo > digit_hi {
            rec_ret!(false)
        }

        rec_call!(rec_rule(n / 10))
    }

    tramp(rec_rule(n))
}
