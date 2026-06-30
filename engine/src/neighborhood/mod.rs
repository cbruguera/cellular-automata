use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Neighborhood {
    Moore { radius: usize },
    VonNeumann { radius: usize },
}

impl Neighborhood {
    pub fn offsets(&self, ndim: usize) -> Vec<Vec<i64>> {
        match self {
            Neighborhood::Moore { radius } => moore_offsets(ndim, *radius),
            Neighborhood::VonNeumann { radius } => {
                let r = *radius as i64;
                moore_offsets(ndim, *radius)
                    .into_iter()
                    .filter(|off| off.iter().map(|&x| x.abs()).sum::<i64>() <= r)
                    .collect()
            }
        }
    }
}

fn moore_offsets(ndim: usize, radius: usize) -> Vec<Vec<i64>> {
    let r = radius as i64;
    let range: Vec<i64> = (-r..=r).collect();

    let mut offsets = vec![vec![]];
    for _ in 0..ndim {
        offsets = offsets
            .iter()
            .flat_map(|prev| {
                range.iter().map(move |&x| {
                    let mut next = prev.clone();
                    next.push(x);
                    next
                })
            })
            .collect();
    }

    offsets.retain(|off| off.iter().any(|&x| x != 0));
    offsets
}
