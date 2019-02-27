use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let multiples = (1..=limit)
    .into_iter()
    .map(|x| {
      let multiples = factors
        .iter()
        .map(move |factor| {
          let multiple = factor * x;
          if multiple < limit {
            return multiple;
          }
          0
        })
        .collect::<Vec<u32>>();
      multiples
    })
    .flatten()
    .collect::<Vec<u32>>();
  let deduped_multiples: HashSet<_> = multiples.iter().cloned().collect();
  deduped_multiples.iter().sum()
}
