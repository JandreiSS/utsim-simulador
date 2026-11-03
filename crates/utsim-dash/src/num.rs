//! Small numeric helpers of the dashboard: quantiles, the Wilson interval of a
//! proportion, histogram quantiles and the Brazilian-Portuguese number text.

use utsim_core::stats::Histogram;
use utsim_stats::special::norm_ppf;

/// Quantile `q` in `[0, 1]` of `xs` (linear interpolation between order statistics, the
/// "type 7" definition of the usual statistics packages). NaN for an empty slice.
pub fn quantile(xs: &[f64], q: f64) -> f64 {
    let mut v: Vec<f64> = xs.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    let pos = q.clamp(0.0, 1.0) * (v.len() - 1) as f64;
    let (lo, hi) = (pos.floor() as usize, pos.ceil() as usize);
    v[lo] + (v[hi] - v[lo]) * (pos - lo as f64)
}

/// Wilson score interval of a proportion `k / n` at confidence `level`. Unlike the
/// plain normal interval it stays inside `[0, 1]` and behaves with few trials, which is
/// the usual situation (a dozen replications). `(NaN, NaN)` for `n = 0`.
pub fn wilson(k: usize, n: usize, level: f64) -> (f64, f64) {
    if n == 0 {
        return (f64::NAN, f64::NAN);
    }
    let z = norm_ppf(1.0 - (1.0 - level) / 2.0);
    let n = n as f64;
    let p = k as f64 / n;
    let z2 = z * z;
    let centre = (p + z2 / (2.0 * n)) / (1.0 + z2 / n);
    let half = z * (p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt() / (1.0 + z2 / n);
    ((centre - half).max(0.0), (centre + half).min(1.0))
}

/// Quantile `q` of the distribution described by several histograms taken together
/// (each bin spreads its count uniformly over its width). `None` without observations.
pub fn histogram_quantile(hists: &[&Histogram], q: f64) -> Option<f64> {
    let mut segs: Vec<(f64, f64, f64)> = Vec::new();
    for h in hists {
        for (k, &c) in h.counts.iter().enumerate() {
            if c > 0 {
                let (a, b) = h.edges(k);
                segs.push((a, b.max(a), c as f64));
            }
        }
    }
    let total: f64 = segs.iter().map(|s| s.2).sum();
    if total <= 0.0 {
        return None;
    }
    segs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let target = q.clamp(0.0, 1.0) * total;
    let mut acc = 0.0;
    for (a, b, c) in &segs {
        if acc + c >= target {
            let frac = if *c > 0.0 { (target - acc) / c } else { 0.0 };
            return Some(a + (b - a) * frac);
        }
        acc += c;
    }
    segs.last().map(|s| s.1)
}

/// Fraction of the observations of the histograms that are `<= x`.
pub fn histogram_cdf(hists: &[&Histogram], x: f64) -> Option<f64> {
    let (mut below, mut total) = (0.0, 0.0);
    for h in hists {
        for (k, &c) in h.counts.iter().enumerate() {
            let (a, b) = h.edges(k);
            let c = c as f64;
            total += c;
            below += if x >= b {
                c
            } else if x <= a || b <= a {
                if b <= a && x >= a { c } else { 0.0 }
            } else {
                c * (x - a) / (b - a)
            };
        }
    }
    (total > 0.0).then(|| below / total)
}

/// Decimal places that match the precision of an estimate: none when the half-width is
/// 3.1, one when it is 0.31... (`118 ± 3`, not `118,4372 ± 3,1`). Without a half-width,
/// three significant digits of the value.
pub fn decimals(value: f64, half_width: f64) -> usize {
    let basis = if half_width.is_finite() && half_width > 0.0 {
        -half_width.log10().floor()
    } else if value.is_finite() && value != 0.0 {
        2.0 - value.abs().log10().floor()
    } else {
        0.0
    };
    basis.clamp(0.0, 6.0) as usize
}

/// A number in Brazilian Portuguese (comma decimal separator); `-` for NaN.
pub fn pt(x: f64, decimals: usize) -> String {
    if !x.is_finite() {
        return "-".to_string();
    }
    let s = format!("{x:.decimals$}");
    // A tiny negative rounded to zero would print "-0".
    let s = if s
        .trim_start_matches('-')
        .chars()
        .all(|c| c == '0' || c == '.')
    {
        s.trim_start_matches('-').to_string()
    } else {
        s
    };
    s.replace('.', ",")
}

/// `mean ± half-width` with the decimals of [`decimals`]; just the mean without an interval.
pub fn pt_mean_hw(mean: f64, half_width: f64) -> String {
    let d = decimals(mean, half_width);
    if half_width.is_finite() {
        format!("{} ± {}", pt(mean, d), pt(half_width, d))
    } else {
        pt(mean, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantiles_interpolate_like_type_7() {
        let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(quantile(&xs, 0.0), 1.0);
        assert_eq!(quantile(&xs, 0.5), 3.0);
        assert!((quantile(&xs, 0.9) - 4.6).abs() < 1e-12);
        assert_eq!(quantile(&xs, 1.0), 5.0);
        assert!(quantile(&[], 0.5).is_nan());
        assert_eq!(quantile(&[7.0], 0.95), 7.0);
    }

    #[test]
    fn wilson_matches_published_values() {
        // 8 of 10 at 95%: the textbook interval is (0.4902, 0.9433).
        let (lo, hi) = wilson(8, 10, 0.95);
        assert!(
            (lo - 0.4902).abs() < 5e-4 && (hi - 0.9433).abs() < 5e-4,
            "{lo} {hi}"
        );
        // All successes: the upper end is 1 and the lower end stays below 1.
        let (lo, hi) = wilson(10, 10, 0.95);
        assert!(
            (hi - 1.0).abs() < 1e-12 && (lo - 0.7225).abs() < 5e-4,
            "{lo} {hi}"
        );
        let (lo, hi) = wilson(0, 10, 0.95);
        assert!(lo.abs() < 1e-12 && (hi - 0.2775).abs() < 5e-4);
        assert!(wilson(0, 0, 0.95).0.is_nan());
    }

    #[test]
    fn histogram_quantile_and_cdf() {
        let h = Histogram {
            min: 0.0,
            max: 4.0,
            bin_width: 1.0,
            counts: vec![10, 10, 10, 10],
        };
        assert!((histogram_quantile(&[&h], 0.5).unwrap() - 2.0).abs() < 1e-12);
        assert!((histogram_quantile(&[&h], 0.9).unwrap() - 3.6).abs() < 1e-12);
        assert!((histogram_cdf(&[&h], 1.5).unwrap() - 0.375).abs() < 1e-12);
        assert_eq!(histogram_cdf(&[&h], 99.0), Some(1.0));
        assert!(histogram_quantile(&[], 0.5).is_none());
    }

    #[test]
    fn numbers_follow_the_precision() {
        assert_eq!(pt_mean_hw(118.4372, 3.1), "118 ± 3");
        assert_eq!(pt_mean_hw(118.4372, 0.31), "118,4 ± 0,3");
        assert_eq!(pt_mean_hw(0.8123, f64::NAN), "0,812");
        assert_eq!(pt(-0.0001, 2), "0,00");
        assert_eq!(pt(f64::NAN, 2), "-");
    }
}
