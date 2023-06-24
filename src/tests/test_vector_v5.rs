pub struct TestSpec {
    pub seed: u64,
    pub input: Vec<u8>,
    pub output: u64,
}

pub fn komi_rand_test_vector_official() -> Vec<TestSpec> {

    let mut test_vector = Vec::new();
    let mut seed = 0u64;

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("This is a 32-byte tester string."),
            output: 0x8e92e061278366d2,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The cat is out of the bag"),
            output: 0xd15723521d3c37b1
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("A 16-byte string"),
            output: 0x467caa28ea3da7a6,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The new string"),
            output: 0xf18e67bc90c43233,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("7 bytes"),
            output: 0xe72e558f5eaf2554,
        }
    );


    test_vector
}

