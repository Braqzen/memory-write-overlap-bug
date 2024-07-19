contract;

abi MyContract {
    fn test();
}

impl MyContract for Contract {
    fn test() {
        let mut latest_random_seed = 1;
        // Error line 11. Assigning a different value works as expected
        latest_random_seed = latest_random_seed;
    }
}
