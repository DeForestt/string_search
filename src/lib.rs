use std::cmp::max;

fn lcs_collected(a: &str, b: &str, n: usize, m: usize, dp: &mut Vec<Vec<i32>>) -> Vec<char> {
    let mut i = n;
    let mut j = m;
    let mut result = Vec::new();
    while i > 0 && j > 0 {
        if a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 1).unwrap() {
            result.push(a.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.reverse();
    result
}

fn create_dp(a: &str, b: &str) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp
}

fn normalized_len_compare(subject: &str, sequence: &str) -> f64 {
    let min_window = sequence.len();
    let mut window = min_window;

    if min_window > subject.len() {
        return 0.0;
    }
    if min_window == 0 {
        return 0.0;
    }

    while window <= subject.len() {
        let mut i = 0;
        while i + window <= subject.len() {
            let sub = &subject[i..i + window];
            let n = sub.len();
            let m = sequence.len();
            let mut dp = create_dp(sub, sequence);
            let lcs = lcs_collected(sub, sequence, n, m, &mut dp);
            if lcs.len() == sequence.len() {
                return min_window as f64 / window as f64;
            }
            i += 1;
        }
        window += 1;
    }
    return 0.0;
}

pub fn compare(str1: &str, q: &str) -> f64 {
    let len1 = str1.len();
    let len2 = q.len();
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let mut dp = create_dp(str1, q);
    let lcs = lcs_collected(str1, q, len1, len2, &mut dp).
        iter().collect::<String>();
        
    if lcs.len() <= 1 {
        return 0.0;
    }

    lcs.len() as f64 * (normalized_len_compare(str1, &lcs))
}

pub fn rank<'a>(query: &str, subjects: Vec<&'a str>) -> Vec<(f64, &'a str)> {
    let mut result = subjects
        .iter()
        .map(|&subject| (compare(subject, query), subject))
        .collect::<Vec<(f64, &str)>>();
    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    result
}

pub fn struct_rank<'a, T: Clone>(
    query: &str,
    subjects: Vec<T>,
    accessor: fn(&T) -> &str,
) -> Vec<(f64, T)> {
    let mut result = subjects
        .iter()
        .map(|subject| (compare(accessor(subject), query), subject.clone()))
        .collect::<Vec<(f64, T)>>();
    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    result
}

mod tests {
    use super::*;

    #[test]
    fn test_lcs_collected() {
        let a = "AGGTAB";
        let b = "GXTXAYB";
        let n = a.len();
        let m = b.len();
        let mut dp = create_dp(a, b);
        let result = lcs_collected(a, b, n, m, &mut dp);
        assert_eq!(result, vec!['G', 'T', 'A', 'B']);
    }

    #[test]
    fn test_lcs_collected_with_no_match() {
        let a = "AGGTAB";
        let b = "XYZ";
        let n = a.len();
        let m = b.len();
        let mut dp = create_dp(a, b);
        let result = lcs_collected(a, b, n, m, &mut dp);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_create_dp() {
        let a = "AGGTAB";
        let b = "GXTXAYB";
        let dp = create_dp(a, b);
        assert_eq!(dp[6][7], 4);
    }

    #[test]
    fn test_normalized_len_compare_with_perfect_match() {
        let subject = "AGGTAB";
        let sequence = "AGGTAB";
        let result = normalized_len_compare(subject, sequence);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_normalized_len_compare_with_no_match() {
        let subject = "AGGTAB";
        let sequence = "GXT";
        let result = normalized_len_compare(subject, sequence);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_normalized_len_compare_with_partial_match() {
        let subject = "AGGTAB";
        let sequence = "GT";
        let result = normalized_len_compare(subject, sequence);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_normalized_len_compare_with_spread_out_match() {
        let subject = "AGGTAB";
        let sequence = "GA";
        let result = normalized_len_compare(subject, sequence);
        assert_eq!(result, (2.0 / 3.0));
    }

    #[test]
    fn test_compare_with_a_perfect_match() {
        let str1 = "AGGTAB";
        let q = "AGGTAB";
        let result = compare(str1, q);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_compare_with_partial_match() {
        let str1 = "Ad_Fields";
        let q = "Ad";
        let result = compare(str1, q);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_compare_with_no_match() {
        let str1 = "AGGTAB";
        let q = "XYZ";
        let result = compare(str1, q);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_compare_with_spread_out_match() {
        let str1 = "AGGTAB";
        let q = "GA";
        let result = compare(str1, q);
        assert_eq!(result, 2.0 / 3.0 * 2.0);
    }

    #[test]
    fn test_rank() {
        let query = "Ad";
        let subjects = vec!["Ad_Fields", "Users", "Aged_groups"];
        let result = rank(query, subjects);
        assert_eq!(
            result,
            vec![(2.0, "Ad_Fields"), (1.0, "Aged_groups"), (0.0, "Users"),]
        );
    }
}
