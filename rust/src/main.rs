use clap::Parser;
use rand::Rng;
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;

struct BloomFilterConfig {
    hash_fns_seed: Vec<u16>,
}

impl BloomFilterConfig {
    fn new(hash_fns_seed: Vec<u16>) -> Self {
        BloomFilterConfig { hash_fns_seed }
    }
}

fn init(num_hash_functions: u8) -> BloomFilterConfig {
    let mut hash_fns_seed = Vec::new();
    for _ in 0..num_hash_functions {
        let seed = rand::thread_rng().gen_range(100..u16::MAX);
        hash_fns_seed.push(seed);
    }

    BloomFilterConfig::new(hash_fns_seed)
}

fn murmurhash(
    key: &str,
    size: u16,
    hash_fn_idx: usize,
    bloom_filter_config: &BloomFilterConfig,
) -> u16 {
    let mut buf = key.as_bytes();
    let hashed_val = murmur3::murmur3_32(
        &mut buf,
        bloom_filter_config.hash_fns_seed[hash_fn_idx] as u32,
    )
    .unwrap();
    (hashed_val as u16) % size
}

struct BloomFilter {
    filter: Vec<u8>,
    size: usize,
}

struct ExistsReturn {
    idx: usize,
    exists: bool,
}

impl BloomFilter {
    fn new(size: usize) -> Self {
        BloomFilter {
            filter: vec![0; size],
            size,
        }
    }

    fn add(&mut self, key: &str, num_hash_fns: usize, bloom_filter_config: &BloomFilterConfig) {
        for i in 0..num_hash_fns {
            let idx = murmurhash(key, self.size as u16, i, &bloom_filter_config);
            let a_idx = idx / 8;
            let b_idx = idx % 8;
            self.filter[a_idx as usize] |= 1 << b_idx;
        }
    }

    fn exists(
        &self,
        key: &str,
        num_hash_fns: usize,
        bloom_filter_config: &BloomFilterConfig,
    ) -> ExistsReturn {
        let mut return_val = ExistsReturn {
            idx: 0,
            exists: true,
        };

        for i in 0..num_hash_fns {
            let idx = murmurhash(key, self.size as u16, i, &bloom_filter_config);
            let a_idx = idx / 8;
            let b_idx = idx % 8;
            let exists = self.filter[a_idx as usize] & (1 << b_idx) != 0;
            if !exists {
                return_val.idx = i;
                return_val.exists = false;
                return return_val;
            }
        }

        return_val
    }
}

impl Display for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.filter)
    }
}

fn experiment_multiple_hashes() {
    let bloom_filter_config = init(100);

    let mut dataset = Vec::new();
    let mut dataset_exists = HashMap::new();
    let mut dataset_not_exists = HashMap::new();

    for _ in 0..500 {
        let uuid_val = Uuid::new_v4().to_string();
        dataset.push(uuid_val.clone());
        dataset_exists.insert(uuid_val.clone(), true);
    }

    for _ in 0..500 {
        let uuid_val = Uuid::new_v4().to_string();
        dataset.push(uuid_val.clone());
        dataset_not_exists.insert(uuid_val.clone(), false);
    }

    for i in 1..bloom_filter_config.hash_fns_seed.len() {
        let mut bloom = BloomFilter::new(10000);

        for key in dataset_exists.keys() {
            bloom.add(key, i, &bloom_filter_config);
        }

        let mut false_positive = 0;
        for key in &dataset {
            let return_val = bloom.exists(&key, i, &bloom_filter_config);
            if return_val.exists {
                if dataset_not_exists.contains_key(key) {
                    false_positive += 1;
                }
            }
        }

        println!("{}", false_positive as f64 / dataset.len() as f64);
    }
}

fn experiment_multiple_sizes() {
    let bloom_filter_config = init(1);

    let mut dataset = Vec::new();
    let mut dataset_exists = HashMap::new();
    let mut dataset_not_exists = HashMap::new();

    for _ in 0..500 {
        let uuid_val = Uuid::new_v4().to_string();
        dataset.push(uuid_val.clone());
        dataset_exists.insert(uuid_val.clone(), true);
    }

    for _ in 0..500 {
        let uuid_val = Uuid::new_v4().to_string();
        dataset.push(uuid_val.clone());
        dataset_not_exists.insert(uuid_val.clone(), false);
    }

    for i in (1000..10000).step_by(200) {
        let mut bloom = BloomFilter::new(i);

        for key in dataset_exists.keys() {
            bloom.add(key, 1, &bloom_filter_config);
        }

        let mut false_positive = 0;
        for key in &dataset {
            let return_val = bloom.exists(&key, 1, &bloom_filter_config);
            if return_val.exists {
                if dataset_not_exists.contains_key(key) {
                    false_positive += 1;
                }
            }
        }

        println!("{}", false_positive as f64 / dataset.len() as f64);
    }
}

#[derive(Debug, Parser)]
#[clap(
    name = "BloomFilterExperiments",
    version = "0.1.0",
    author = "Siddhartha Dhar Choudhury"
)]
struct BloomFilterExperimentsConfig {
    #[clap(short = 'a', long, conflicts_with = "sizes")]
    hashes: bool,

    #[clap(short = 's', long, conflicts_with = "hashes")]
    sizes: bool,
}

fn main() {
    let args = BloomFilterExperimentsConfig::parse();

    if args.hashes {
        experiment_multiple_hashes();
    } else if args.sizes {
        experiment_multiple_sizes();
    } else {
        panic!("Specify either hashes or sizes")
    }
}
