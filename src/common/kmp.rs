///
/// kmp
fn kmp(s: &str, m: &str) -> i32 {
    let m = m.chars().collect::<Vec<char>>();
    let s = s.chars().collect::<Vec<char>>();
    let mut m0 = 0;
    let mut s0 = 0;
    let next = get_next(&m);
    while s0 < s.len() && m0 < m.len() {
        if m[m0] == s[s0] {
            m0 += 1;
            s0 += 1;
        } else if m0 == 0 {
            s0 += 1;
        } else {
            m0 = next[m0] as usize;
        }
    }
    if m0 == m.len() { (s0 - m0) as i32 } else { -1 }
}

fn get_next(m: &[char]) -> Vec<i32> {
    if m.len() == 1 { return vec![-1]; }
    let mut next = vec![0i32; m.len()];
    next[0] = -1;
    next[1] = 0;
    let mut cn = 0;
    let mut i = 2;
    while i < m.len() {
        if m[i] == m[cn] {
            next[i] = cn as i32 + 1;
            i += 1;
            cn += 1;
        } else if cn == 0 {
            next[i] = 0;
            i += 1;
        } else {
            cn = next[cn] as usize;
        }
    }
    next
}

#[test]
fn f1() {
    assert_eq!(kmp("dbcabc", "abc"), 3);
    assert_eq!(kmp("abc11bcd", "cd"), 6);
}