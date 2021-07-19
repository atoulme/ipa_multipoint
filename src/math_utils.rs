use bandersnatch::Fr;

/// Computes the inner product between two scalar vectors
pub fn inner_product(a: &[Fr], b: &[Fr]) -> Fr {
    a.iter().zip(b.iter()).map(|(a, b)| *a * *b).sum()
}

// Creates a vector from the scalar `x`
// contents of vector = <x, x^2, x^3,.., x^n>
// XXX: double check that it is fine to use a vandermonde matrix in regards to testing distributions to
// expand challenges instead of fetching each challenge from the distribution
// so we don't need `n` different challenges
pub fn vandemonde_challenge(mut x: Fr, n: usize) -> Vec<Fr> {
    let mut challenges: Vec<Fr> = Vec::with_capacity(n);
    challenges.push(x);
    for i in 0..n - 1 {
        challenges.push(challenges[i] * x);
    }
    challenges
}

#[test]
fn test_vandermonde() {
    let n = 32;

    let two = Fr::from(2u128);

    let challenges = vandemonde_challenge(two, n);
    assert_eq!(challenges.len(), n);

    let mut two_n = two;

    for i in 0..challenges.len() {
        assert_eq!(two_n, challenges[i]);
        two_n = two_n * two;
    }
}
