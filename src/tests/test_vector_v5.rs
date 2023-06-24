use crate::tests::test_vector_v4::test_vector;

pub struct TestSpec {
    pub seed: u64,
    pub input: Vec<u8>,
    pub output: u64,
}

pub fn komi_rand_test_vector_official() -> Vec<TestSpec> {

    let mut test_vector = Vec::new();
    let mut seed = 0u64;

    let mut bulk_buffer = Vec::<u8>::with_capacity(256);
    for i in 0..256 {
        bulk_buffer.push(i as u8);
    }

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

    /**

    bulk(6) = 0xa56469564c2ea0ff
    bulk(12) = 0x64c2ad96013f70fe
    bulk(20) = 0x7a3888bc95545364
    bulk(31) = 0xc77e02ed4b201b9a
    bulk(32) = 0x256d74350303a1ba
    bulk(40) = 0x59609c71697bb9df
    bulk(47) = 0x36eb9e6a4c2c5e4b
    bulk(48) = 0x8dd56c332850baa6
    bulk(56) = 0xcbb722192b353999
    bulk(64) = 0x5cf87bcba93e6a5b
    bulk(72) = 0x6c79a1d9474f003f
    bulk(80) = 0x88684fa67b351c33
    bulk(112) = 0xdc481a2af36a87dd
    bulk(132) = 0xe172275e13a1c938
    bulk(256) = 0xa9d9cde10342d965
    *
    */

    let bulk_table: [(usize, u64); 14] = [
        (6, 0xa56469564c2ea0ff),
        (12, 0x64c2ad96013f70fe),
        (20,0x7a3888bc95545364),
        (31, 0xc77e02ed4b201b9a),
        (40, 0x59609c71697bb9df),
        (47, 0x36eb9e6a4c2c5e4b),
        (48, 0x8dd56c332850baa6),
        (56, 0xcbb722192b353999),
        (64, 0x5cf87bcba93e6a5b),
        (72, 0x6c79a1d9474f003f),
        (80, 0xdc481a2af36a87dd),
        (112, 0xdc481a2af36a87dd),
        (132, 0xe172275e13a1c938),
        (256, 0xa9d9cde10342d965)
    ];


    for (size, result) in bulk_table {
        test_vector.push(
            TestSpec {
                seed,
                input: bulk_buffer[0..size].to_vec(),
                output: result,
            }
        );
    }


    test_vector
}

