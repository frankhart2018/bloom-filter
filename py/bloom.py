from dataclasses import dataclass
import mmh3
import numpy as np
import random
import uuid


@dataclass
class BloomFilterConfig:
    hash_fns_seed: list[int]


def init(num_hash_functions) -> BloomFilterConfig:
    hash_fns_seed = []
    for _ in range(num_hash_functions):
        seed: int = random.randint(100, 1000000)
        hash_fns_seed.append(seed)

    return BloomFilterConfig(hash_fns_seed=hash_fns_seed)


def murmurhash(
    key: str, size: int, hash_fn_idx: int, bloom_filter_config: BloomFilterConfig
) -> int:
    hashed_val: int = mmh3.hash(
        key=key, seed=bloom_filter_config.hash_fns_seed[hash_fn_idx]
    )
    return hashed_val % size


class BloomFilter:
    def __init__(self, size: int, bloom_filter_config: BloomFilterConfig) -> None:
        self.__filter: np.ndarray = np.zeros(size // 8, dtype=np.uint8)
        self.__size: int = size
        self.__bloom_filter_config: BloomFilterConfig = bloom_filter_config

    def add(self, key: str, num_hash_fns: int) -> None:
        for i in range(num_hash_fns):
            idx = murmurhash(key, self.__size, i, self.__bloom_filter_config)
            a_idx = idx // 8
            b_idx = idx % 8
            self.__filter[a_idx] |= 1 << b_idx

    def __str__(self) -> str:
        return self.__filter

    def exists(self, key: str, num_hash_fns: int) -> tuple[str, int, bool]:
        for i in range(num_hash_fns):
            idx = murmurhash(key, self.__size, i, self.__bloom_filter_config)
            a_idx = idx // 8
            b_idx = idx % 8
            exists = self.__filter[a_idx] & (1 << b_idx)
            if not exists:
                return (key, i, False)

        return (key, 0, True)


def main():
    bloom_filter_config = init(num_hash_functions=100)

    dataset = []
    dataset_exists = {}
    dataset_not_exists = {}

    for _ in range(500):
        uuid_val = str(uuid.uuid4())
        dataset.append(uuid_val)
        dataset_exists[uuid_val] = True

    for _ in range(500):
        uuid_val = str(uuid.uuid4())
        dataset.append(uuid_val)
        dataset_not_exists[uuid_val] = False

    for i in range(1, len(bloom_filter_config.hash_fns_seed)):
        bloom = BloomFilter(10000, bloom_filter_config)

        for key in dataset_exists:
            bloom.add(key, i)

        false_positive = 0
        for key in dataset:
            _, _, exists = bloom.exists(key, i)
            if exists:
                if key in dataset_not_exists:
                    false_positive += 1

        print(float(false_positive) / float(len(dataset)))


if __name__ == "__main__":
    main()
