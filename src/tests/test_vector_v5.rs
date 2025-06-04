pub struct TestSpec {
    pub seed: u64,
    pub input: Vec<u8>,
    pub output: u64,
}

const BULK_BUFFER: [u8; 256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

const EXTENDED_TEST_VECTOR: [(u64, usize, u64); 256] = [
    (0xaaaaaaaaaaaaaaab, 0, 0x6eb51151ca3f5e8f),
    (0x4924924924924925, 1, 0xd0d7684a0b441a15),
    (0x69a69a69a69a69a5, 2, 0xbdc899591539ec),
    (0x6150985426150987, 3, 0x2c6edc4d21a46158),
    (0x2fda5a14a848eeca, 4, 0x315daecdd6244136),
    (0x15b1d689af81d36d, 5, 0xe4957662cdff9e62),
    (0xf9e093e2b8768523, 6, 0x62a7be086c89690f),
    (0x16b78aa9093e9b53, 7, 0x3e878a58a68083d1),
    (0x95d067d2882bfebb, 8, 0x3166c67d8776a85e),
    (0x948b6c84507b6f9c, 9, 0x7462610dd885192a),
    (0x61303f66f72defd7, 10, 0x4e4a59e308aa2259),
    (0x7ed32397e9a542f2, 11, 0x55be9f5842e8b946),
    (0x51f6a8d1cf569a93, 12, 0xc327534c28691c13),
    (0x2e28bf5eb90a312a, 13, 0xc39ccfb434394d5c),
    (0xf951a40b0afad49, 14, 0x24d23cada0ba2d0c),
    (0x296daa8df978b0d9, 15, 0xee6ec663e9a59621),
    (0x43415dd23bb3ba15, 16, 0x9ab82afef9ea773a),
    (0x42326731d8292bb5, 17, 0x2311c390617d78c7),
    (0x528725aebe03a757, 18, 0x9ad23821ed5c9908),
    (0x60308f6839a6fc4d, 19, 0xc76d4bf48120d123),
    (0x38529f34561b7133, 20, 0x55416fbe14f39489),
    (0x84aa24b3bf850cef, 21, 0x4039b57275007620),
    (0x64b60aa0cace24e0, 22, 0xd66e9702dac480a2),
    (0xd37c685bd7a89a1c, 23, 0xc83f3c0560669b6d),
    (0xf0aa4e7757a431cc, 24, 0xa91ede34a52f32a8),
    (0x113d89acca5554b1, 25, 0x25661871d1323254),
    (0x5a6e038309e8132d, 26, 0xba98e403bdd2a349),
    (0xc30bc0ef7ac0bbf7, 27, 0xc0c729eb68a8b2c),
    (0x9add0800e2225106, 28, 0xf5a8cbcbbb67c4f2),
    (0xde7e6f00fa44121, 29, 0x201d795e7e317639),
    (0xf12dfe0390c415cd, 30, 0xf7a0694be8b7d49b),
    (0x9a40ef0eaf9a22e9, 31, 0x9632f08aff847fbd),
    (0x5ac5c6c5c97a1a23, 32, 0x67c693cf06e05301),
    (0xc293375904790d61, 33, 0x5fd68a7ec0236d29),
    (0x740e17654289c851, 34, 0x5ac1ff989775d801),
    (0x82c4dd0a00b74e61, 35, 0x31adba1710aec203),
    (0x6b95826da53b2b18, 36, 0x47b2d237d5e9a413),
    (0xccb9d556a80df03f, 37, 0xd35405b6cf47a2cc),
    (0x762ab327edb3bddd, 38, 0xc042de37e2ee0356),
    (0xfdb0d2c6a58cf9de, 39, 0x2c071473ed3daae4),
    (0x8b77d6bb9c329c15, 40, 0x2c1b03df6c8eea17),
    (0x48c3178b0861294b, 41, 0x4aad88a670a17dd4),
    (0x2c25c2576a77c00, 42, 0x713b8276d6a7d2b),
    (0x408874d7b5d58c6a, 43, 0x44056c245d31217),
    (0xc3cf11988cdba8b0, 44, 0x34ec88f5a4b7fe7),
    (0x4c2add82514ea253, 45, 0xc393dc2cfa1cdcb9),
    (0xe3fd3dfd1265ee5d, 46, 0xf6305bb91b6674de),
    (0xe85a0ae2817945ec, 47, 0xdfa0323095d82f0f),
    (0x712b19951c20fcc8, 48, 0x278374e530d5ea96),
    (0x4a4476cefaa6361f, 49, 0x5001abedd7953c25),
    (0xd4032a767301cb59, 50, 0x1a1f84ed52f893ba),
    (0xbf525963b821b089, 51, 0xaed3c517bca3ac8),
    (0x5d492cd7bd9a6cb7, 52, 0x4b33a0b6f084c612),
    (0x7b5c136a9e9549db, 53, 0xafd1fdfb09d2a916),
    (0x9d54c4f9618d8042, 54, 0x35119a0c25eeb0d4),
    (0x68a8dd14b6473659, 55, 0x4988722f1375c6db),
    (0x690da0ff09b1c606, 56, 0x91e76c16ac786cf9),
    (0xfecffa151fba4fe5, 57, 0x72a8c737dca2994),
    (0xe5da1f755ddf48fa, 58, 0x7a1d725add899fc0),
    (0x3ca20a041a5bcf1a, 59, 0xc234cfa4f88e15e1),
    (0x933782bd79bdc099, 60, 0x90f523c98751d9cb),
    (0x2f5089c12ab0fd64, 61, 0xc795772ecdc3e53c),
    (0x60435bd189194871, 62, 0x3f2cc53a32b12bff),
    (0x941726136b6934ec, 63, 0xeb5715d40f511843),
    (0x6762ed0262823b32, 64, 0xffb9036027da62d9),
    (0x852673558451ed00, 65, 0xa4be816eb81cd7c),
    (0xb7e07f51fe2e817a, 66, 0x591a9700a6d8abfb),
    (0x71076626c6d5199f, 67, 0xef4713b66ee9d6cc),
    (0x977d03190213676c, 68, 0x9493ed7f3a589f8),
    (0x276f3c2a87f2b65e, 69, 0xe808b88cbc9d0aaa),
    (0x6f1b29c78616eb23, 70, 0x788999dbfe167a08),
    (0x77c18d03cd66c4ae, 71, 0x6f346a0c9f3acc04),
    (0x6a6be79290ef04a7, 72, 0x68b509b6747c5a67),
    (0x275e0c1aa339ac0c, 73, 0xb59a94bf7bcd4489),
    (0x2ed728e3fa7f4c5b, 74, 0x2fc8b6b04f5989c5),
    (0x712de039f3c55f4b, 75, 0xa74de4642a58667d),
    (0xe941736d2e244f73, 76, 0xffdc9164c6ce970c),
    (0x908f2f9f988cb464, 77, 0x703fe58c0519fd9f),
    (0x26bf63c5bbcffc40, 78, 0x6171435ed3e01a52),
    (0x6c021ee5324049ee, 79, 0x6fc9b11aab6a8dbe),
    (0xee26cc94535bbba8, 80, 0x2ec8429e51a611ba),
    (0xeeb2dc32987663b3, 81, 0x221c36c14e5cf0fd),
    (0x6b0260e3325e9717, 82, 0x281f12e98c385c6d),
    (0x2bc4fa06f8f048db, 83, 0xa4d3fb0d1817c617),
    (0xdc197e69b5310bc3, 84, 0xe3bea7e2fa480593),
    (0x88cbadf81ffb26e4, 85, 0xb022ddbd80087351),
    (0x99af3d8be6f119ee, 86, 0xad288c969b8e2261),
    (0x52fe61dd1eb26694, 87, 0x520c8797632ebf42),
    (0xbb4b8cfe0c2afbd, 88, 0xf6448561b5e53bb6),
    (0x66c488ad76253335, 89, 0x292c36f8e21109c0),
    (0x50e8d8395e195ce8, 90, 0xdc73cb92b464219c),
    (0x4eab1e100cbbe5a2, 91, 0x4ca674a307cb763f),
    (0x8f1b675713bcd37c, 92, 0x679bfa57a8c910fa),
    (0xe73d3da481551cd6, 93, 0xed713647e9e45a22),
    (0xc3b6d8bce5622b8e, 94, 0xdd1b14bf1610dfee),
    (0xc063ec266ee751cd, 95, 0x70a4240988529e35),
    (0xcf2d75f2fa202ef2, 96, 0xc9848c19947817e0),
    (0x6f4da18478a470b0, 97, 0xf584ac65df81109e),
    (0xfcb722ca287a2996, 98, 0xd082d0ca7c09508c),
    (0x4a8692253f177711, 99, 0xcea45c0fab3bfc7d),
    (0x7fb2cd68b9e1366a, 100, 0x4d4faa2d847a9c84),
    (0xac2254bdf33ba9c7, 101, 0xc902c24599911651),
    (0x7288bf6896b5bcd2, 102, 0x4136de3a4dfd1899),
    (0x80a32a640f0f0820, 103, 0x2e3e7b53ea4446b0),
    (0x126e14bd5aed8b35, 104, 0x94b8156565ad1fae),
    (0x49eed56e55a2daf8, 105, 0xa9c9258997daa09d),
    (0x54c7608de9fcdf25, 106, 0xfa20254c9db3053c),
    (0x52601e33a67f1d4e, 107, 0xb033a075c87b194f),
    (0x4ffba8198ad303c9, 108, 0x31e6fcb76b9e56bb),
    (0x31ce9436c5440578, 109, 0x7760778b4bde8f4b),
    (0xdb47893055f1f64d, 110, 0xfa162b0e85b00b38),
    (0xa63ef64cda8b14b8, 111, 0x71c3a820bc96b438),
    (0x43ab673f4969c959, 112, 0xc11bb49ebe5a6d96),
    (0xdebe23a86b48f191, 113, 0xe0c05b8fd38833b2),
    (0x2a3fd8c4401ed7f3, 114, 0xbf567cd00e3080ef),
    (0x7bc8ee96b0ac8174, 115, 0x53f1abfe9ab0a394),
    (0xb13c63b9ac312dd0, 116, 0x72da78f92972396c),
    (0xe694d14fc6aeab7, 117, 0x69b808367776b355),
    (0xddd66795efb2dd5b, 118, 0x2ab51f42f0033977),
    (0x35214a93f852fafb, 119, 0x9c6a3e4653bad21),
    (0x2be78071a34210ac, 120, 0x65378a1238dc487a),
    (0x754dcaff9e69ed4f, 121, 0xb3baf5f527cdc710),
    (0x7726d72beed97037, 122, 0x3d1a189a0246b13c),
    (0x5287ada3b3a29239, 123, 0xa9633f2411e2615f),
    (0xd60ce68050864ddc, 124, 0xb1a44810fb22498d),
    (0x8aac1c4add0426e4, 125, 0x7c1559b0c2b4fa4e),
    (0xda1627cd9b95522d, 126, 0x7888897b6ac45dab),
    (0x71c20f22d4d223a9, 127, 0x3b1c8a64014a9a8),
    (0xa8ee515012cb493c, 128, 0x1bee37d4d15a1e8f),
    (0xc058534425f5cea7, 129, 0xb1a20755888d1752),
    (0xa08d32bda4a1a25f, 130, 0xad86bb75acb49ecc),
    (0x9688b43da03a5330, 131, 0x211311372d96a081),
    (0xdc9b4e641b608948, 132, 0x72aaa2ee3eef621b),
    (0xf385725e863663bd, 133, 0x8d161973992770d1),
    (0x194f6e52955638a3, 134, 0x3f70028e289043a1),
    (0xc21e0af2c43e37e3, 135, 0xbdab240ace166cc7),
    (0xfdc4b6138af4ed40, 136, 0x9b56e23f300988f),
    (0xd038d49a38192862, 137, 0x26ae0e86d05457f4),
    (0x95ef02d0daf47235, 138, 0xcebf6a145dae9b67),
    (0x5b7f20aa64dab59b, 139, 0x2a14139dc2213c85),
    (0x7bb5153de29022ae, 140, 0xd5c893deea9d2c1c),
    (0x46525c3c8c5e7dde, 141, 0x1bb2bb8fe9536833),
    (0x8c7d5edc78b4c1db, 142, 0xb1b003ac584291f2),
    (0xb98f4371d0d12442, 143, 0x9157ccfbc1da360d),
    (0xc42354df99317129, 144, 0xb4b7cad837ab8def),
    (0xe1db7d75fc0371a4, 145, 0x6bc9e9ff1aa36cb4),
    (0xfee491087de96447, 146, 0x58a9e06df7630cf7),
    (0xdf5a48f24cc1213e, 147, 0x2e62822da3939ce9),
    (0x1b4685249fe3bde4, 148, 0x681599000057d0bf),
    (0x9b0625972aa64b55, 149, 0x6eaafb5818837679),
    (0xc05fa932ff16d79e, 150, 0xe9ed49cbe2846761),
    (0xfffc172657308ff7, 151, 0xe98cce01147e099f),
    (0xd4a4caca0becb207, 152, 0xb9a7ace8abfa8626),
    (0x858a04ea53cd3f1c, 153, 0xd7e77c940e33ea02),
    (0xa993e1652a9285f3, 154, 0xdda53934ad344f14),
    (0xa13bacd8b12d5fc5, 155, 0xe9bc62ceb3b2b390),
    (0x6cd36569fa5ba9e8, 156, 0x65c1f6ca1ecdb6f9),
    (0x459d6c12caf1e300, 157, 0x4781a6838d668e5c),
    (0x2d2b5c84f2ac0af2, 158, 0x4bce984ddab3591d),
    (0x48eb2fbd1fdd2750, 159, 0xd7be86c827b4cef7),
    (0x7692efbf3f571c9, 160, 0x3c06db6a37d3a2fd),
    (0x7fe564714865e359, 161, 0x644b87ab1fc611f8),
    (0x5f1b1e5c3317e242, 162, 0xd5907f545c88093),
    (0x34682f2d1801a0e5, 163, 0x85eeced15d329e7f),
    (0x9b4373e1db7372ce, 164, 0x56d675d8a26cf8f7),
    (0x5250501b54c02f3d, 165, 0x8a9680d29b5e406e),
    (0x7103e9069181ff73, 166, 0xa2645aedcc9f4bd5),
    (0x9ad427e3259351d5, 167, 0x1f3f3e331e934307),
    (0xa5865058eb4aff52, 168, 0x9e264d86a9e325d),
    (0xe027879b6555eede, 169, 0xc7d8c4fa1dfce3bd),
    (0x63c386b23605a6a2, 170, 0xc206342712259c58),
    (0xef69f54afa97a934, 171, 0x7b39cffee1da1882),
    (0xc420eaf96b6f2001, 172, 0x42c4ac0ec7eee1ac),
    (0x34087ab89e217c1a, 173, 0x509aed6a1545edf0),
    (0x6159b6657be3d83e, 174, 0x670b12b1027aa1e7),
    (0xe332f3fee1185480, 175, 0xee2063407ffa03d9),
    (0xd81d23f2aef5bdd3, 176, 0x85a728dcd0bd8f37),
    (0xff9bcae23c5c1ae1, 177, 0xda4dda4406a63ac9),
    (0x978f1014756aa410, 178, 0x286549c422d4662),
    (0x65e20b26160387f5, 179, 0x42deb71786fb8bef),
    (0xf19f77f92829f5, 180, 0xcd746edde86333ec),
    (0x87e3b001cdea4a82, 181, 0x6b9451519f5379cf),
    (0xc4b4af0cc0bf556, 182, 0x126c187d412f7ee3),
    (0xc3009b38f09a108, 183, 0x8d1ac43691fd58b7),
    (0xc7b430e1a9f28a7e, 184, 0x7276119a723ec33d),
    (0x13e5cb9e7428cb34, 185, 0xc6a874a82abbea59),
    (0x95c9cf0b4bbc602e, 186, 0x52567da8dd2b55b1),
    (0x4e978b13fd09d847, 187, 0x9730b1b8c5d6862c),
    (0x5f55abd41e9ce19b, 188, 0x29a422a0051c0d52),
    (0x34e95a2885042a33, 189, 0x1c0ffbf393285a44),
    (0x9dc63848d439b9cd, 190, 0x1f320a0f2ea3ed71),
    (0xc2d60bde3b5b286a, 191, 0x77297e3ea1477d84),
    (0xca6a9f12c94cd53, 192, 0xe15fdede38f609c3),
    (0xbb83acfa2e7e9b6c, 193, 0x36a7e446e0cde39a),
    (0x8ede8f00b39c8da4, 194, 0x9d09b13c6f9c3391),
    (0xd2c3ca5c108339b8, 195, 0x900c5bb32d1786e2),
    (0x31eac423c9e1773c, 196, 0xeba04d485204736),
    (0x78d31dddb530a47, 197, 0xb0e6fc94c54ec7ec),
    (0x2e983c8a53488825, 198, 0xeae892e5a94ff222),
    (0xe2b228b4f9fd1b7b, 199, 0x216300d481191c8f),
    (0x7afd612cb5e0da4a, 200, 0x82541f3411b508db),
    (0xb8fd699e4b7406b3, 201, 0x9bf64c92aabdfc0d),
    (0x55d8c0915bfba750, 202, 0xd2937ae129c37251),
    (0xf89d0a9487216506, 203, 0x5b0e51f83ffa25ca),
    (0x9559d0018ab42c86, 204, 0xa0b259ed25e87ee0),
    (0x977deafb5a2062cf, 205, 0x84078d83e2613f72),
    (0x8ea14177f44ff9b1, 206, 0xf744a38eb0a0b399),
    (0x2859159feaf9805a, 207, 0xa7d9f501e3c151e9),
    (0x9f2a70507ca86a6a, 208, 0x865081b988c6bf8e),
    (0x61b697e1d7d1074d, 209, 0x7467b790186f410c),
    (0x1c7db0b98e4d7106, 210, 0xc6beddd3bce0d0bf),
    (0x2f2bc1250e84a6e9, 211, 0x920883258c588f1b),
    (0xb85299759bce44d9, 212, 0x8591882d338434ac),
    (0xd4e32004b2bb3f95, 213, 0x768e51f7baefdc44),
    (0x906fd97988f7c786, 214, 0x83f73c1227705def),
    (0x9a76c12668855530, 215, 0x150f54e855f8429f),
    (0xb518f8e59f5726f7, 216, 0xb1b36800fafae3ce),
    (0xf3a2510a7473e399, 217, 0x2ab8e845496e2c40),
    (0x1afbb388a5bbaa0d, 218, 0xb13089df185c4bac),
    (0xc6683cbc789d0dca, 219, 0x56b56a84158c6c34),
    (0x3499d83619aa83d, 220, 0x70daa274c556a28f),
    (0x30e4028e7ccbc42c, 221, 0x981c9c6d1c070945),
    (0x89c3338a879ccf4d, 222, 0x553d42f0a2f084cc),
    (0x13f81191cd00b6, 223, 0x2d2dc6ae2bdca5e9),
    (0x9abb1bcabd71d7ab, 224, 0xc57954b49d7e95a1),
    (0xae33b9acdf9c6fd6, 225, 0xac9701b745f0bfe0),
    (0x4402b0dd61e564c2, 226, 0xbe3a1b27b8e4875d),
    (0x84653e404c389309, 227, 0xe90b7250cf7a0554),
    (0x31ab1d825a65b50d, 228, 0x6bee1f5b0642a063),
    (0x644b7703ef004195, 229, 0x6370fd6073b6b086),
    (0x16eb7be4571c15d4, 230, 0x712e59e2d66a6abf),
    (0xd5f024b185cb0d47, 231, 0x351e11fad67f3261),
    (0x7acd8a5142cbc6db, 232, 0xc1e40f90d4bc9f45),
    (0xf8da0e4c43754754, 233, 0xb658eef976ea5f22),
    (0x3dc0d1af065cb803, 234, 0x97d96b130b0a3eaa),
    (0x2bfd022dab1754b, 235, 0x2dab834d0932ef74),
    (0xd6f121121a5b02a7, 236, 0xd17a908be9cd8cce),
    (0x902f4c5dbfbe53c4, 237, 0x4c1d1321387d6068),
    (0x7c5604f5f3094639, 238, 0x9482423a016cd7fd),
    (0x40179680aa7a2ae6, 239, 0x5b64231043af0f89),
    (0x93dd056f82cecb63, 240, 0xb85c39bb31ffaa3a),
    (0x6d4acef19395d6c6, 241, 0x9b5606f5de79e2d2),
    (0xef58cfb337e949d0, 242, 0xff2ec8ed49d3b288),
    (0x51bc742395ef4b2e, 243, 0xaaef4acd12c3ffb3),
    (0xb22d14bde3465b7, 244, 0x2e815b87a9b8f8c2),
    (0x867c3c807ce68ac6, 245, 0xc5684fde103b585e),
    (0x2d41bdd2a6cee7a5, 246, 0x3de087f78062168a),
    (0x175fd3ef79f1f574, 247, 0xb4632c7b6637aab8),
    (0x81ca7b016c6ab494, 248, 0x549ac69ebaf1d280),
    (0xb4179c668d13e453, 249, 0xaaeaa5d1424e3f98),
    (0x8a9ac6227c174fd5, 250, 0xc67e7065c109f7ea),
    (0x2ca5980b356e56b2, 251, 0x25465f56d1bb2bff),
    (0x1601de027ceb3e80, 252, 0xe8a4af82f95e14c),
    (0x8789e2a3dbaf63dc, 253, 0xf9bee57c4283d754),
    (0x5f0e81548a83d952, 254, 0x857ad8ddf23e190f),
    (0x37f9acefdbd99ba2, 255, 0x7af4e033fe8267d2),
];

pub fn komihash_test_vector_official() -> Vec<TestSpec> {
    let mut test_vector = Vec::new();
    let seed = 0u64;

    // let mut bulk_buffer = Vec::<u8>::with_capacity(256);
    // for i in 0..256 {
    //     bulk_buffer.push(i as u8);
    // }
    //
    test_vector.push(TestSpec {
        seed,
        input: Vec::from("This is a 32-byte tester string."),
        output: 0x8e92e061278366d2,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The cat is out of the bag"),
        output: 0xd15723521d3c37b1,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("A 16-byte string"),
        output: 0x467caa28ea3da7a6,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The new string"),
        output: 0xf18e67bc90c43233,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("7 bytes"),
        output: 0xe72e558f5eaf2554,
    });

    /*

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
        (20, 0x7a3888bc95545364),
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
        (256, 0x94c3dbdca59ddf57),
    ];

    for (size, result) in bulk_table {
        test_vector.push(TestSpec {
            seed,
            input: BULK_BUFFER[0..size].to_vec(),
            output: result,
        });
    }

    let seed = 0x0123456789abcdefu64;

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("This is a 32-byte testing string"),
        output: 0x6ce66a2e8d4979a5,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The cat is out of the bag"),
        output: 0x5b1da0b43545d196,
    });
    test_vector.push(TestSpec {
        seed,
        input: Vec::from("A 16-byte string"),
        output: 0x26af914213d0c915,
    });
    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The new string"),
        output: 0x62d9ca1b73250cb5,
    });
    test_vector.push(TestSpec {
        seed,
        input: Vec::from("7 chars"),
        output: 0x90ab7c9f831cd940,
    });

    /*

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
        (20, 0x07045bd31abba34c),
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
        (256, 0xb2b3405ee5d65f4c),
    ];

    for (size, result) in bulk_table {
        test_vector.push(TestSpec {
            seed,
            input: BULK_BUFFER[0..size].to_vec(),
            output: result,
        });
    }

    let seed = 0x0000000000000100u64;

    /*

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

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("This is a 32-byte testing string"),
        output: 0x5f197b30bcec1e45,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The cat is out of the bag"),
        output: 0xa761280322bb7698,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("A 16-byte string"),
        output: 0x11c31ccabaa524f1,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("The new string"),
        output: 0x3a43b7f58281c229,
    });

    test_vector.push(TestSpec {
        seed,
        input: Vec::from("7 chars"),
        output: 0xcff90b0466b7e3a2,
    });

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
        (256, 0x066c7b25f4f569ae),
    ];

    for (size, result) in bulk_table {
        test_vector.push(TestSpec {
            seed,
            input: BULK_BUFFER[0..size].to_vec(),
            output: result,
        });
    }

    test_vector
}

pub fn komihash_test_vector_extended() -> Vec<TestSpec> {
    let mut vec = Vec::<TestSpec>::new();
    for (seed, len, result) in EXTENDED_TEST_VECTOR {
        vec.push(TestSpec {
            seed,
            input: BULK_BUFFER[0..len].into(),
            output: result,
        });
    }
    vec
}
