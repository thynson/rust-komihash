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

    seed = 0

    bulk(3) = 0x7a9717e9eea4be8b
    bulk(6) = 0xa56469564c2ea0ff
    bulk(8) = 0x00b4313a24431306
    bulk(12) = 0x64c2ad96013f70fe
    bulk(20) = 0x7a3888bc95545364
    bulk(31) = 0xc77e02ed4b201b9a
    bulk(32) = 0x256d74350303a1ba
    bulk(40) = 0x59609c71697bb9df
    bulk(47) = 0x36eb9e6a4c2c5e4b
    bulk(48) = 0x8dd56c332850baa6
    bulk(56) = 0xcbb722192b353999
    bulk(64) = 0x90b07e2158f88cc0
    bulk(72) = 0x24c9621701603741
    bulk(80) = 0x1d4c1d97ca684334
    bulk(112) = 0xd1a425d530652287
    bulk(132) = 0x72623be342c20ab5
    bulk(256) = 0x94c3dbdca59ddf57
    */

    let bulk_table: [(usize, u64); 17] = [
        (3, 0x7a9717e9eea4be8b),
        (6, 0xa56469564c2ea0ff),
        (8, 0x00b4313a24431306),
        (12, 0x64c2ad96013f70fe),
        (20,0x7a3888bc95545364),
        (31, 0xc77e02ed4b201b9a),
        (32, 0x256d74350303a1ba),
        (40, 0x59609c71697bb9df),
        (47, 0x36eb9e6a4c2c5e4b),
        (48, 0x8dd56c332850baa6),
        (56, 0xcbb722192b353999),
        (64, 0x90b07e2158f88cc0),
        (72, 0x24c9621701603741),
        (80, 0x1d4c1d97ca684334),
        (112, 0xd1a425d530652287),
        (132, 0x72623be342c20ab5),
        (256, 0x94c3dbdca59ddf57)
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

    let seed = 0x0123456789abcdefu64;

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("This is a 32-byte testing string"),
            output: 0x6ce66a2e8d4979a5,
        }
    );


    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The cat is out of the bag"),
            output: 0x5b1da0b43545d196,
        }
    );
    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("A 16-byte string"),
            output: 0x26af914213d0c915,
        }
    );
    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The new string"),
            output: 0x62d9ca1b73250cb5,
        }
    );
    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("7 chars"),
            output: 0x90ab7c9f831cd940,
        }
    );

    /**

    seed = 0x0123456789abcdef
    bulk(3) = 0x84ae4eb65b96617e
    bulk(6) = 0xaceebc32a3c0d9e4
    bulk(8) = 0xdaa1a90ecb95f6f8
    bulk(12) = 0xec8eb3ef4af380b4
    bulk(20) = 0x07045bd31abba34c
    bulk(31) = 0xd5f619fb2e62c4ae
    bulk(32) = 0x5a336fd2c4c39abe
    bulk(40) = 0x0e870b4623eea8ec
    bulk(47) = 0xe552edd6bf419d1d
    bulk(48) = 0x37d170ddcb1223e6
    bulk(56) = 0x1cd89e708e5098b6
    bulk(64) = 0x765490569ccd77f2
    bulk(72) = 0x19e9d77b86d01ee8
    bulk(80) = 0x25f83ee520c1d241
    bulk(112) = 0xd6007417091cd4c0
    bulk(132) = 0x3e49c2d3727b9cc9
    bulk(256) = 0xb2b3405ee5d65f4c

    */

    let bulk_table: [(usize, u64); 17] = [
        (3, 0x84ae4eb65b96617e),
        (6, 0xaceebc32a3c0d9e4),
        (8, 0xdaa1a90ecb95f6f8),
        (12, 0xec8eb3ef4af380b4),
        (20,0x07045bd31abba34c),
        (31, 0xd5f619fb2e62c4ae),
        (32, 0x5a336fd2c4c39abe),
        (40, 0x0e870b4623eea8ec),
        (47, 0xe552edd6bf419d1d),
        (48, 0x37d170ddcb1223e6),
        (56, 0x1cd89e708e5098b6),
        (64, 0x765490569ccd77f2),
        (72, 0x19e9d77b86d01ee8),
        (80, 0x25f83ee520c1d241),
        (112, 0xd6007417091cd4c0),
        (132, 0x3e49c2d3727b9cc9),
        (256, 0xb2b3405ee5d65f4c)
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

    let seed = 0x0000000000000100u64;

    /**
    "This is a 32-byte testing string" = 0x5f197b30bcec1e45
	"The cat is out of the bag" = 0xa761280322bb7698
	"A 16-byte string" = 0x11c31ccabaa524f1
	"The new string" = 0x3a43b7f58281c229
	"7 chars" = 0xcff90b0466b7e3a2
	bulk(3) = 0x8ab53f45cc9315e3
	bulk(6) = 0xea606e43d1976ccf
	bulk(8) = 0x889b2f2ceecbec73
	bulk(12) = 0xacbec1886cd23275
	bulk(20) = 0x57c3affd1b71fcdb
	bulk(31) = 0x7ef6ba49a3b068c3
	bulk(32) = 0x49dbca62ed5a1ddf
	bulk(40) = 0x192848484481e8c0
	bulk(47) = 0x420b43a5edba1bd7
	bulk(48) = 0xd6e8400a9de24ce3
	bulk(56) = 0xbea291b225ff384d
	bulk(64) = 0x0ec94062b2f06960
	bulk(72) = 0xfa613272ecd49985
	bulk(80) = 0x76f0bb380bc207be
	bulk(112) = 0x4afb4e08ca77c020
	bulk(132) = 0x410f9c129ad88aea
	bulk(256) = 0x066c7b25f4f569ae
    */

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("This is a 32-byte testing string"),
            output: 0x5f197b30bcec1e45,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The cat is out of the bag"),
            output: 0xa761280322bb7698,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("A 16-byte string"),
            output: 0x11c31ccabaa524f1,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("The new string"),
            output: 0x3a43b7f58281c229,
        }
    );

    test_vector.push(
        TestSpec {
            seed,
            input: Vec::from("7 chars"),
            output: 0xcff90b0466b7e3a2,
        }
    );

    let bulk_table: [(usize, u64); 17] = [
        (3, 0x8ab53f45cc9315e3),
        (6, 0xea606e43d1976ccf),
        (8, 0x889b2f2ceecbec73),
        (12, 0xacbec1886cd23275),
        (20, 0x57c3affd1b71fcdb),
        (31, 0x7ef6ba49a3b068c3),
        (32, 0x49dbca62ed5a1ddf),
        (40, 0x192848484481e8c0),
        (47, 0x420b43a5edba1bd7),
        (48, 0xd6e8400a9de24ce3),
        (56, 0xbea291b225ff384d),
        (64, 0x0ec94062b2f06960),
        (72, 0xfa613272ecd49985),
        (80, 0x76f0bb380bc207be),
        (112, 0x4afb4e08ca77c020),
        (132, 0x410f9c129ad88aea),
        (256, 0x066c7b25f4f569ae)
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

