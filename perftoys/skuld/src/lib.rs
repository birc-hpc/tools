pub fn count_bytes(x: &[u8], counts: &mut [u32; 256]) {
    for &a in x {
        counts[a as usize] += 1;
    }
}

pub fn fit_counts_to_termwidth(counts: &mut [u32; 256], width: u32) {
    let widest = match counts.iter().max() {
        None => 1,
        Some(m) => *m,
    };
    for i in 0..256 {
        counts[i] *= width;
        counts[i] /= widest;
    }
}

// FIXME: Make this pretty
pub fn print_counts(counts: &[u32; 256]) {
    for i in 0..256 {
        if counts[i] > 0 {
            println!("{:>3}: {}", i, "#".repeat(counts[i] as usize));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        let x = "foobar";
        let mut counts = [0u32; 256];
        count_bytes(x.as_bytes(), &mut counts);
        assert_eq!(counts['f' as usize], 1);
        assert_eq!(counts['o' as usize], 2);
        assert_eq!(counts['b' as usize], 1);
        assert_eq!(counts['a' as usize], 1);
        assert_eq!(counts['r' as usize], 1);
    }

    #[test]
    fn test_fit_counts_to_termwidth() {
        let x = "foobar";
        let mut counts = [0u32; 256];
        count_bytes(x.as_bytes(), &mut counts);

        fit_counts_to_termwidth(&mut counts, 4);
        assert_eq!(counts['f' as usize], 2);
        assert_eq!(counts['o' as usize], 4);
        assert_eq!(counts['b' as usize], 2);
        assert_eq!(counts['a' as usize], 2);
        assert_eq!(counts['r' as usize], 2);
    }
}
