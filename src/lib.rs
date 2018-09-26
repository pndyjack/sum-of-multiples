pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let mut multiples: Vec<u32> = Vec::new();
  for multiplier in 1..=limit {
    factors.iter().for_each(|factor| {
      if factor * multiplier < limit {
        multiples.push(factor * multiplier);
      }
    });
  }
  multiples.sort();
  multiples.dedup();
  multiples.iter().sum()
}
