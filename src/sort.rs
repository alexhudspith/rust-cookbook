use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct KeyValue {
    key: &'static str,
    #[allow(dead_code)]
    value: f64,
}

impl PartialEq for KeyValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

// can't derive Eq since it compile-time checks that all fields are total Eq
impl Eq for KeyValue {}

// can't derive Hash since it compile-time checks that all fields are Hash
impl Hash for KeyValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

// mustn't derive PartialOrd: it uses all fields even if Ord doesn't
// Clippy (only) forbids this by default
impl PartialOrd for KeyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// mustn't derive Ord: it uses all fields even if PartialOrd doesn't
// Clippy (only) forbids this by default
impl Ord for KeyValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(other.key)
    }
}

const A1: KeyValue = KeyValue { key: "A", value: 1.0 };
const A2: KeyValue = KeyValue { key: "A", value: 2.0 };
const B2: KeyValue = KeyValue { key: "B", value: 2.0 };
const B0: KeyValue = KeyValue { key: "B", value: 0.0 };


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::error::Error;
    use std::hash::{Hash, Hasher};

    #[test]
    fn eq_deref() {
        assert_eq!(&A1, &A2);
    }

    #[test]
    fn eq_deref_mut() {
        let mut a1 = A1.clone();
        let mut a2 = A2.clone();
        assert_eq!(&mut a1, &mut a2);
    }

    #[test]
    fn eq_no_deref_ptr() {
        assert_ne!(&A1 as *const _, &A2 as *const _);
    }

    #[test]
    fn eq_no_deref_ptr_mut() {
        let mut a1 = A1.clone();
        let mut a2 = A2.clone();
        assert_ne!(&mut a1 as *mut _, &mut a2 as *mut _);
    }

    #[test]
    fn partial_cmp() {
        assert_eq!(A1.partial_cmp(&A2), Some(Ordering::Equal));
        assert_eq!(A1.partial_cmp(&B0), Some(Ordering::Less));
        assert_eq!(B2.partial_cmp(&A2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp() {
        assert_eq!(A1.cmp(&A2), Ordering::Equal);
        assert_eq!(A1.cmp(&B0), Ordering::Less);
        assert_eq!(B2.cmp(&A2), Ordering::Greater);
    }

    #[test]
    fn eq() {
        assert!(A1.eq(&A2));
        assert!(A2.ne(&B2));
    }

    fn do_hash<T: Hash>(v: T) -> u64 {
        let mut h = DefaultHasher::default();
        v.hash(&mut h);
        h.finish()
    }

    #[test]
    fn hash() {
        assert_eq!(do_hash(A1), do_hash(A2));
    }

    #[test]
    fn sort_stable() {
        let mut a = [A2, B2, A1, B0];
        // Timsort
        a.sort();
        assert_eq!(a, [A2, A1, B2, B0]);
    }

    #[test]
    fn sort_unstable() -> Result<(), Box<dyn Error>> {
        let mut a = [A2, B2, A1, B0];
        // Pattern-defeating quicksort
        a.sort_unstable();
        let x: &[KeyValue; 2] = (&a[..2]).try_into()?;
        let y: &[KeyValue; 2] = (&a[2..]).try_into()?;
        assert!([[A1, A2], [A2, A1]].contains(x));
        assert!([[B0, B2], [B2, B0]].contains(y));
        Ok(())
    }

    #[test]
    fn sort_floats_total() {
        let mut a = [5.3_f64, 2.6, 0.0, f64::NEG_INFINITY, f64::INFINITY, -2.4e32, f64::NAN, -0.0, -f64::NAN];
        a.sort_by(f64::total_cmp);

        let last = a.len() - 1;
        assert!(a[0].is_nan() && a[0].is_sign_negative());
        assert!(a[last].is_nan() && a[last].is_sign_positive());
        assert_eq!(&a[1..last], [-f64::INFINITY, -2.4e32, -0.0, 0.0, 2.6, 5.3_f64, f64::INFINITY]);
    }
}
