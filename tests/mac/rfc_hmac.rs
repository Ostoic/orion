// Testing against RFC 4231 test vectors
#[cfg(test)]
mod rfc4231 {

    extern crate hex;

    use self::hex::decode;
    use crate::mac::hmac_test_runner;

    #[test]
    fn test_case_1() {
        let secret_key = decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let data = "Hi There".as_bytes().to_vec();
        let expected = decode(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
             daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }

    #[test]
    fn test_case_2() {
        let secret_key = "Jefe".as_bytes().to_vec();
        let data = "what do ya want for nothing?".as_bytes().to_vec();
        let expected = decode(
            "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
             9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }

    #[test]
    fn test_case_3() {
        let secret_key = decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let data = decode(
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\
             dddddddddddddddddddddddddddddddddddd",
        )
        .unwrap();
        let expected = decode(
            "fa73b0089d56a284efb0f0756c890be9b1b5dbdd8ee81a3655f83e33b2279d39\
             bf3e848279a722c806b485a47e67c807b946a337bee8942674278859e13292fb",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }

    #[test]
    fn test_case_4() {
        let secret_key = decode("0102030405060708090a0b0c0d0e0f10111213141516171819").unwrap();
        let data = decode(
            "cdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd\
             cdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd",
        )
        .unwrap();
        let expected = decode(
            "b0ba465637458c6990e5a8c5f61d4af7e576d97ff94b872de76f8050361ee3db\
             a91ca5c11aa25eb4d679275cc5788063a5f19741120c4f2de2adebeb10a298dd",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }

    #[test]
    fn test_case_5() {
        let secret_key = decode("0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c").unwrap();
        let data = decode("546573742057697468205472756e636174696f6e").unwrap();
        let expected = decode("415fad6271580a531d4179bc891d87a6").unwrap();

        hmac_test_runner(&expected, &secret_key, &data, Some(expected.len()), true);
    }

    #[test]
    fn test_case_6() {
        let secret_key = decode(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaa",
        )
        .unwrap();
        let data = decode(
            "54657374205573696e67204c6172676572205468616e20426c6f636b2d53697a\
             65204b6579202d2048617368204b6579204669727374",
        )
        .unwrap();
        let expected = decode(
            "80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f352\
             6b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }

    #[test]
    fn test_case_7() {
        let secret_key = decode(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaa",
        )
        .unwrap();
        let data = decode(
            "5468697320697320612074657374207573696e672061206c6172676572207468\
             616e20626c6f636b2d73697a65206b657920616e642061206c61726765722074\
             68616e20626c6f636b2d73697a6520646174612e20546865206b6579206e6565\
             647320746f20626520686173686564206265666f7265206265696e6720757365\
             642062792074686520484d414320616c676f726974686d2e",
        )
        .unwrap();
        let expected = decode(
            "e37b6a775dc87dbaa4dfa9f96e5e3ffddebd71f8867289865df5a32d20cdc944\
             b6022cac3c4982b10d5eeb55c3e4de15134676fb6de0446065c97440fa8c6a58",
        )
        .unwrap();

        hmac_test_runner(&expected, &secret_key, &data, None, true);
    }
}
