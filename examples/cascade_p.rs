/// Caculate shadow map cascade updating probability
fn main() {
    // exe cnt p0 p1 p2 p3

    let mut args = std::env::args().skip(1);
    let c = args.next().unwrap().parse().unwrap();

    let mut p = [0.0; 4];
    for pi in p.iter_mut() {
        *pi = args.next().unwrap().parse().unwrap();
    }

    let r = probability(c, &p);
    println!("{:?}", r);
}

fn probability(n: u32, p: &[f64; 4]) -> Vec<f64> {
    let base = p.iter().map(|x| x * 100.0).collect::<Vec<_>>();
    let mut pool = base.clone();
    let mut count = [0; 4];

    for i in 1..=n {
        let ((a, ai), (b, bi)) = two_largest(&pool, i % 2 != 0);
        if pool[ai as usize] < 1.0 || pool[ai as usize] < 1.0 {
            for (i, x) in pool.iter_mut().enumerate() {
                *x += base[i];
            }
        }

        pool[ai as usize] -= 1.0;
        pool[bi as usize] -= 1.0;
        count[ai as usize] += 1;
        count[bi as usize] += 1;
    }

    let t = (2 * n) as f64;
    count.iter().map(|c| (*c as f64) / t).collect::<Vec<_>>()
}

fn two_largest<T: Copy + PartialOrd + Default>(arr: &[T], reverse: bool) -> ((T, i32), (T, i32)) {
    let mut max_a = <T as Default>::default();
    let mut max_a_idx = 0;

    let mut max_b = <T as Default>::default();
    let mut max_b_idx = 0;

    let mut cmp = |i, v| {
        if v > max_a {
            max_b = max_a;
            max_b_idx = max_a_idx;

            max_a = v;
            max_a_idx = i;
        } else if v > max_b {
            max_b = v;
            max_b_idx = i;
        }
    };

    if reverse {
        let mut i = arr.len() as i32 - 1;
        for v in arr.iter().rev() {
            cmp(i, *v);
            i -= 1;
        }
    } else {
        let mut i = 0;
        for v in arr.iter() {
            cmp(i, *v);
            i += 1;
        }
    }

    ((max_a, max_a_idx), (max_b, max_b_idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        let arr = [5, 2, 1, 2];
        assert_eq!(two_largest(&arr, false), ((5, 0), (2, 1)));

        let arr = [5, 2, 1, 2];
        assert_eq!(two_largest(&arr, true), ((5, 0), (2, 3)));

        let arr = [5, 1, 2, 2];
        assert_eq!(two_largest(&arr, false), ((5, 0), (2, 2)));

        let arr = [5, 1, 2, 2];
        assert_eq!(two_largest(&arr, true), ((5, 0), (2, 3)));

        let arr = [4, 1, 1, 2];
        assert_eq!(two_largest(&arr, false), ((4, 0), (2, 3)));
    }
}
