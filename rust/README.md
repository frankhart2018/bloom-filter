# Bloom Filter Rust

Unlike the Python implementation, this does not have graphs to display, rather this version offers command line arguments to switch between two experiments:

1. Where the size of bloom filter is fixed and the number of hash functions applied is varied.
2. Where the number of hash functions is fixed and the size of the bloom filter is varied.

## Usage

1. Running the first experiment (fixed size, varying number of hash functions):

```bash
user@programmer~:$ cargo r -- -a
```

2. Running the second experiment (fixed number of hash functions, varying size):

```bash
user@programmer~:$ cargo r -- -s
```

### Note

To view the help menu, run:

```bash
user@programmer~:$ cargo r -- -h
```