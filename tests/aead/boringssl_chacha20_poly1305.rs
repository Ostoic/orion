#[cfg(test)]
mod boringssl_aead_chacha20_poly1305 {

	extern crate hex;
	use self::hex::decode;
	use crate::aead::aead_test_runner as chacha20_poly1305_test_runner;

	// Testing against BoringSSL test vector from [boringssl](https://boringssl.googlesource.com/boringssl/+/master/crypto/poly1305/poly1305_tests.txt).
	// Pulled at commit (master): 0f5ecd3a854546d943104e1f7421e489b7f4d5aa
	#[test]
	fn boringssl_test_case_1() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("070000004041424344454647").unwrap();
		let aad = decode("50515253c0c1c2c3c4c5c6c7").unwrap();
		let input = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip \
		             for the future, sunscreen would be it."
			.as_bytes();
		let output = decode("d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d63dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b3692ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc3ff4def08e4b7a9de576d26586cec64b6116").unwrap();
		let tag = decode("1ae10b594f09e26a7e902ecbd0600691").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_2() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("070000004041424344454647").unwrap();
		let aad = "1".as_bytes();
		let input = "123456789abcdef0".as_bytes();
		let output = decode("ae49da6934cb77822c83ed9852e46c9e").unwrap();
		let tag = decode("dac9c841c168379dcf8f2bb8e22d6da2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_3() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("070000004041424344454647").unwrap();
		let aad = "123456789abcdef0".as_bytes();
		let input = "1".as_bytes();
		let output = decode("ae").unwrap();
		let tag = decode("3ed2f824f901a8994052f852127c196a").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_4() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("070000004041424344454647").unwrap();
		let aad = "123456789abcdef".as_bytes();
		let input = "123456789abcdef0".as_bytes();
		let output = decode("ae49da6934cb77822c83ed9852e46c9e").unwrap();
		let tag = decode("2e9c9b1689adb5ec444002eb920efb66").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_5() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("070000004041424344454647").unwrap();
		let aad = "123456789abcdef0".as_bytes();
		let input = "123456789abcdef".as_bytes();
		let output = decode("ae49da6934cb77822c83ed9852e46c").unwrap();
		let tag = decode("05b2937f8bbc64fed21f0fb74cd7147c").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_6() {
		let key =
			decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f").unwrap();
		let nonce = decode("ffffffffffffffffffffffff").unwrap();
		let aad = "123456789abcdef0".as_bytes();
		let input = "123456789abcdef0".as_bytes();
		let output = decode("e275aeb341e1fc9a70c4fd4496fc7cdb").unwrap();
		let tag = decode("41acd0560ea6843d3e5d4e5babf6e946").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}
	// Skipped because orion doesn't allow empty plaintext
	// #[test]
	// fn boringssl_test_case_7() {
	// let key = decode("9a97f65b9b4c721b960a672145fca8d4e32e67f9111ea979ce9c4826806aeee6").unwrap();
	// let nonce = decode("000000003de9c0da2bd7f91e").unwrap();
	// let aad = "".as_bytes();
	// let input = "".as_bytes();
	// let output = "".as_bytes();
	// let tag = decode("5a6e21f4ba6dbee57380e79e79c30def").unwrap();
	//
	// chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input,
	// &output).unwrap(); }

	#[test]
	fn boringssl_test_case_8() {
		let key =
			decode("bcb2639bf989c6251b29bf38d39a9bdce7c55f4b2ac12a39c8a37b5d0a5cc2b5").unwrap();
		let nonce = decode("000000001e8b4c510f5ca083").unwrap();
		let aad = decode("34ab88c265").unwrap();
		let input = decode("8c8419bc27").unwrap();
		let output = decode("1a7c2f33f5").unwrap();
		let tag = decode("2a63876a887f4f080c9df418813fc1fd").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_9() {
		let key =
			decode("4290bcb154173531f314af57f3be3b5006da371ece272afa1b5dbdd1100a1007").unwrap();
		let nonce = decode("00000000cd7cf67be39c794a").unwrap();
		let aad = decode("87e229d4500845a079c0").unwrap();
		let input = decode("86d09974840bded2a5ca").unwrap();
		let output = decode("e3e446f7ede9a19b62a4").unwrap();
		let tag = decode("356d9eda66d08016b853d87c08b5c1b3").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_10() {
		let key =
			decode("422a5355b56dcf2b436aa8152858106a88d9ba23cdfe087b5e74e817a52388b3").unwrap();
		let nonce = decode("000000001d12d6d91848f2ea").unwrap();
		let aad = decode("bef267c99aec8af56bc238612bfea6").unwrap();
		let input = decode("537a645387f22d6f6dbbea568d3feb").unwrap();
		let output = decode("281a366705c5a24b94e56146681e44").unwrap();
		let tag = decode("59143dab187449060a3ec2a1681613cc").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_11() {
		let key =
			decode("ec7b864a078c3d05d970b6ea3ba6d33d6bb73dfa64c622a4727a96ede876f685").unwrap();
		let nonce = decode("000000002bca0e59e39508d3").unwrap();
		let aad = decode("cc1243ea54272db602fb0853c8e7027c56338b6c").unwrap();
		let input = decode("b76733895c871edd728a45ed1a21f15a9597d49d").unwrap();
		let output = decode("1fb9b2958fce47a5cada9d895fbb0c00d3569858").unwrap();
		let tag = decode("219b4252deb16a43b292165aabc5d5ce").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_12() {
		let key =
			decode("2c4c0fdb611df2d4d5e7898c6af0022795364adb8749155e2c68776a090e7d5c").unwrap();
		let nonce = decode("0000000013ce7382734c4a71").unwrap();
		let aad = decode("0115edcb176ab8bfa947d1f7c3a86a845d310bf6706c59a8f9").unwrap();
		let input = decode("0dc6ff21a346e1337dd0db81d8f7d9f6fd1864418b98aadcdb").unwrap();
		let output = decode("dad65e4244a1a17ce59d88b00af4f7434bd7830ffdd4c5558f").unwrap();
		let tag = decode("7ae32f186cf9ec59b41b764b34307d4f").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_13() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a84138648a5919a").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_14() {
		let key =
			decode("a8b9766f404dea8cf7d7dfaf5822f53df9ccd092e332a57f007b301b507d5e14").unwrap();
		let nonce = decode("00000000c7f2f7a233104a2d").unwrap();
		let aad = decode("c6d83b6a56408a356e68d0494d4eff150530b09551d008373d6dee2b8d6b5619d67fdb")
			.unwrap();
		let input =
			decode("4d6faeaee39179a7c892faae3719656cc614c7e6ecd8fcb570a3b82c4dace969090338")
				.unwrap();
		let output =
			decode("a15443f083316eef627a371f4c9ac654d0dd75255d8a303125e9f51af4233ff4ceb7fe")
				.unwrap();
		let tag = decode("63c2b4e0973096299488b0a66ffa54c1").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_15() {
		let key =
			decode("5e8d0e5f1467f7a750c55144d0c670f7d91075f386795b230c9bf1c04ba250bc").unwrap();
		let nonce = decode("0000000088049f44ba61b88f").unwrap();
		let aad = decode(
			"5d09bf0be90026f9fc51f73418d6d864b6d197ea030b3de072bd2c2f5cab5860a342abbd29dba9dc",
		)
		.unwrap();
		let input = decode(
			"51a1eebcc348e0582196a0bce16ed1f8ac2e91c3e8a690e04a9f4b5cf63313d7ad08d1efbff85c89",
		)
		.unwrap();
		let output = decode(
			"35aa4bd4537aa611fd7578fc227df50ebcb00c692a1cf6f02e50ed9270bd93af3bc68f4c75b96638",
		)
		.unwrap();
		let tag = decode("4461139c4055333106cf7f7556fd4171").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_16() {
		let key =
			decode("21a9f07ec891d488805e9b92bb1b2286f3f0410c323b07fee1dc6f7379e22e48").unwrap();
		let nonce = decode("00000000066215be6567377a").unwrap();
		let aad = decode("dfdfdf4d3a68b47ad0d48828dc17b2585da9c81c3a8d71d826b5fa8020fee002397e91fc9658e9d61d728b93eb").unwrap();
		let input = decode("c1b0affaf2b8d7ef51cca9aacf7969f92f928c2e3cc7db2e15f47ee1f65023910d09f209d007b7436ee898133d").unwrap();
		let output = decode("8ff4ceb600e7d45696d02467f8e30df0d33864a040a41ffb9e4c2da09b92e88b6f6b850e9f7258d827b9aaf346").unwrap();
		let tag = decode("b2ad07b86aca1b3ab34033c12d6a08cc").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_17() {
		let key =
			decode("54c93db9aa0e00d10b45041c7a7e41ee9f90ab78ae4c1bba18d673c3b370abde").unwrap();
		let nonce = decode("000000003f2d44e7b352360f").unwrap();
		let aad = decode("f1d1b08dd6fe96c46578c1d1ad38881840b10cb5eae41e5f05fe5287223fa72242aea48cb374a80be937b541f9381efa66bb").unwrap();
		let input = decode("1241e7d6fbe5eef5d8af9c2fb8b516e0f1dd49aa4ebe5491205194fe5aea3704efaf30d392f44cc99e0925b84460d4873344").unwrap();
		let output = decode("027b86865b80b4c4da823a7d3dbcf5845bf57d58ee334eb357e82369cc628979e2947830d9d4817efd3d0bc4779f0b388943").unwrap();
		let tag = decode("6de01091d749f189c4e25aa315b31495").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_18() {
		let key =
			decode("808e0e73e9bcd274d4c6f65df2fe957822a602f039d4752616ba29a28926ef4a").unwrap();
		let nonce = decode("000000001b9cd73d2fc3cb8e").unwrap();
		let aad = decode("d57cfbe5f2538044282e53b2f0bb4e86ea2233041fb36adb8338ded092148f8c2e894ef8766a7ec2dd02c6ac5dbab0c3703c5e9119e37c").unwrap();
		let input = decode("3436c7b5be2394af7e88320c82326a6db37887ff9de41961c7d654dd22dd1f7d40444d48f5c663b86ff41f3e15b5c8ca1337f97635858f").unwrap();
		let output = decode("9b950b3caf7d25eaf5fca6fa3fe12ed077d80dcd5579851233c766bb8bb613ec91d925a939bb52fb88d5eda803cfe2a8cda2e055b962fd").unwrap();
		let tag = decode("0887ec7d5e1a4e532746ec247a30825a").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_19() {
		let key =
			decode("4adfe1a26c5636536cd7cb72aa5bded0b1aa64487ad0e4078f311e8782768e97").unwrap();
		let nonce = decode("00000000d69e54badec11560").unwrap();
		let aad = decode("bda1b0f6c2f4eb8121dcbd2eebd91a03ae1d6e0523b9b6f34b6f16ceca0d086654fb0552bfd5c8e1887730e1449ea02d7f647ae835bc2dab4bbc65b9").unwrap();
		let input = decode("19b3f9411ce875fcb684cbdc07938c4c1347e164f9640d37b22f975b4b9a373c4302ae0e7dfdeba1e0d00ced446e338f4c5bc01b4becef5115825276").unwrap();
		let output = decode("ea765a829d961e08bacaed801237ef4067df38ad3737b7c6de4db587a102a86fc4abbaabea0ee97c95ca7f571c7bab6f38cbae60cd6e6a4ce3c7a320").unwrap();
		let tag = decode("a27f18846f5a4f7fcc724656c91cf4f3").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_20() {
		let key =
			decode("eb3db86c14b7cc2e494345d0dfb4841bbd3aa1e2bc640cca0c6c405520685639").unwrap();
		let nonce = decode("0000000088b54b28d6da8c81").unwrap();
		let aad = decode("34b08bb0df821c573dcb56f5b8b4a9920465067f3b5bf3e3254ea1da1a7fc9847fd38bdfe6b30927945263a91fa288c7cf1bee0fddb0fadf5948c5d83eb4623575").unwrap();
		let input = decode("f75c0a357271430b1ecff07a307b6c29325c6e66935046704a19845e629f87a9e3b8aa6c1df55dd426a487d533bb333e46f0d3418464ac1bef059231f8e87e6284").unwrap();
		let output = decode("146ec84f5dc1c9fe9de3307a9182dbaa75965bf85f5e64563e68d039a5b659aa8863b89228edb93ff3d8c3323ab0d03300476aa4aca206d4626a6b269b2078912d").unwrap();
		let tag = decode("854cbb42bade86a09597482c8604681a").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_21() {
		let key =
			decode("dd5b49b5953e04d926d664da3b65ebcffbbf06abbe93a3819dfc1abbecbaab13").unwrap();
		let nonce = decode("00000000c5c8009459b9e31a").unwrap();
		let aad = decode("fe6f4cbb00794adea59e9de8b03c7fdf482e46f6c47a35f96997669c735ed5e729a49416b42468777e6a8d7aa173c18b8177418ded600124a98cbb65489f9c24a04f1e7127ce").unwrap();
		let input = decode("f21f6706a4dc33a361362c214defd56d353bcb29811e5819ab3c5c2c13950c7aa0000b9d1fe69bb46454514dcce88a4a5eda097c281b81e51d6a4dba47c80326ba6cea8e2bab").unwrap();
		let output = decode("911ead61b2aa81d00c5eff53aeea3ab713709ed571765890d558fb59d3993b45f598a39e5eff4be844c4d4bd1ef9622e60412b21140007d54dcf31b2c0e3e98cf33a00fd27f0").unwrap();
		let tag = decode("2865d2a26f413cc92416340f9491e1be").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_22() {
		let key =
			decode("3b319e40148a67dc0bb19271d9272b327bc5eee087173d3d134ad56c8c7dc020").unwrap();
		let nonce = decode("00000000ce5cf6fef84d0010").unwrap();
		let aad = decode("a026b6638f2939ec9cc28d935fb7113157f3b5b7e26c12f8f25b36412b0cd560b7f11b62788a76bd171342e2ae858bcecb8266ff8482bbaed593afe818b9829e05e8e2b281ae7799580142").unwrap();
		let input = decode("27b5627b17a2de31ad00fc2ecb347da0a399bb75cc6eadd4d6ee02de8fbd6a2168d4763ba9368ba982e97a2db8126df0343cdad06d2bc7d7e12eec731d130f8b8745c1954bfd1d717b4ea2").unwrap();
		let output = decode("368fb69892447b75778f1c5236e1e9d5d89255c3d68d565a5bba4f524d6ad27de13087f301e2ef4c08f5e2c6128b1d3e26de845c4ac4869e4c8bd8858ad0d26dec3b5d61a9e3666a3911ba").unwrap();
		let tag = decode("1414f1b91966340417c38226ccca9d3d").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_23() {
		let key =
			decode("43bf97407a82d0f684bb85342380d66b85fcc81c3e22f1c0d972cd5bfdf407f4").unwrap();
		let nonce = decode("000000008b6ba494c540fba4").unwrap();
		let aad = decode("1e0acf4070e8d6758b60d81b6d289a4ecdc30e3de4f9090c13691d5b93d5bbcef984f90956de53c5cf44be6c70440661fa58e65dec2734ff51d6d03f57bddda1f47807247e3194e2f7ddd5f3cafd250f").unwrap();
		let input = decode("4b4c7e292a357f56fdf567c32fc0f33608110d7ce5c69112987d7b5a0bd46d8627a721b0aed070b54ea9726084188c518cba829f3920365afc9382c6a5eb0dd332b84612366735be2479b63c9efc7ff5").unwrap();
		let output = decode("d0076c88ad4bc12d77eb8ae8d9b5bf3a2c5888a8d4c15297b38ece5d64f673191dc81547240a0cbe066c9c563f5c3424809971b5a07dcc70b107305561ce85aecb0b0ea0e8b4ff4d1e4f84836955a945").unwrap();
		let tag = decode("c5ca34599c6a8b357c6723ee12b24da8").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_24() {
		let key =
			decode("12fc0bc94104ed8150bde1e56856ce3c57cd1cf633954d22552140e1f4e7c65d").unwrap();
		let nonce = decode("00000000d3875d1b6c808353").unwrap();
		let aad = decode("b8be08463e84a909d071f5ff87213391b7da889dc56fd2f1e3cf86a0a03e2c8eaa2f539bf73f90f5298c26f27ef4a673a12784833acb4d0861562142c974ee37b09ae7708a19f14d1ad8c402bd1ecf5ea280fab280").unwrap();
		let input = decode("24592082d6e73eb65c409b26ceae032e57f6877514947fc45eb007b8a6034494dde5563ac586ea081dc12fa6cda32266be858e4748be40bb20f71320711bf84c3f0e2783a63ad6e25a63b44c373a99af845cdf452c").unwrap();
		let output = decode("9d9ae6328711fb897a88462d20b8aa1b278134cdf7b23e1f1c809fa408b68a7bfc2be61a790008edaa98823381f45ae65f71042689d88acfa5f63332f0fba737c4772c972eba266640056452903d6522cefd3f264e").unwrap();
		let tag = decode("e84211b6cfd43543f8b1b4db07a494d1").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_25() {
		let key =
			decode("7b6300f7dc21c9fddeaa71f439d53b553a7bf3e69ff515b5cb6495d652a0f99c").unwrap();
		let nonce = decode("0000000040b32e3fdc646453").unwrap();
		let aad = decode("9ff377545a35cf1bfb77c734ad900c703aee6c3174fdb3736664863036a3a9d09163c2992f093e2408911b8751f001e493decc41e4eeeed04f698b6daed48452a7e1a74ec3b4f3dcf2151ca249fa568aa084c8428a41f20be5fd").unwrap();
		let input = decode("572f60d98c8becc8ba80dd6b8d2d0f7b7bbfd7e4abc235f374abd44d9035c7650a79d1dd545fa2f6fb0b5eba271779913e5c5eb450528e4128909a96d11a652bf3f7ae9d0d17adbf612ec9ca32e73ef6e87d7f4e21fe3412ce14").unwrap();
		let output = decode("229da76844426639e2fd3ef253a195e0a93f08452ba37219b6773f103134f3f87b1345f9b4bf8cfc11277c311780a2b6e19a363b6ac2efe6c4cc54a39b144e29c94b9ebbde6fd094c30f59d1b770ebf9fcad2a5c695dc003bf51").unwrap();
		let tag = decode("55e025a1eb87bc84d4be00c775c92ad2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_26() {
		let key =
			decode("4aeb62f024e187606ee7cc9f5865c391c43df1963f459c87ba00e44bb163a866").unwrap();
		let nonce = decode("000000009559bd08718b75af").unwrap();
		let aad = decode("51f5b503b73a5de8b96534c2a3f2d859ece0bd063ea6dfa486a7eec99f6c020983f7148cccb86202cf9685cc1cc266930f04e536ad8bc26094252baa4606d883bd2aeed6b430152202e9b6cc797ff24fc365315ed67391374c1357c9a845f2").unwrap();
		let input = decode("c5d586ceece6f41812c969bcf1e727fe6ff8d1ae8c8c52367c612caa7cdf50e0662f5dffc5ea7d3cc39400dfe3dc1897905f6490fd7747b5f5f9842739c67d07ce7c339a5b3997a7fb4cd0d8e4817ff8916b251c11ef919167f858e41504b9").unwrap();
		let output = decode("252ea42b6e5740306816974a4fe67b66e793ebe0914778ef485d55288eb6c9c45fa34ac853dc7a39252520514c3cb34c72b973b14b32bc257687d398f36f64cc2a668faffa7305ab240171343b5f9f49b6c2197e4fbe187b10540d7cdcfa37").unwrap();
		let tag = decode("ab1d8a5a1f3eda9b5609c0028737477f").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_27() {
		let key =
			decode("9a19e72f005cae1ae78b8e350d7aabe59fc8845999e8c52fad545b942c225eaf").unwrap();
		let nonce = decode("00000000d9dae2ea8d2ffc31").unwrap();
		let aad = decode("1cd73b72c4e103afbefd7c777e0480f3f5e68c60b85bd2e71ef5caebb175d7fc6535d39f38f92c24f2eb0fe97d878ed3d5967c0bb4394a5d41f7d34cda6e1523d3848f049cde554a7d31e1afeab5d3e6150f85858335cbd28c8a7f87d528058df50eea06").unwrap();
		let input = decode("2110378d856ded07eb2be8e8f43308e0c75bc8a3fcc7b1773b0725b7de49f6a166c4528e64120bdf7c9776615d3ce6feeb03de964a7b919206a77392f80437faceb6745845cafc166e1c13b68e70ca2a1d00c71737b8fcbbbd50902565c32159e05fcd23").unwrap();
		let output = decode("5f009fbce4ec8e4ca9d8d42258b1a3e4e920b2fbad33d5e9f07557d9595e841025193b521ba440110dd83958e8ee30219d952b418e98a6c624894aa248aedc0678f2d263e7bfaf54ca379fef6c5d2f7ac422ea4b4369408b82d6225a7a2cf9a9f46fd4ef").unwrap();
		let tag = decode("1c6bdff7d8b9554dc7bf40e50b37d352").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_28() {
		let key =
			decode("ba1d0b3329ecc009f1da0fab4c854b00ad944870fdca561838e38bad364da507").unwrap();
		let nonce = decode("000000008a81c92b37221f2f").unwrap();
		let aad = decode("e57883961b8d041d9b9eeaddcfd61fa9f59213f66571fadffffdd1498b9b014f1ef2e7e56c3044d7f9fa7a1403a1169e86430a2a782137093f5456e142aad03a5f7a66d38009dd01b7fc02c9cf61642dedaf7cc8d46066c281ee17780674c3a36eae66c58d2d765075").unwrap();
		let input = decode("6289944ffa3ccea4bf25cd601b271f64e6deb0eba77d65efb4d69ca93e01996e4727168b6f74f3ccf17bd44715f23ceb8fc030c0e035e77f53263db025021fd2d04b87a1b54b12229c5e860481452a80a125cb0693a2ba1b47e28ee7cbaf9e683c178232c7f6d34f97").unwrap();
		let output = decode("9c44d9135db0dbf81c862c1f69bec55a279794cdd29a58e61909aa29ec4c120c9c5a508d856b9e56138095714a4bb58402a1ad06774cf4ecdf2273839c0007cb88b5444b25c76f6d2424281101d043fc6369ebb3b2ff63cdb0f11a6ea1b8a7dafc80cdaef2813fa661").unwrap();
		let tag = decode("689a141bc11159d306dad7a4ecf6ad9d").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_29() {
		let key =
			decode("0cf8c73a6cffc1b8b2f5d320da1d859d314374e4a9468db7fd42c8d270b7613a").unwrap();
		let nonce = decode("000000003c4c6f0281841aff").unwrap();
		let aad = decode("a38d09a4f1c9241623c639b7688d8d35345ea5824080c9d74e4352919db63c74d318f19e1cbb9b14eebd7c74b0ad0119247651911f3551583e749ea50ff648858dcaaa789b7419d9e93a5bf6c8167188dbac2f36804380db325201982b8b06597efeb7684546b272642941591e92").unwrap();
		let input = decode("4434728d234603c916e2faa06b25d83bad3348990ecde2344368d1a7af1309bd04251bb2e0b72044948f8dea33cce2618283b6af742073a9586b26c1089335fe735141e099785a1235810a3a67ff309e2f0ce68220ba0077ad1a5dc1a4aef898a3b9ff8f5ad7fe60149bd0bd6d83").unwrap();
		let output = decode("bdfbfea261b1f4c134445321db9e6e40476e2dd2f4e4dbe86e31d6a116d25830762e065b07b11a3799aab93a94b4f98c31c0faeb77ec52c02048e9579257e67f5a6bae9bc65210c25b37fc16ee93bda88fd5f30a533e470b6188c6ce5739fa3e90f77120b490fc1027964f277f40").unwrap();
		let tag = decode("780cc54bb6f1c9b78545c1562cd9d550").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_30() {
		let key =
			decode("69f4e5788d486a75adf9207df1bd262dd2fe3dd3a0236420390d16e2a3040466").unwrap();
		let nonce = decode("000000006255bf5c71bb27d1").unwrap();
		let aad = decode("0c83039504c8464b49d63b7f944802f0d39c85e9f3745e250f10119fa2c960490f75ae4dced8503b156d072a69f20400e9494ab2fa58446c255d82ff0be4b7e43046580bc1cf34060c6f076c72ea455c3687381a3b908e152b10c95c7b94155b0b4b303b7764a8a27d1db0a885f1040d5dbcc3").unwrap();
		let input = decode("c15048ca2941ef9600e767a5045aa98ac615225b805a9fbda3ac6301cd5a66aef611400fa3bc04838ead9924d382bef8251a47f1e487d2f3ca4bccd3476a6ca7f13e94fd639a259ef23cc2f8b8d248a471d30ac9219631c3e6985100dc45e0b59b8fc62046309165ddb6f092da3a4f067c8a44").unwrap();
		let output = decode("f0bb2b73d94f2a7cef70fe77e054f206998eacf2b86c05c4fa3f40f2b8cebf034fe17bcbee4dea821f51c18c0aa85b160f8508bd1dc455cc7f49668b1fb25557cdae147bf2399e07fcacaca18eccded741e026ef25365a6b0f44a6b3dd975ee6bb580f5fccd040b73c18b0fbf8f63199ba10fe").unwrap();
		let tag = decode("2ecccea4607d14dbb2d2475792aeb468").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_31() {
		let key =
			decode("ad7b9409147a896648a2a2fe2128f79022a70d96dc482730cd85c70db492b638").unwrap();
		let nonce = decode("00000000a28a6dedf3f2b01a").unwrap();
		let aad = decode("9a6defddb9b8d5c24a26dd8096f5b8c3af7a89e1f7d886f560fabbe64f14db838d6eb9d6879f4f0b769fe1f9eebf67fcd47b6f9ceb4840b2dba7587e98dc5cae186ef2a0f8601060e8058d9dda812d91387c583da701d2ba3347f285c5d44385a2b0bf07150cbc95e7fcfa8ae07132849a023c98817c03d2").unwrap();
		let input = decode("791d293ff0a3b8510b4d494b30f50b38a01638bf130e58c7601904f12cb8900871e8cf3d50abd4d34fda122c76dfee5b7f82cd6e8590647535c915ae08714e427da52f80aef09f40040036034ca52718ea68313c534e7a045cd51745ec52f2e1b59463db07de7ca401c6f6453841d247f370341b2dbc1212").unwrap();
		let output = decode("c2f109d6d94f77a7289c8a2ab33bc6a98d976554721b0c726cbf4121069473e62ba36e7090e02414f3edc25c5d83ac80b49ad528cda1e3ad815b5a8c8ae9ad0753de725319df236983abd3f69ab4465d9b806c075b1896d40bdba72d73ba84c4a530896eb94ffccf5fb67eb59119e66a1861872218f928cf").unwrap();
		let tag = decode("17ec6cf2b172f01e3c456ad047196805").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_32() {
		let key =
			decode("48470da98228c9b53f58747673504f74ca1737d7d4bb6dbf7c0cba6ca42f80b9").unwrap();
		let nonce = decode("0000000056fb4923a97e9320").unwrap();
		let aad = decode("df8ab634d3dca14e2e091b15ecc78f91e229a1a13cba5edd6526d182525ec575aa45bc70fb6193ffcd59bad3c347159099c4f139c323c30a230753d070018786b2e59b758dd4a97d1a88e8f672092bef780b451fd66ba7431cbb5660ea7816cdf26e19a6ebb9aadc3088e6923f29f53f877a6758068f79a6f2a182b4bf").unwrap();
		let input = decode("bc6626d651e2b237f22ee51608ddcffeba5f31c26df72f443f701f2b085d6f34f806e29673584cb21522179edb62a82427d946acabce065b88b2878e9eb87ed1004e55ef58f51ec46375ac542c5782725ff013136cb506fcf99496e13fcd224b8a74a971cc8ddb8b393ccc6ac910bd1906ea9f2ed8a5d066dc639c20cd").unwrap();
		let output = decode("a62e313ecf258cc9087cbb94fcc12643eb722d255c3f98c39f130e10058a375f0809662442c7b18044feb1602d89be40facae8e89ca967015f0b7f8c2e4e4a3855dbb46a066e49abf9cef67e6036400c8ff46b241fc99ba1974ba3ba6ea20dc52ec6753f6fc7697adbccd02b0bbea1df8352629b03b43cc3d632576787").unwrap();
		let tag = decode("d29a8968067aeb457ffc114c3a9efb95").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_33() {
		let key =
			decode("b62fb85c1decd0faf242ce662140ad1b82975e99a3fa01666cac2385ab91da54").unwrap();
		let nonce = decode("000000002f4a5ca096a4faf8").unwrap();
		let aad = decode("cfe3b7ab7550b0e8e2e8235fa0dcef95647ce6814abd3dc3f5a3bd7d6d282504660c34ad8341e4d11402c7d46c83a494d7ddb105e1002979023e0e3dc2978c9ae53e10eb8567e7a02b60e51e945c7040d832ca900d132b4205a35034fed939a1b7965183c25654931a9b744401c4649c945710b0d9733b87451348b32ba81de30ea7").unwrap();
		let input = decode("03b14f13c0065e4a4421de62ab1d842bffb80f3da30bf47d115c09857f5bdd5756fd7c9ac3d9af1c9fb94f2640f7f4386cfba74db468e5288dbe4dd78bfe4f69e41480ca6138e8beacc6eaa3374157c713cfa900c07dd836eaecc8827fa3e70e052ae09e8473e2ae1a10b1bb669ef60a8dd957f6553daa8114918e17371f2ac327bd").unwrap();
		let output = decode("8965db3d3ae4fb483208f147276e7d81b71a86e7202ffc9b1eaade009bc016838dc09ca4bcf30887b2f4243fbd652cd90ebed1ceef8151ff17ea70518d03b0f2a24960aa7de9b30fa65c2e2d57360061aae6d9376e984e9fcd5e5dd0911a4bc8deca832ffb76f252bd7da523076593ba6b174f7d9fb0377e066ecbb6638036241e86").unwrap();
		let tag = decode("28a5284696ed82714eaa94c9ebe6e815").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_34() {
		let key =
			decode("de9c657258774d4ebc09d109a0fc79d66493ae578797cac4eb8830a6a4b547e0").unwrap();
		let nonce = decode("00000000b5e35fe3398efa34").unwrap();
		let aad = decode("436ea5a5fee8293b93e4e8488116c94d3269c19f1d5050def23d280515457b931bbed64a542b317cc5023d648330a4b7adca14dd6f3783207b94f86ccaa0a0ac39b7db00ac87a99e3cd8a764ed9c75da8454479636ab2b29e770b166a5b75cacc425c919bf1ce9ac34afe6b4425c3d9fd2e48bc81e7d15516d60e592bfcc2ebefb660f0995f2b5").unwrap();
		let input = decode("4d68fb683aa4f4c7a16ba1114fc0b1b8d8898610fa2763e435ded8771b3651078bef73d4dfd14e76a34cd5eb9ef4db4ead4da9e83f4ce50fe059977b2d17d687c29335a04d87389d211f8215449749969f7652dc1935a0f9a94538dc81dc9a39af63446a6517609076987920547d0098a9c6766cf5e704883ea32feaea1889b1554b5eb0ce5ecc").unwrap();
		let output = decode("97a97b8f0f5420845ae8d57567f9bba693d30e6db916fad0b971f553ad7d993f806f27ab8b458d8046062ced4778c004b4f958a4436141637c6039963308dea2f54008b7feab79650295ed41bf9e65e1a2d75ab1c7b2a70ebb9e9f38d07a9a672d3e95ea78afe9ac02f2566b48b0251aef6eeeca8bd15bd8d43b559426aa9d15d960ee35cb3edf").unwrap();
		let tag = decode("4ef49e8a0c2ef85826d7f03e81c577f2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_35() {
		let key =
			decode("6885bd333c336c7672db8ebdf24c1a1b605c5a4ae279f0f698162f47e6c73401").unwrap();
		let nonce = decode("00000000f0c4a213a6168aab").unwrap();
		let aad = decode("8ded368f919efb522bb6a9ad009e02ffbc6a16536e34d95cdb34f1153d7cb7b0f3c2b13dd05cedae27cfe68ec3aca8047e0930a29c9d0770c1b83c234dcb0385deae7ae85da73a5f8de3dfb28612a001f4e552c4f67ae0e2ec53853289b7017a58591fd6f70b0e954876bb2f7ec33001e298856a64bb16181017ba924648c09fc63c62eff262c80d614679bd").unwrap();
		let input = decode("fa905a2bfa5b5bad767239fb070a7bc0b303d1503ecd2b429418cc8feba843e5444ed89022fdb379c3b155a0f9ceab2979000a0f60292a631771f2fde4ef065aa746426609082969530a9c70ad145308c30ba389ea122fd766081511a031ce3a0bd9f9f583c7000b333b79ac004fbde6ec3eb2d905977ff95dcff77858e3c424fe8932a6a12139e6ec8d5e98").unwrap();
		let output = decode("0cb3d6c31e0f4029eca5524f951244df042fc637c4162511fea512a52d3f7581af097eb642e79e48666cb1086edbd38c4777c535a20945fabc23e7c9277e2b960aac46865f1026eb6da82759108b9baece5da930ccfc1052b1656b0eadaa120ed0c45ad04b24ae8cdb22ceab76c5f180b46a392ab45b1b99c612546e6b947f4d5c06ad5abee92ff96345ad43").unwrap();
		let tag = decode("fad7d5a5193dfb121c68529ba8c0c35d").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_36() {
		let key =
			decode("fbc978abb1240a6937ccc16735b8d6ed5411cdbc1897214165a174e16f4e699b").unwrap();
		let nonce = decode("000000007968379a8ce88117").unwrap();
		let aad = decode("3913cd01299b8a4e507f067d887d7e9a6ded16dd9f9bb3115c5779aa14239fd33ee9f25756d45262dc3011069356425b5c81a4729594e17c9747119f81463e85625d5603d05e00f568b0c800bb181eb717be8d7a93166a504ce1bc817e15530c5bd2b3df1d4222245ea78a38bc10f66c5cf68d661503131f11af885c8a910b6dce70bc3a7448dfae00595beb707fe054d3").unwrap();
		let input = decode("1a8196cd4a1389ec916ef8b7da5078a2afa8e9f1081223fa72f6524ac0a1a8019e44a09563a953615587429295052cc904b89f778ef446ed341430d7d8f747cf2db4308478524639f44457253ae5a4451c7efca8ae0b6c5c051aaa781e9c505489b381a6dcba87b157edc7f820a8fbaf2a52e484dc121f33d9d8b9ac59d4901d6ed8996ed4f62d9d4d82274c449cd74efa").unwrap();
		let output = decode("d152bcb4c24c3711b0fad28548dc4db605bbc89237cdbea7dbf956b8855d1161a0781f27bd56d798141e2ace339955efb98fe05d9b44cd011e645106bf47726183958cb6df34ce5766695f60bc70b6fe0fabb9afa009a8ef043dbf75f861881368fa07726625448fe608d578cdc48277f2dc53eaaf1bdc075269a42f9302a57cad387a82c6969608acacda20e1cac4596c").unwrap();
		let tag = decode("96ae06cd7c72456e5568a42317046158").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_37() {
		let key =
			decode("77d1a857fbadfe01aba7974eea2dfb3dc7bf41de73686aece403993e5016c714").unwrap();
		let nonce = decode("00000000fdd913a321c40eb0").unwrap();
		let aad = decode("3cb2c06c20cb0832bbacebfc205d77393ca1816346ea2681de4d3ab1fadb774ad273e4713290454496f5281ebc65e04cfe84ed37cd0aedc4bbe3decbd8d79d04a4e434876650e0d64309e336bfb10e924066a64acb92260b2dbd96735d03af03909aa6a80a6e89fda81037257aec21fe9be7e91a64e88e0a58fa38ecba4c4c4cffb61958f3c486cbb0b1d0b0014a2d1d3df248eec1ca").unwrap();
		let input = decode("db8915bfe651e2ecb3ce0b27d99a6bfa7a7c507cfcb2987293018636c365a459c6a138b4428be538413db15bda69e697cbb92b154b7f4d2cbb07965225aa6865d7dcd1ba2c17c484b00b1986fed63e889f25a4966dc3ed4273f1577768f665362d7d3e824484f0dded7f82b8be8797ad951719719365e45abbf76324bc7d657799d4d4f4bb1dba67d96ab1c88519a5bee704f7214814").unwrap();
		let output = decode("acb825e6023b44b03b2efc265603e887954e8612b2ee134bdcb61501cfb9492952bf67be597c3a005b09af74d9e421a576d2c65e98104780feab838d8cb1bd135452ea39dc8907a4c1a6a9161805e4fa3e16989e6a418a7eea2582bf895da967028eab7c95d846a6de4b9980785814cf00484baa2f6de609912fff689bce6e854261ffe866bd8e63274605c7c5ad677bd7897ade543e").unwrap();
		let tag = decode("bcf523a9bcf772e157941753c6d7401e").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_38() {
		let key =
			decode("b7e9b90dc02b5cd6df5df7283ef293ed4dc07513d9e67331b606f4d42dec7d29").unwrap();
		let nonce = decode("00000000a6c191f6d1818f8e").unwrap();
		let aad = decode("0f4269ed5ef0bfff7be39946a4e86e8bf79f84b70cd0b14fecb7be3c071316ce86de3d99d6871e0ba5667d9d7bba7dcaba10cb2a36668b6c3e2fb6c102938b75008bb9c213ebf9b85b5e91a802df0d31d7f11d764b2289f6225212694ab6b7c0e3ff36e84245d9f4f43fc5f98e654dea7ba9bd918658879c5bb4a1642af0d83113e3cf935d3c0d5208318f66f654eb17d8c28a602543e77ad3e815").unwrap();
		let input = decode("2ada0e3c7ca6db1f780ce8c79472af4e8e951ddc828e0d6e8a67df520638ff5f14a2f95a5e5931749ae2c4e9946ae4d5eb5de42fb5b77d2236e2e2bd817df51be40b1b8a6c21015a7c79fe06dba4a08b34013dfa02747b5f03930268404c455dc54a74d9c6e35485e10026da573cb41cd50b64cfafe4cfcdf3c9684ef877e45d84e22bd5e15fa6c8fd5be921366ff0dc6fe2df45f7252972c9b303").unwrap();
		let output = decode("22586fe7338e99cdaad9f85bd724ba4cfe6249b8a71399f9a3707b5c4323b8d96679568dfc8d230aefb453df596e13eb3e8a439249bd64bc93a58f95089a62b94f6562b821c83d91f56c55147381e9de4beb4ae81bd6fe7caef7e7e9a2078f2fba8f3e70d4910da9accc92b8e81a61b0fefbece4bd89443e66e8ddda8e47a66a62f17fd0e7d0a4852ce1a4d43d72a0b5e8914bbec698f060f2b092").unwrap();
		let tag = decode("bd05336ed6426de412aac37661953052").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_39() {
		let key =
			decode("6b2cb2678d1102f2fbbd028794a79f14585c223d405e1ae904c0361e9b241e99").unwrap();
		let nonce = decode("000000007b3ae31f8f938251").unwrap();
		let aad = decode("1c32fd3df22b3e440e2a3c7a7624990194cb16a5f74af36f87fd6ca7d410ce9064316a2d091945deef7d9b35ceec8396069307caced2b80afd7d53ec479c35cedf2dfd4c95c3dd8400f71ad34028c6e4f8681d93d0774064ba38f3fb9b0c1dfa1f5f0c7d20676a5911d999fb6a1d41367a8e99d852bf3d3b7b3f4c233249ed1ca135389a674ff48232ded3f6800a97b6d409c40e6cd70d09bf9d2ad25d9b9485").unwrap();
		let input = decode("b3cb745930e05f3ab8c926c0a343a6eb14809fd21b8390a6fcc58adb5579e5432021765b2d249a0ecf6ba678634c4f53f71495865f031ee97aa159f9ead3a3fcb823ee5238bdf12706a9c6137d236e2e7110ce650c321e41daf0afd62bab2a8fe55d7018de49a14efe6d83a15b2f256d595e998d25309f23633360f5745c50c4e5af8ccc9a8a2cb47064105a023e919c7795d2dc331d3f2afb8c42e5c0bcc26d").unwrap();
		let output = decode("ef70c7de98ab1d4ad817024a970be463443640eb0cd7ff234bdd00e653074a77a1d5749e698bd526dc709f82df06f4c0e64046b3dc5f3c7044aef53aebb807d32239d0652dd990362c44ec25bf5aeae641e27bf716e0c4a1c9fbd37bbf602bb0d0c35b0638be20dd5d5891d446137e842f92c0ee075c68225e4dbacb63cc6fb32442b4bcda5e62cb500a4df2741a4059034d2ccb71b0b8b0112bf1c4ca6eec74").unwrap();
		let tag = decode("d48657033095db3f873c33445fec8d35").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_40() {
		let key =
			decode("4dbc80a402c9fceaa755e1105dc49ef6489016776883e06fcf3aed93bf7f6af7").unwrap();
		let nonce = decode("000000002358ae0ce3fb8e9f").unwrap();
		let aad = decode("cf6ce7b899fb700a90d2a5466d54d31358ecf0562e02b330a27ba0138006b342b7ed6349d73c4c5c6d29bde75a25089b11dac5b27adea7e7640ca1a7ceb050e3aae84a47e11640a6e485bd54ae9fdb547edc7313d24a0328429fcffd8b18f39880edd616447344ebeec9eadb2dcb1fa7e67179e7f913c194ebd8f5a58aea73b0c5d1133561245b6d9c5cfd8bb0c25b38ffb37db5e2de5cdded6b57355e9d215cb095b8731f").unwrap();
		let input = decode("197c06403eb896d2fa6465e4d64426d24cc7476aa1ae4127cd2bd8a48ce2c99c16b1cbf3064856e84073b6cf12e7406698ef3dd1240c026cbd1ab04ee603e1e6e735c9b7551fd0d355202b4f64b482dd4a7c7d82c4fe2eb494d0d5e17788982d704c1356c41a94655530deda23118cba281d0f717e149fbeb2c59b22d0c0574c1a2e640afad1a6ceb92e1bf1dde71752a1c991e9a5517fe98688a16b073dbf6884cfde61ac").unwrap();
		let output = decode("aa87f9a83048b6919c8f2b050315db4e2adae4a9c2ca0109b81961b520e63299dcb028cec0b9d3249a945ee67dd029b40f361245c740f004f8cf0d2214fcfa65e6124a3e74b78aa94345c46fdc158d34823ed249ee550431eaae9218367321cdd6e6a477650469bb3cc137a8f48d9cf27934b16703608b383d2145659922fb83bb2e7ee2ef938a90f2ff846a4a949129b1fb74dde55c5ae013c2f285de84f7dac7d1662f23").unwrap();
		let tag = decode("298f84c8312029a7b1f38c5ea6021f57").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_41() {
		let key =
			decode("9e4a62016dae4b3223fed1d01d0787e31d30694f79e8142224fe4c4735248a83").unwrap();
		let nonce = decode("00000000263a2fc06a2872e7").unwrap();
		let aad = decode("4cd65f68f9f88c0516231f2a425c8f8a287de47d409d5ecde3ad151e906b3839fb01bb91a456f20ea9d394d4b06604ab1f9009ef29019af7968d965d1643161ab33a5354cda2fdc9f1d21ec9cb71c325c65964a14f9b26eb16560beb9792075a1597394000fd5f331bd8b7d20d88e5f89cf8d0b33e4e78e4904bb59c9c8d5d31ac86b893e4a0667af1be85fdb77f7ec3e2594a68048d20c2fb9422f5879078772ee26a1c560cbcbb2113").unwrap();
		let input = decode("5a46946601f93a0cee5993c69575e599cc24f51aafa2d7c28d816a5b9b4decda2e59c111075fb60a903d701ad2680bb14aeda14af2ae9c07a759d8388b30446f28b85f0a05cd150050bd2e715ff550ebbd24da3ebb1eac15aba23d448659de34be962ab3ab31cb1758db76c468b5bb8ce44b06c4e4db9bd2f0615b1e727f053f6b4ffb6358d248f022bcad6ca973044bed23d3920906a89a9a9c5d8024ec67d7f061f64529a955ce16b3").unwrap();
		let output = decode("e944bb2ab06d138ad633c16ce82706ecf0ef5d119be1f3460c9ce101d9c4e04ef1677707fca40d1f8ca181e07273707b06624d6d7063c3b7b0bb0151b757b3e5237fb8004c161233d8bc7e5f28ea1c18da1874b3d54c5ad6ff0835eed35c8853704585cf83996e5e7cec68180af414e04f08134d3b0384ebdf0393c9310b55d8698fe10cb362defc0995e9a13b48b42cff61ffd9fe4c3c8c6dab355713b88f6e98a02e7231a0c6644ec4").unwrap();
		let tag = decode("6234e81e089b779d0d509d14e566b5d7").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_42() {
		let key =
			decode("18ca3ea3e8baeed1b341189297d33cef7f4e0a2fab40ec3b6bb67385d0969cfe").unwrap();
		let nonce = decode("00000000b6aef34c75818e7c").unwrap();
		let aad = decode("f40f03beaa023db6311bad9b4d5d0d66a58d978e0bcbbf78acebde1f4eb9a284095628955a0b15afc454152f962ec3ea2b9a3b089b99658e68ede4dee5acd56672025eb7323bcbc6ba5d91c94310f18c918e3914bbbf869e1b8721476f9def31b9d32c471a54132481aa89f6c735ab193369496d8dbeb49b130d85fbff3f9cb7dccea4c1da7a2846eef5e6929d9009a9149e39c6c8ec150c9ab49a09c18c4749a0a9fcba77057cdea6efd4d142256c").unwrap();
		let input = decode("ef6d1bb4094782f602fcf41561cba4970679661c63befe35ff2ca7ad1a280bf6b1e7f153fa848edfeffe25153f540b71253e8baba9aeb719a02752cda60ea5938aab339eead5aabf81b19b0fc5c1ed556be6ad8970ea43c303d3046205b12c419dea71c4245cfedd0a31b0f4150b5a9fe80052790188529ab32f5e61d8ccde5973ed30bdf290cbfbd5f073c0c6a020eac0332fced17a9a08cef6f9217bd6bef68c1505d6eed40953e15508d87f08fc").unwrap();
		let output = decode("c531633c0c98230dcf059c1081d1d69c96bab71c3143ae60f9fc2b9cd18762314496ab6e90bf6796252cb9f667a1f08da47fc2b0eecda813228cae00d4c0d71f5e01b6ce762fa636efffe55d0e89fdc89ba42521cc019ab9d408fcd79c14914e8bbf0ea44d8a1d35743ad628327e432fdcfeb0b6679ddca8c92b998473732abd55dba54eefff83c78488eee5f92b145a74b6866531476fc46279d4fde24d049c1ce2b42358ff3ab2ba3a8866e547af").unwrap();
		let tag = decode("e3b4192f6e50528c4f4f70267f094c56").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_43() {
		let key =
			decode("95fdd2d3d4296069055b6b79e5d1387628254a7be647baafdf99dd8af354d817").unwrap();
		let nonce = decode("00000000cd7ed9e70f608613").unwrap();
		let aad = decode("24a45a3a0076a5bcfd5afe1c54f7b77496117d29f4c0909f1e6940b81dde3abacb71ec71f0f4db8a7e540bd4c2c60faee21dd3ce72963855be1b0ce54fb20ad82dbc45be20cd6c171e2bebb79e65e7d01567ad0eeb869883e4e814c93688607a12b3b732c1703b09566c308d29ce676a5c762a85700639b70d82aaef408cf98821a372c6a0614a73ba9918a7951ea8b2bb77cd9896d26988086d8586d72edc92af2042ff5e5f1429a22f61065e03cfcd7edc2a93").unwrap();
		let input = decode("0248284acffa4b2c46636bdf8cc70028dd151a6d8e7a5a5bc2d39acc1020e736885031b252bfe9f96490921f41d1e174bf1ac03707bc2ae5088a1208a7c664583835e8bb93c787b96dea9fc4b884930c57799e7b7a6649c61340376d042b9f5faee8956c70a63cf1cff4fc2c7cb8535c10214e73cec6b79669d824f23ff8c8a2ca1c05974dd6189cfee484d0906df487b6bd85671ce2b23825052e44b84803e2839a96391abc25945cb867b527cdd9b373fbfb83").unwrap();
		let output = decode("40c6318d9e383e107cdd3e1c8951562193c3ef64ee442432a63e2edefc78f32ab07772aeac172cb67ecf4d21f8b448423527bbeb9d8ddd0b46bdb27f74096ceb24e41963b4cdca176676a75bdbe3abc270b349ac0c6cbd9c3a5cd5bce20202fc5cc0c1bdd4fd25e121e0a24bd7bbeb9b19b1912467bf5338ee2ce88aa383c082b42cc399c9654ca325f35523e81438beb3f8926be79c378822d7c8f785614408a5f7cac49e4543188725643e6c1a70b46d0ec400").unwrap();
		let tag = decode("874875c9a0ba3060a0680291c3dc85a2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_44() {
		let key =
			decode("6ae1102f84ed4dc114bb9d63f4dc78d7dbb1ab63f1659dd95f47940a7b7a811f").unwrap();
		let nonce = decode("00000000c965d578ba91d227").unwrap();
		let aad = decode("dfd4ac3e80b2904623ff79ea8ee87862268939decf5306c07a175b6b9da0eb13ac209b4d164755929e03240a0fe26599f136fb2afdffd12bb20354aa1d20e5799839abb68ae46d50c8974e13e361d87ef550fe6d82e8b5b172cf5cd08482efdef793ede3530d24667faf3a1e96348867c2942641f4c036981b83f50236b8e8a10b83ebf6909aad0076302f1083f72de4cf4a1a3183fe6ec6bfe2e73e2af8e1e8c9d85079083fd179ccc2ee9ff002f213dbd7333053a46c5e43").unwrap();
		let input = decode("b82a8a9209618f1f5be9c2c32aba3dc45b4947007b14c851cd694456b303ad59a465662803006705673d6c3e29f1d3510dfc0405463c03414e0e07e359f1f1816c68b2434a19d3eee0464873e23c43f3ab60a3f606a0e5be81e3ab4aa27fb7707a57b949f00d6cd3a11ae4827d4889dd455a0b6d39e99012fd40db23fb50e79e11f8a6451669beb2fbd913effd49ad1b43926311f6e13a6e7a09cf4bebb1c0bf63ce59cd5a08e4b8d8dbf9d002e8a3d9e80c7995bb0b485280").unwrap();
		let output = decode("a9aeb8f0a2b3ca141ac71a808dcc0c9798ac117c5d2bd09b3cfe622693a9f8ca62e841b58bddb2042f888e3099b53638b88dfc930b7a6ee4272d77e4b1d7e442bab6afbde96ab0b432f0092d9ca50eef42f63c60c09e7b8de019b32ebe4030c37b8183cc1e3b913b0ce4ee4d744398fa03f9af1c070bed8cdafd65b3a84140cb4deadc70184de757332ce3780af84353f540755227e886a8d7ad980f3dd6fd68263d82e93f883381dec888bc9f4f48349aa2b4c342cb9f48c6").unwrap();
		let tag = decode("f6dcad5412b95994f5e4d6829c2eba98").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_45() {
		let key =
			decode("405bb7b94715b875df068655f00513cb1ae23ffaac977ce273e57d3f83b43663").unwrap();
		let nonce = decode("000000005c6da1259451119a").unwrap();
		let aad = decode("6fe6446505677bf08b385e2f6d83ef70e1547712208d9cebc010cba8c16ea4ece058d73c72273eed650afdc9f954f35aa1bdf90f1118b1173368acbc8d38d93ebf85bd30d6dc6d1b90913790c3efa55f34d31531f70c958759b2ba6f956c6fcdd289b58cb4c26e9515bf550f0fd71ab8527f062c9505cbb16e8e037d34de1756bef02a133dbf4a9c00ac03befc3fb7f137af04e12595ce9560f98b612480fcdba3b8be01db56ebec40f9deae532c3b0370b5c23a2a6b02a4de69efa8900c").unwrap();
		let input = decode("f9f143c0c52c94b4ba7b0608b144156a49e7b5d27c97315743d171911e3645ab7957c80924e3c6b9c22ab7a1cac4b7e9c0de84e49fd5e4a2d1ab51d764fc5670318688ec942f7ab34c331dce8f90fea6972e07f0dadec29d8eb3b7b6521ddd678a6527a962f4d8af78c077e27f7a0b2ef7eabd19e92b7f8c1e8fb166d4763ce9c40c888cf49aa9cdfc3e997c8fe1cce3fe802441bbd698de269ff316f31c196e62d12c6bb5cd93fb3c79ca6369f8c1ac9102daf818975ea7f513bb38576a").unwrap();
		let output = decode("1a4b073881922c6366680cc9c2a127b26f264148651b29abb0c388cf6c9b1865dba5a991e1f8309efbdb91bce44b278772c58fd41273526c33fec84beb53d1689b9da8483f71be6db73a73417069bb4cd3f195236e8d0a00d124eed3a6b6f89415b19a27fbe35774f6a1a6ee4bd4350b252b975f0db2d2eea82f4836350850d6290901e726e8af13644e2d98bc1d569c20800521e6affe976bd407049a2e6d9dd23f88d52e651391ecd2fc45b864310824aaadfa203762a77c1d64562dae").unwrap();
		let tag = decode("90fcc2544880250f1c3abe8a3761ba08").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_46() {
		let key =
			decode("8c602bd94c630cd00c7a9c508067a5a9f133d12f06d9f6fe2a7b68dce4786d8a").unwrap();
		let nonce = decode("00000000760de0f7b7cb67e2").unwrap();
		let aad = decode("9e14907c3a8e96c2636db1f3d78eb1f673d6ef043cbbb349467f1fe29bf60f23d5d5d1c3b133a8ad72065d822347541c13d1574baf737eb3cc3382fb479e6d5193b9c8e7d2444c66971ef099dc7f37f6cd97b9f7959d46e2cf25e8a5b3111b4d9e2ef906d905f0ee2d17587f7082d7c8e9a51509bde03d3d64338e1838d71700f1b4fcb100b5e0402969da462f26f974b4f9e766121f8fd54be99fc10beb9a606e13fbb1f960062815d19e67f80093360324013095719273c65542b0e31b1a2a3d928f").unwrap();
		let input = decode("c3ff559cf1d6ba6c0cc793ca09a0ba573a28359386a6ec93e1bacd8e630209e0b477a20aedec3c9cbf513ee6a1e3887112218d6155b9875f7e6c4bbba2c31972e905d19f529f4f0f9502996199f94f8728ba8d6424bb15f87fcacd88bb42c63fcc513759712bd0172b1e87c9da122f1993ffb7efd3a5c34b240dd3db89dddea36dbeb2836d9f8648f8e7cd428c0f948097af753b35f9876059e7702027bb00dc69071206e785f48fcbf81b39cc0343974ac70784a2e60c0df93b40379bea4ad8cac625").unwrap();
		let output = decode("2794e6e133f6892f23837fff60cf7c28ee9942f8982ef8089db117903d0143293fdf12ea1cc014bcd8806fb83c19570eed7af522db0de489bbc87133a13434518bcfb9cda4d9f6d832a69209657a447abf8afd816ae15f313c7ea95ec4bc694efc2386cdd8d915dc475e8fadf3421fbb0319a3c0b3b6dfa80ca3bb22c7aab07fe14a3fea5f0aee17ab1302338eeac010a04e505e20096a95f3347dc2b4510f62d6a4c1fae6b36939503a6ac22780a62d72f2fc3849d4ef21267fffdef23196d88fbb9b").unwrap();
		let tag = decode("7fa630c9bcb455e89f13d7a99d5e8dbe").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_47() {
		let key =
			decode("bd68ff5eb296c71cfe6bc903c14907f7726bcb1331f0c75f7801cd1b7948f3a1").unwrap();
		let nonce = decode("0000000065a748004b352ba6").unwrap();
		let aad = decode("5557b08a5010cbc9f46bb140c2505f68684eb24889324bff44b27234fd7a95a99cfb4ff90a8f9982085b725f78ac42eca6ce7f3314e457dc41f404008681a9d29ba765660de2e05bb679d65b81f5e797d8417b94eb9aabbd0576b5c57f86eae25f6050a7918e4c8021a85b47f7a83b4c8446898441c5cc4e0229776ef3e809cb085d71f3c75ec03378730cb066150f07e60f96aec983c0e7e72bf6bf87ae42228dfda195f97855fcdf4e6d1c4479d978abcfa276d16ed60ecbfbfc664041335ce65a40a2ca3424df").unwrap();
		let input = decode("52bf78c00f6e5dca2fc60e2e9a52e827df97808e9cf727773860cafc89f4b64178a19b30b46ed813fe00c8f09b25a6a1b6e350d5b005122934a59bfbd5e6e0c635c84a5226c3f2f7dcf951560f18ac220453d583015fdb2e446c69c6e6fdecf2e595e04fab1b0c506e3c6bd5e4414a35f15021e97f447aa334f54a8f1ef942dec6273511b5668b696fca97188ff15ed84b2f46145cce031c1a7f00bd88bb83d90797edc46161b3fda7a2299173496d73b812139556e8b4eb318078b9eb2ae5046e83b79dd3d45950").unwrap();
		let output = decode("a5c8cf42287d4760fca755e2111817b981c47e85b0047de270ec301ca5f7b3679f4749210892b6ea6568f3a6a4344734a0efc0120ffedecf212d55cbcbb67815ac964875af45f735b70092a8f8435f52fc01b981ae971d486026fb69a9c3927acfe1f2eab0340ae95f8dbee41b2548e400805ece191db5fd1f0804053f1dbfaf7f8d6fded3874cb92d99a2729d3faaa60522060cf0b8101b463b3eb35b380fcddb6406c027d73fe701a5090c8dd531c203ce979e26b9ced3431e2b726a7244a20d9377bd62951bf5").unwrap();
		let tag = decode("82c6194de4d27aac4c54b023b9831634").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_48() {
		let key =
			decode("934fd043c32d16a88fad01c3506469b077cb79d258b5664fa55ad8521afdcaa2").unwrap();
		let nonce = decode("00000000c7091f6afbbeb360").unwrap();
		let aad = decode("f737dd85638eb324dd3891219c5eef7c2dd053cfd055d447a411eba304a4b27dce981d112c4540590933c153d603022c91ebd2b4a58069d27e6ca17a462ef822ca41bffa80b43a68b1b564644cb3c5a7f0fddf7a13a30ff24437fddd8ef93c6f6f205d054f81890d982bd4d4ece0b1563677e843fe48c1f54e9a57ed4da66061482712e710a401073be5080d5b8b96525bffa67de5af31d50385fbbf1a87c21bf0e0a1fdff69ec32c7b7103e0b8ee6c844245e0fc84b9f89fcce62966cea68e2871d3b82e8df424c76309fc88d").unwrap();
		let input = decode("2bdd1fc4f011ef97ea52ec643819941c7e0fb39023c2f3c7683804a0ddee14a5d1784a5246966d533b3538edc7d8742d27061c3cab88df0318ab242102de3a54d03632eeb871b72c7e8f8065b49f4a91e95e15f3f46b29fd76b8fcea0d23570c5530e3bbb8a6aafa9ae32c1b3eac653c5ed5fdb2da5a986075808f6385870c85b1913e26042a9d8e78f5bc2ea6de5a64f8aeafa22adcffc7f6932d543c29bb3a04614783f948680e433a71573568d2ce984d249fb4fc06a9f358c76aa3e64a357f4eae924c1356bd5baccf7e0f").unwrap();
		let output = decode("dd13fbf22c8d18354d774bcd18f7eb814e9b528e9e424abc4e3f2463195e8018576565d16ab48845d11c9277f2865ebb4dc412fd5b27078f8325eadf971e6944c66542e34d9dda971e2aba70dbd3e94a1e638d521477a027776b52acf90520ca229ebc760b73128879475d1cbe1f70fc598b549cd92d8a9ac6833e500c138c56474db84cb3d70b7aa4f293a4c2b4d818b0ff9fd85918dc590a12a8c0e375c4d98b7fc87596547eb960676aad5559834588f00f251a9d53f95c47af4df3c4299175d5211779c148cfc988a5e9d9").unwrap();
		let tag = decode("aeb0a4eb29886f0a7a12ec0516bd4af5").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_49() {
		let key =
			decode("f9f6eb9ad736a8f66e7459fef5ec2890188dc26baf34a95f6f0384e79f5c6559").unwrap();
		let nonce = decode("000000007858dfc084fe4b0f").unwrap();
		let aad = decode("2048d1c2ddfb5ec385b201832c7a993f229ba72ec16d6ebf723ef0c5032b9966209a9e8a63151b40412e96b82f86728ea6588c7e8e11ac71cc8eabab8c4b54de866658d9c5011def61fb3dbe4e630158a45ea41a2ed55ebd1efb1abeda7637de6fa5fd2f151c6d2f385bf6cd002ca8b4a2896e0d65944ee913e3c784669dd201b1985ef3577f7f123a5f9bcffa176c8f557c4f729133cac518642f27d9b22ca9b97faaafe5b669a10b79ace4a7d5727df146c77ce681357d69f9c2d65b4401bd73cd113387e3b3a05d897adad7a24c485e7b").unwrap();
		let input = decode("a644ca6e7cc076e87eb2929fd257693fce0f6fb64fd632f7f07c648ebd03696c8e262e6a810d7b7c4e5eef8c65b5323c99dbba50a70b4a9e5c2a9e7315973cd67f35d8052ce9a85a206416dd3031929f4f929b13d0a5fb10cb73c65f6c0ace019da146b51c5274a099f44e3669d26add6f2ff081e886f3cf952fe0dbbe6b0534c23e307574bd35fbd657f5fcbd5dc19fb382a1dc0a2dc8285a0350f71554e4c601497749e35567dd4a273cddc9a48ce53a5f1d297fd8baf8d1b9feb35d9151114345abada4d90db947bb9a743c175f5653d1").unwrap();
		let output = decode("4146faffd7313f5d9f625370d20413cc62ab65f4acfa3c7ee1125b937dd7a39f638fc46c8ed004fb525698de5d8620ec153435571817c3de257b0d0e648ebb92940c86a98262d54e764f28cbdd4f7d9bea970291f2110414f62064d7229c6332236c507b3dac742e651d85a2a22fb243c0cc7cc2d016e5bea38f33f9a9ce048944a5fe8b078d71d23168e12dfe5a0f0b829771edc7073fb96032b7be471337a37aca0cf7c0cdd543eed686cd34934717fd79a3f18492eef72f9f450b880aa7e2e1b65e3b04c22e72301338b43aa32ceec2e6").unwrap();
		let tag = decode("61c6d4d6918b04fc1b72a7a0e9a3b799").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_50() {
		let key =
			decode("29b19636cdd32507fd98ec4ee26caab1a917646fb8f05b0dc01728a9f4a127f0").unwrap();
		let nonce = decode("0000000006699d245916686d").unwrap();
		let aad = decode("39116c49cc13adb065b92cb7635f73d5f6bf6b5ccbf72a3f65a5df6bd4a661105015358d9e69f42e98aed795e8161282bc113058b7ef3b9e23fcd8eeab34a392e03f4d6329c112cb968385ec52a7afc98bb8695785af6b27b700973cc952630b7247ce226b4fbb99b8a486370bf6345d4516c52c64e33f407c4f2d1ba90545c88732d98bbd97972ac5e94c694624a9b3782b0099824651cb7567914d25b3e13181a791dbcd40e76e836b3350d310a52151bf835d3c357c9871482c2928e8404c6e533406d4d6fa8f63366f2c4ed828141f1ff00f01a536").unwrap();
		let input = decode("5fdf913aceab1d6dbaf7d9a29352fa8a3eb22718043a79cffa2fe8c35c820aec7c07644b8785dcf7a433b4189abb257fb12b06fae0662641011a069873c3e3c5ccc78e7358184a62c2005c44b8a92254958eb5ff460d73cd80284d6daba22c3faba046c5426fe8b7cacec64b235a8f8d3e2641e5bc378830594bcfb27c177aea745951ee5780a63705727ef42c4ad3abf556d88e3830f3db6b09e93edd09485cbf907f79de61f8dc5cb5fb7665ffa0ef53cb48702f6a81d8ad421cef20c1dbdf402b8fafed56a5361b2f93f914a2380fdd0557faf1f4de").unwrap();
		let output = decode("01e237220b619054a1f3670928fe67d40484b5af40fbd04d032500aac5acaa3b4584dd99a58c390627636a50de5d744f76a56a33205f9e3b00e16162eb47ff3333e1e208ca200f1a5338a86e17bd92dd2d16af8bb022a7dc05b923d019e05247f1a0d0b4bfcfce58dd6d83830705707676d55739abee89fcd5cb94b8fde006a5da02df64b00a467f45970b5ca440f22319b9735a55d454b9fba0588fef0c59d3d83823eba6e0601a96e10233826c5adeea6b2a51d386a07a9e047ad405b23d4c3d89f30c31e3199f0c8f927bfac43ceea1f969de0a8c0f").unwrap();
		let tag = decode("b9fec6da464c7b85b2a4726694562fe9").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_51() {
		let key =
			decode("bae06b9b5456707551c7b0e207aae02a19b4848ad8ca4ce40705bf8c856a6e52").unwrap();
		let nonce = decode("000000009c27065c3ef2d522").unwrap();
		let aad = decode("5d5590db1bd316eb7a0e30e4c7a6dfdbef9d3287fdb8d824389599c3c2ee262b2192eb5b9708e66e22dbc7eca83fa1a995da3ce64c86fe5aa08b826d476dc439497e2d12e2702c63c8d27aa7f09fedee816dc8bffe1351d53271a34d4292b613b7efcedb7e3cf3e6ad389eef12471e9e20e38e7ae22a323abbadfe8f2e84271bffb1819feb4f77b82843cb8757cfae293631bc6d39669107e7015c85d7343ffa6fc1bbe6f5ab4de30cd752a281e03061ea89de2a3f5e90e20da22fd6e8525c100738667f42212b2cf45fcb23bbb54b21c117484b22c6e514685314df").unwrap();
		let input = decode("50cdd88137ff428a88e87b5845be4924f6387537bb5c0b654c80107ab5698db75b2e131848e7aec156d31aed0766d31c379fece4095d38264c6d5945974d25f729c3b0ba11ea853e9cebdb6f03bb670fce08adff74d0a8f02d633fb34e0fb7337a8e66e1c12084d914fb6173b8105684db822752c6751a372bb16690284d661b8b8bc6a6dfbddf45ebc2219596f9f2f878c118df69030de38b4d99dde43b9b9e20a3dab691645dd518342f49b06a0fe0a397adf261e99f07af5b0b3798b1022ba0939c42a54d3b93641cffa3c2e174bce9ab7ad7e7c7924308d1a77a").unwrap();
		let output = decode("66b7f69ac49fab4e5975aeb6fa9287d8eac02ac312c4de78f77f59da16cbcf87274e66801c4b862c33ea79cdc76528862bb2956c06db8b8acfac4794ebf39e35ac03cc73a4351a4ff762f681a48d6f25cad36e2814c9b5c40b9ae92509e58429106847789454d376836936bebc7a80e6c66e7aa52936d6b361378a41f849ad4e48f9ee2d3e92217a908fa8eb35736ac8ada7d32ae05391f2d807be3512543c36138a5fe660dd4cd4cd184bb43b6ba6bc0bae634e2fa9669304cd510ed5103f630068ff76d3375738de60a381842b421477e25a490cdd6894b2704125").unwrap();
		let tag = decode("94118ccc68de1921d480aab43d1ef0d1").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_52() {
		let key =
			decode("2cb374cb048c168f2e43597f028d9e73cade1b458284ffc260d4fc6b9011c414").unwrap();
		let nonce = decode("000000009fb909169bc9f4e9").unwrap();
		let aad = decode("0c7bd4f3a30ee944ccf9489181e6911684dcffad4593a9b65a67dfc80718c69b35897d01281016b7731e12c15cad8482e79458e08a755622e3f3f22a23ef6c8487a36ad1771ba06c641f06f85de0db3776cc6df06ad8fe3b4d60d58508de943083f17cbb9dc0d390ac94d8429e8c6fcfe063f424fbde0f62f6a7f91a626d195dc498a6e69bd93109c4e9ba13e7330aba456d710a4b0cc279d4045660406e26d61dff70d4a33c4f1052869f9248024e7a0f85f1effb32f6f7ccb1f860f3ef04e8f7b29096e6bcf9d4b3e0ce703e9bf228fdf515c2ff9cbabd16987be0f9babd3d8a").unwrap();
		let input = decode("39eb929482784b463546f5d84f80510f2019923d465b99d194246d68c7ae343f91971d8f7059cebb86aa5dd099289aa648248b8c5ca04e66ac5e9bf06776e3883495397618a0227f035666806e636836b47d3d2d255a49db79866cf00d9ddabda259c4f968a1e01e651c7811cebbee2ee71803ea1d9d23487eb221f2d9555756800aba5e6abbefd6fb72b3151cc99ced599cd86df2a9b1ce94f89f347eeb124d9e7f0d9cc48d3dedd819e6d3dbac57ecee199547b266116a2035c9acc4c8ca3271ac74952372897c4a5f2cb84e2d81817fec9d6774f6d8a5b2021684132db4fca3").unwrap();
		let output = decode("91ddadb86b7ebef798ddaa59da51d71316fcf6c9678143178227d778750dc9827fc6cc21e605c505023e6db25849df7fb6fc1ca4d223aa215f8c85b724643c83bf8218815a9f9e2952384e0ca6a80a3760b39daf91a3c6154c4728c2371fd181fa3764753d0b0c23808a82cd8f0497246e3a0f17f8906a07c725d2891ce968a9d432c2b102d85c05510b28e715bb60d0403a77490e7f18be81218bc4f39287b9bb09f50227dd2f55e4fb70c4438da8ba3c8ffbced87d90155913faa9979fc57e6cbeddfaba3d3ab4163c0eebc7d94279c27d3ed56338893dba542eaefba30f8c3b").unwrap();
		let tag = decode("8980e8e4fe796428b733f4f8e1954a45").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_53() {
		let key =
			decode("f0f16b6f12b3840bbd1c4a6a0811eef237f1521b45de9986daec9f28fca6485c").unwrap();
		let nonce = decode("000000007ac93e754e290323").unwrap();
		let aad = decode("13bfcc17b810099cda31ca53a1323db9b07633ceb2088a42263a4cbd6a4d47978776005c9a20203319c3a3ae434e9a26fb541047dc9df38dc36c095267272e203d0b24d119a70a7e96041b6d82b7c4d5570e1e4a1cf2f6e44ae63fe005a1f5b900778c482f7bd89e2e02305e35b8f61b7bb2c78a13aebfce0145d1c5aa0bf1d10d23616d5a3a446de550302f56f81dc56fe4f3700f14242688d9b92d8a427979b403c8de8c493a2cde510eaf6b285e6675b173aa0314a386b635c7577d5aff0d868a0cb3f73c8d2005f8c7c9dab5a060ef80102c9d4a4af988838afe87aff04c0689e8c3c7f9").unwrap();
		let input = decode("0530556424d823f90a7f1c524c4baa706aad2807e289e9479301e3e7a71f2a5e14e6232ea785f339c669af2e6d25f1d5a261096a548d23864945c3a589b67b09b0304a784d61b42b2419139485242e0d51fcbe9e8fed996d214de8717e6a71f8987ccad65eb92e66707034a5ae38e6486e26eb4374c565aad5df949dab209f7f7bcd8eb6fc52761a26cfe5d01fd349e59f4042e6dbe6b232f9301b971dee121d8aa1e62d40f043a42f3aa859d867eb809b1ced5ae1ec62cacf94a69fafd0631a8b5dfd66d855900fb295eec90ae5fcbf77beae267a79d24081bb322d8c4e0630fed252541b36").unwrap();
		let output = decode("2c14c3931e98e84507c4c165c2ed47ad4a178f0e216cd7ac2453bbbf9f85dd06bd8ef54a9ff1fd3dd8e0cafb635d8f2de861a0db5b14d03f17aaea8c89b3010797c71c13a0e666899d7ff6e53c4f08be8ddb3e37688b5afa088079b6c7519b833e16560073e699530302028a3496e05edddec01a23a4c7983956250e8d9e616f7b940856955cde81c1efabf6b7b92f153d03f4cd17e7f7d2907670cfc84d45c1d7936775a3fce47968504278ffaecacea0871b227f250e2979516f6fa310fec0d8df1af7872e5a534e82870aa05f43ef0a455846b93ce938064fa33e92de262e4156dae56775").unwrap();
		let tag = decode("16c972829819b8fb030b2c5f40dab717").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_54() {
		let key =
			decode("3792943c0396f1840496917ce8ad89608385007e796febeea3805f3f4cbeccf7").unwrap();
		let nonce = decode("0000000023b2f9068b2c4c85").unwrap();
		let aad = decode("7eb6d7b7bbaaa3c202a4f0f1de2263767169eb4a64853240d48c0f8d5d31b08d5baf42977614a57aad99426cde76d242cb37d2956d8c77dc4fd62a3abf30e8ac6cd58c8ef35e67497022960138c57787818892460f3bfc16e37ff388b1edc6ce2bc53c22717edc7a03d4c78b0dbbe9121c7fd8a3e3993b87a4fe389bff13bdae3b349de0b6db561602c53f746022aeb4483c723b67825042f4af20b7dd1e6031cf54215266295c524ac8e1370424c5c5e607fb3e23e97c8eebe64656775edf616422a8b974e1acf13ab45c9a367a7dd9b2d62f48bbc05819b65eccb813ca813f57b22ee4c280dbb5a9d8d5").unwrap();
		let input = decode("be6b67eb943ee7b5c785cd882f653e73a8f75b4a41a2a7c56ae5a10f729caf39948fe48ad0e51240e2e7aa43193c7ec6ce7f4909fc94c9f99e38e6a0ad7e98eb29c5c2e61c99e9cbe890f154185cec213a74725d23c1a4e4d0cb9b1a36b78c87e5eee20d2aa29aae80d4759eb0c51c5dc3a95bdbbf7e14eb434419a6c88a954ac03d0c98739f4211b8732acd71c297f578b8cb64ccac45f7235ddc7f2a3f5f997525c1ed39dc550126cdf9cedaf55425489085e91b170be6205a5a395f2dd4084a3e8dbc4fd8b13252f7effae067b571cb94a1e54aba45b1b9841308db0cc75b03cfce4ddafe89ce20f2d1").unwrap();
		let output = decode("0b316ab2bcf5359900fa4082d5d253b49ad94b70e3fab544f98bd111cbcef6766cf953deec08cae1f489fe12f7acc0032db8a6b0c0eee0c206ea5fb973feaebf90f690e840094db5e13fdd7157ba127368c995b426529435a1bcdd1f14ce9125b8a0e4c96b6ec09e3c36a180adf81941c002d19c19d53c2009be803b987504606b7d43bdee5e0b32ff23c466b6cccfcd0d4e88fd1332e73712b5ab725c1a383e584f34f80daff29d285ae5e43cf1d0cc7a828e75c25daced3a581a93d7a50f313b33f38dddfaa23cd5b9914797db820ee2400d52bf5fa982277fe9b5881ac42981633b3957b0e935051828").unwrap();
		let tag = decode("c549aa944d6d97e52e0793ed572682c0").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_55() {
		let key =
			decode("fe4be6054773f634356ac328591fbc6f833b0d1beeb38dd5b6feb7481b4489d4").unwrap();
		let nonce = decode("000000000b3f16f898a5a7d5").unwrap();
		let aad = decode("834cd775cbefe4b33a3ca53a00c06a3c4a666983e4115a029f15729460daa45d1505e95172d3695625a186b28b8be173a925af04665f209267b3c5123e8be13da447ee1ae856bb0925f35aaa76e04a7bca8460f76c2024de2149f38a8cfba81694b854885d72568105571b6b213a0bc188a44cc7fe13153cbf261401b238cf12a95e23cb56f240114f16e2f1e3a514615aab4449c0c49e4d900b0e17d1a8dabb53d43dca32fa052d576b73dd9b40856b515d6d7efc2a5c17e0ebcb17bd59dc86f22ce909301a2652f134e82ef0e4519487ed12d51536024f2ae8f75d937c42d003076e5dea8de0c684cda1f34253d8fc").unwrap();
		let input = decode("76ced1ade6d1ef4069afddb32e7432d4ff2fd06685121f7b16464e7a72d365744f547d2ccf53486310e38b42d8bacaf711e54c5458d2d68c4dbcc8de31ab6732f4430e88a64565f5b287640775aaa2af1cc461d3e415bb275c6246b1b58517aa72667eae291a2982eda175d1b22c5a58e6fec2b3743d55712f201ca24ba5c0ae8c25724871b2ec2fb914a8da5a52670ab9b43a83b8568ce74db5c634061cb80530c8070c38b8f48c33ba136cb9f2158ee7eda8b65f2192fc94d1291f182f101795b7190c74b319d2d3e02a97c824d9c9471a83797e4936310b207e3a1e0bcf75f7c3e3ee48a747641cdc4377f2d55082").unwrap();
		let output = decode("f8defb6fe95dfec499b909996a1f75a198a90e4d6c6464d00a357a555311c42fe92dbbc4b79c935e4f0b1a95e44fdbc1380bebabca28db4dd0d2870daaafc38ef27908c3509e945714801cc51f1a07b2430c74fa64f2a7c2f7fd1551d258c9c3be020873fc1bf19f33ab6c660911dcf2317195d0efee82d20ec26d22611f9cf86c51a64e28b3a1f344500018e0855c88dae3c07acaeaa10b60388484dce93e16e6e1a6e69e899806648a92568c8780e9f4baacd98cbb353ac2f908e775d92303cfab843f15be0e0c322a958802fb1a60fcc7631f151f4c2b8cb965d2d296acef250275a2fecc0cea803ce7c058b12dd2").unwrap();
		let tag = decode("baf9a51180f172e5c0cc2c946ce55055").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_56() {
		let key =
			decode("a288b11ce5382ec724ce4ab2d7efa8e777e91ebd04367935e15f9dac483e9596").unwrap();
		let nonce = decode("00000000874144dbf648b325").unwrap();
		let aad = decode("04abe8588c8c8c39a182092e5e7840442bd1c1149da102c4ee412bd8b82baa5087ef7291b5cd077c177c42770b0023e0e462b06e7553f191bcb0315a34918dcdbffe2b99c3e011b4220cc1775debcc0db55fa60df9b52234f3d3fa9606508badc26f30b47cdb4f1c0f4708d417b6853e66c2f1f67f6200daf760ceb64ffc43db27f057ad3ee973e31d7e5d5deb050315c1c687980c0c148ee1a492d47acfcd6132334176c11258c89b19ba02e6acc55d852f87b6a2169ed34a6147caa60906ac8c0813c0f05522af7b7f0faddb4bc297405e28ecf5a0f6aac6258422d29cfe250d61402840f3c27d0ce39b3e2d5f1e520541d2965e").unwrap();
		let input = decode("4c9195280a79a509919af4947e9e07231695fd7c5088539f23936ce88770ce07d9ad3ae4a463b3a57d0634d3a77ceaadf347a334682b04be8e58b8e86fb94a1f93255132b8cdb0df86f5bea354eea4e8315fea83e3fdf6e58aa9f26e93caa08e5e2551a94bd916a51fed29ec16f66800cda6a0aa24ec308bf5fb885afba272685de27c1edcdd3668048ef07b06e90d464a8aa28664903cac45e154e8e1e39c257e1ff506b9d95cef4f300bb73b899e7828602c3c1d290b8cf55ee5fd72ecce9e6efc9293aebf674a70e2a7673e75629c12950622dff71d3ec0992e57776c788c6927d30b4e24b749191c3ce8017f0ada6276e43720").unwrap();
		let output = decode("0afce770a12f15d67ac104ba0640aab95922390607473cbda71321156a5559906be933fb0980da56f27e89796eaa1054f5aacf1668d9f273cc69071b9e8e22af6a205a6a88f7ad918e22f616bddbb07c78913c7e056e769e6fcf91c7600c2740212e3a176e4110cac9e361a59a773457064d2dc652dd115d04f1c3756c0e1d39f6737a16b4508663e310934c49c58058b3c7b9af7bb2334c8a163608c42499658986927cda365e2aead3ac29de16e47e954383ea566f8fb245a4e5a934c767bb3bf7e0eb8a477fd0e1f61bcb238462a0d19c5cea9293ca58ade76829413216a7882cd2846323046694f78cd8b0347792ebb75abdc1").unwrap();
		let tag = decode("eb9b2ee43e9a3ae1e33561800169d868").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_57() {
		let key =
			decode("65b63ed53750c88c508c44881ae59e6fff69c66288f3c14cfec503391262cafc").unwrap();
		let nonce = decode("000000007f5e560a1de434ba").unwrap();
		let aad = decode("51a3588398808e1d6a98505c6e5601ae2a2766f1f28f8f69d1ccbcad18038c157b41525be58ae4527a073748b7a04809e52a5df0c7988417607738e63d7ead47db795a346b04e740186e73ccad79f725b58ee22dc6e30d1f0a218eda1791e2229b253d4ab2b963a43e12318c8b0785c20fca3abcf220c08745d9f9602f0ece544a05736d76b12d249699c9e3e99f3f13cf4e5dc13a04125c949a5b30d034b23cb364c8781964bc6c30e5e5ca9673d517ef5f35965d8a8cf1be017e343df97b6bee37b30638b154286d1f36d2f9a0eaa23cc484eac5a05b15d9efc537d989dbc8b3106c0dc1a56e97e6aec2eff54a82cf7ae9df2af46b4c860f83").unwrap();
		let input = decode("845ef27b6615fb699d37971db6b597930a7ef1e6f90054791eb04ddfe7252b5f88fd60eba5af469bc09661c0987a496fa540621afeec51bebda786826800943d977039dee76235248112ff8b743f25ed5f3cb0d3307f5e118d84fdbb9c3f5531bc177fb84549c994ea4496c65e5249da987dd755d46dc1788f582410266a10f291c1474f732183a2a39afe603771bb9c423fe3e8906f2be44a0c9a7c3f0ceb09d1d0f92d942383a875c0567c7869f045e56dd1a4d6e90c58d44fe0c5760bb4fd01de55439db52b56831e5a26a47de14249453a4f8e7da3cb3282c6622916197ebfaad85dd65c61e7d2d3ba626276366746f396394c1bf75f51ce").unwrap();
		let output = decode("027b14197b4012256b133b78ddc94e72fb4d724fefa4ae329f5a5fa3fa784fe6d7e1e805e3f7a75557de64de506d38237b467fa577efb59e7cfe2356bed6655c5aa4e238dcfeb75c16549a0917268768a96acb5e20546a1fb7e3a7cff887f49f2cd7a135f72a98a779150f3207bf733e88861fd79eadbf77fa3bfe97bfe8b6a991cb3bcc2cde8287f7e89384846561934b0f3e05e0646e0e1907770df67a7594161a4d0763faa6fa844080932159999d528ee0558710058ce16f97d13ac9fd9bf5044191188bbfb598d0fafbdf790b61ce0781ecc04218a30ded45efd498cc9ba03562ed2b4a993ee98876b3ab7a9bc07829f1c4ca6ead98c06b").unwrap();
		let tag = decode("e0bf9b6837428843f5a233ee5ddb8a1e").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_58() {
		let key =
			decode("4986fd62d6cb86b2eaf219174bec681bebcdef86c8be291f27d3e5dc69e2feba").unwrap();
		let nonce = decode("00000000d08d486620ed2e84").unwrap();
		let aad = decode("7dd3f656a03c001b45ca0680bc3ac9d68c6e96b591d3c69eb8c65e489009d845cb331c98b82e627e06d5bf01e74c573df268c2386f12628c019951d42f55991ff20d72a7b2c45f41d0be7af428c92f324aaab8df70d900301cdf09a3d93eb711c919d34a86fff9cb078322ee2e0ad48dbdf3b7884f0f2dc5c36262c59bcfd75ac6200f59c6fcd0ce10ff5005fef5df8f0432377dfbfc1db8f559e27e1aeef3380ea3864867d36a25a18654779a751586cad3b8a46b90864ee697b08605673b8d2123433c020a21c4db243dde2420c12fd4d54a2704a0c8c376454a1b5e80fd6db89aabd56d9b421f29649e474824dfa56cb5c673c504d10be52b53751709fe").unwrap();
		let input = decode("3a22ad5de387db4fdd5d62a1b728c23a8dddc50b1e89f54f6198b90499f9da3122ebeb38ebf5fdfe30309734f79aff01e3de1e196b35bffa33bae451f31f74b8aec03763f9e0861a34fe5db0b40c76e57c7fc582bfa19c94ee25b5e168270f379bf9f8a0a18bed05de256f8f0dd7c23ba2ff1c7f721409462f04cc611ad9bd4c3c9acf30742acfb9518a6375cbb15d65a1bc6993ea434894f93d4f6e05996ebc1bd56579296309a2c6b8fde95072168b5fd31927c4c0abaa056bcd16221d5f220be47591f43255013a262dce439817f534830ba82155347e5fe3101f8011b89365a6568214ed0661914e8cb3431d6c8f2347dfc1209a3eca4aaf0a111f47fe").unwrap();
		let output = decode("c40180afd53001663ff4834110f56e6b0f178cd3c0e7f7de5d0089ee41d8403ffb98e84922706544a344d7e2625b12cf66b9c966f9f57d7b94e3e4b34e6f0aaed1763ce012782e2f5e1682e6c343fc7961fedddd0919d0b910e9923c17e36406979b256b85aec24ee352f03b48c1302eab419c83dccc5372cc059e9de596224fa70098eb32fc9579e97917b923914fa2efc30ab29b457bf14e45583b3771486bdc0876f3ea6e1a646746c4f8c5cb2641a1557c8473e6ea67d4811a67485ae9a678ff3a2408ca845c3b51957e189eef47dfc1d46bde4b9d754d7df13f828ddadb06e4ebddb5f0dafbdb28de4c5e6078926f20cdf9e97ecd58e309e640f74f06").unwrap();
		let tag = decode("2e8eb9ff4467c0f61c2abf6ca10893ef").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_59() {
		let key =
			decode("7d28a60810e43d3dfa32e97c07957ec069fc80cc6a50061830aa29b3aa777dfc").unwrap();
		let nonce = decode("0000000047738ac8f10f2c3a").unwrap();
		let aad = decode("324292813b7df15bc070cc5d8a4bf74ead036430be63abc43304cf653959a24a91c7de5a671c50fa8a87e21bb82b069999aadfb6895d8bda4c3083d17b8ca55b9ab1511ed8c4b39d8c28c11a22ef90c08a983e3fe2d988df9e02b16a20b24f39ddb28429625f511db08298c4dc321f6c268fc836a6191df6232f51c463a397a8d8b33374abe94e62c0f5c322387e1fc4a1c1980a04a1a3c2c31b32f183a11c3268c6dca521149dc16af120a78be6627210e8ddbc44472bc24d66ce3681c7579b3d9a425212a704a4f5105cb80f0d18ee860953d10b59c114826779bbc368d7a0eece9f223e47cd8e5fd453607d101d9d9c2bd9a658d6520b87d7b4263f6d845a524a36e4").unwrap();
		let input = decode("b50278ae0f0fa2f918bb9a5ed3a0797c328e452974d33cbf26a1e213aa20c03d0d89490869754abf84dbbe231d7bccdced77d53fd4527356d8e02b681fc89a535ae87308bf7fbc26197a5ea85bdb3aa033b8da5cd197ea6d72f96f63b03f4ecc7adedf399a5043776cdb32c08f30b77f34df85f8adb8e02649a04b020b03e17d445ca63e4ed73ae432c481392e031eba2f9d2f7f981d1e50917822bd6ff71c239d33444ada3523a59dfbce5457eadec1ab926c9e6c5299c7521e3f204b96901a712504fcc782e8cea80ba12a7f7e71cec3d0871899b6ca059061da037715f7d13fed01c9cade1e687b4fbb1f4ac4b040db3b43800f112fb900e4f772d61b921cbce4da6f").unwrap();
		let output = decode("2c217e969c04740a1acfa30117eb5b32dc573df3354f4cc3bf8f696ff905f1e640f3b2c250473b376622e0c9bda13b94640521be1ef0fc660b4c10dbe2bfc093030753e04f6aaecf813b43b61f960455974b8bb8a9b461d1e8fd3802315e863c00448f24dd38deb90e135493274eb14ccbde15c50dcad734ed815a806be6622492a84cd062e3ba567b909a205a1d0d2bedd40169697d261c7b6c2e0b1f069853fd470e8f364a142c386c439a6dbe192ded5a3d0fbf73799f588c59e58c60249d980ddcf0d9693631cd9b3f972509c3a77123d38d9e267ecad06e1208e3f1c0a69fbca7c3bb1a48fda19493d0f8f48398820057b94120f3ef97d87e9e8a1b301a2534c68f").unwrap();
		let tag = decode("ce507bdb0c71f8e89f5078495f7995b8").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_60() {
		let key =
			decode("a76e9b916f5a67b78a5949651c8c3a9741a1bc3c41cdf85fd2c8f3e9a0616098").unwrap();
		let nonce = decode("000000000808da8292dc14e0").unwrap();
		let aad = decode("6aeb7031e4a2e23eea93f05fdc562aa2bf43b8998bea7344377aaddc60fbdb7bcb1491d379ed0cb613ee757cfb66490db61bb431d2fad34b38ddd55bc5b22aa6c4773b9992f34b878c5663f6e8cdb5f80a17f4d312bf342492e48d1ce4c6d754076a634fece61500acf8168d47381af4faf980c6cac2bfd5da8c09b6edb0f543bf0fe02643e38d73fa37d8ae87fb66193f22e57faf4393c007d48c8631a685d520578f8f89db684fb371ea02f3a58b1e2168f0216321139472e0d03b6d90ba8aab65402e1c1ac4f9172a60e27e3d997b9b05e2f672120d6c87bcafa6d4c9b4cf8ba8a82932d92840368fc53dc5b48526103dcab5f1531038aabe89171327ac559b98a3cf4ea70bf051").unwrap();
		let input = decode("9c149eeb09345c3c22462b03e49eb4dba6bc98b269b1086d752bcd8eea53b8977b238a04a994baf915591686baab90b79a3bf7d9adb2c6c2e31acd3e72f0813fb745aa5fb2e3da408f78001c9c09bd26a1a2646011b6120aaa2bbacc4a16c39fb5257b9b2ea2ad8bf70bcc9855cf11841116c2767310cf3cd49d1aa44cd505f079761e064d5bc7cea4a7173b086882a77d3fc179efc86fc4db8a373491d2ed81eabc63c950e832db17d09f474d4ec46bde47830caf26fabaa0372b81fccc449c0e19ccd630caf693a7b43bb1c408a54e03f50c44280a05ad89fb6e8f01d8ac278edf556e5d86ceb4b614fb2ef133819c6e1ff6abb86c54a135256204b5cd400b93624d3932e7c2b046").unwrap();
		let output = decode("9c3faab9261a63cea9477b3269007283995b06ba77ef83d9e693f7e4ee9855550eef94855be39a7a435b6a3584b202973777c7b2482376ba47b49311947a64983b60236756ee4455d4cfada8c36af8eb06b06ba2f6b79ffb1185c89f2b2a831cfaa3855fc1841d8910908be5078352011168a67d36372d851a3217cabf593ea462dcd325cf9a4f67e85418fd5c924e9b92ab026cbee4e7ab1067066cb5949dfc699a68fe539e1abb13cec33904e5207e6963d24f5a0b770613b8b00014e791bfff88f9c25ca126127a2f8d1d1e9794efd28dce98b53e228073faae8d5047530d502184fc341321c3f55fcbf41187fc31262c325b97f519959b6a29b36c71f76f60196bb1457b77c8bb").unwrap();
		let tag = decode("73b00b1705602479aab944dcc1b282a2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_61() {
		let key =
			decode("98cd2477a7a072c69f375b88d09ed9d7b9c3df3f87e36ce621726f76e3b41a1d").unwrap();
		let nonce = decode("0000000077d185aaf715aa48").unwrap();
		let aad = decode("f5bb1496052a4361dddf72a288e36953a3d815d6876c013f1d6ba839e127f721b052b1f7d8ca20c7dc0386a7d459ebd7eb9fc8cb08941e6ca9ddb980f3115f65bc1928a414d441ae71dcb879d5bfe0cde0562bc37f8fde0d5291ad405c92fcbb860c43b55ac0fe663b54b3d0616aca13a5c82b7b5d34125a05c2acb5530141030e6f2aa0c8322b2c8fa307e7518918e550e9f48921c6168f094d8758e16b9f815fd0458095c4143f0922adb1840d0e685636825a9c90ee90ee537f4b8dceecbc4287c82dc9a00d7e51671e37ea284ee3ca501b1b2596459d3f592f70186f41125739e342c9f6be9241973b1414dfe5fb8cba1af82e679278cfcf95420df0c5364af4d7e72ad57d5c871fcbc35462").unwrap();
		let input = decode("42b31eefdacab0f03ef6060156000c8195adb0976cabbe1a42bfcc09f85659c60b98638401f2d2e2facfb9a97a62926bb0cecaf3af0180a01bfb6e576babf7fc43331937a92abd30cddfa3e450f895e9dd914dea3fafd759c136d685310ebce28ac0613ccdbf30115946c9634b67510b77d0e37f07714b2ddac9d7095b8d4bd887c132c4a9127eb01c8dedb4c39c87b98a741316656f9a8d5a5b0c0ac84789aa2347a5f99ca5ad55cd1bcf98f703eb4b00badb8a8555f38b3b368db8ba7ceea94e8b219f51edce75d84166b5602156ed5962a93a51db73c59d87e906179d7a74a2a2a69d8ad99f323225c87e475d3f771b4a203a2e2b03b458401044649fa6536dfab24d7037807dcbf6518e6578").unwrap();
		let output = decode("7a3bf3e3ad5ae3ab71fb1f7121c3d8fb511099484b50af7ca128ee0337ed4b828dc4cde0b88dc1e8089101fa82c9beb3eb48fdcf0f5b16da441f5a3fce9a590022af95a94aed6a3e71e505f60f303c78c356f274ea85a55354078530664ecda32c80e77dc20974b3b38f4825b8fbee8c3970769a2f42c5181608a8d7d76ef4d093961b665ee42b9708fcafe2c82d3a307173e2a25ad2528c3bf83352b9265e45b70722d7cf8c9b80826d21335234ee3db69d0d37871c83222365900c96c17a7e9f5742d0bfe383be24d0d44590d4b0f29f7abe0c65daaffb968b3f2657b1eb300534eacb52ec7a6b6f9f57a50a91b1799f491361cf613c934b7f520dc4eeeb40ffc45e10be0a95e76f366d4eac14").unwrap();
		let tag = decode("69302888812eea030d621b640e7bcf7c").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_62() {
		let key =
			decode("2f0f4631ab1c1bcf8f3ad0559c818d50e0af7d8cd63faa357f2069f30881d9cb").unwrap();
		let nonce = decode("000000007d0ced2fdb1c9173").unwrap();
		let aad = decode("1ccfa1ececc8de1e200d0ecc19dcf67b7c96bea3a282c2bccba61035db5c14776387b8b8f58e5757deb0129d4e5e315f64df354a5985d2e47ebbbeafe0c914f7cf1d63dd0311ace19e69a8b6ff0ab25cc8df0408d22132205e89e5eb679268d82b2913e64e3f885bbf4a6d379b760b94590e3140dd7275ab4713cb56d0b716e2718f11316640cb394802862d39e77a46d0c065af3caf7dec14e887039d8aa8c3d3a8ac1ee06026f49d00b2f59d971b54735e95a51f199389a93a4fc24ebaba1f7a2eef7412f61febf79084fbf481afc6fb6b204084e5ef5df71f30506459dea074f11fc055cd2a8c0fc922c4811a849984352a56a15659b7d07a4cc90b88623638ea00c4c8bc13884df2237b359f2877aa41d6").unwrap();
		let input = decode("6516ba1d29357144eebfa486d21decf223da3aa76ec29bbfcbe7f1eeaf4a847710e5080177f7e5a7c8b4752c219b1cc70aef4db861ba67d0fa6222d9f4a1dc756a0ba44e62906f9374a960c16198866d867854d88f528a60e212eb91645787e75685b2e215c0a41990abc344a77236ec0186ba63a664592938cc5a8ac1d3eb99c95ce00e19fbe249263083d85b052d48bfdffc01585dc57bb2a2c6c4a819604c1ec0548c6f0f78dc05e4418b36277dc07233c7532f9c289d6aed0cc6bc7df4fd0a536c497b982e2dad2c30d2db1c6545a845c5dfa83a4ac49ef06fc9c919079d3e299e31b5c3be370814ae5022ae469d3ee55246a41bd0dc4e64351cc38c3c09af0a1aee3b388a6892deff0df3f93cd92d722b").unwrap();
		let output = decode("e580093789ba17ffb46672dc326f09278aca08598d3e5458eaa53e6ed45d5c71a396e35b5ea3fe7b7c0496a734d24f1c75420694be2ff095d5172fd3407794e4b99fd7c374fbe8d1564a048614d3f355bfb5866de1a53e1a51f9f5e8312253cfd82f36efaa1898c850ca0d975ad1e8b0d9597a5a9e6516fe2a3c92efb7495557a8afc3da15b0d3e2ba58f612519836946cf2d15b898320d16a026c8c00a1be2e35f0ebe68f28d91c6c45d24c3f3c157cb132fa659b7794df883d90741fa2d2afcc4f27858e13ecd41b154a35d24947ae7361170060c107d8ecacb393ea67104b60457278a392fdf1794bab97d3b02b71a4eb015eaa38a4b4c944c2bc7cd5e329da4a1ab2937a6af81a6caa5fce752331fdefd4").unwrap();
		let tag = decode("19bbacfac768bb0ce71e39c5d4d3e9a0").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_63() {
		let key =
			decode("a48b9b6df475e566aba7671fbd76772cb0eff0b12499967978ce3e25fac92feb").unwrap();
		let nonce = decode("000000002ccbf0d6c40cb302").unwrap();
		let aad = decode("1c2503d5aa1aad193f0da12874074ea0432bb76a61cd43a3017061514da0759846a0f3ae3a49fdb0b6d29f713de665beacb6568f2694112ca380d13f3c1698316866a7a7f87f1d7503a92176ab84fc08977b46ba664508a858e7525753c45511b3d2f407d5e993c6ede77f13d12975707e5195704970a89f71fc30828049f92f944f3aa93d6a5297e678e08952919beb7eac5919df1919cab3c3da6aa696a1eeab6371f310f7e81143e7d240b0213ae554524b52000306160dd4877bf13ba0f13bbe867da7c7d707f31335eef4cd942938ac890a0829ec66bd30ae01a2188a6e5ea0f17cd7dc875e17f03c0ab5dd18e36db8a1fc1f72859ee046b62368f168b3bea2234e0432c07b7d8e1b9277f21e692c513b9e816e6860").unwrap();
		let input = decode("09da1cacd001dce4f7573a065a4406fe0da04ab367a2d87780a2762e168957a88d3fa78f0a4b6978d449026e5a801d32884b6e14fdaaaf864214f928ebc03dead081fee96683ebb032362d5088c4c2a3b1e242f055f2604919f4dd551db777a258cf9da6d95a2bde249247812b9efc7985cf08707620808524d6dd3079b0b63bf0f71ea5de834ccb8b7c6a97125fd6ca49148e866d3134bbf1d8a6b714e9a80fe549c8bfefe342f41be2ba2300e0028f78cefab65274632dfdbe70bf7d655ec4036df561f2d4fc4d56a482bbe2f9f2ae279b3aa216b39afee75e53602de319484db89a51e844f38c361634e474f8f1f01c340f3f3594860d671346449c6d08ee38de22d246309bc7e4a252a29c86aa6d94b5b4fa58904c70").unwrap();
		let output = decode("7d35cfe4be56bd6e0e09dedcd01735b915bc1891a4d1f6a541abc4bcd0ebe89dcb8e365e5813742e8ec65777b6159422fada747da99394252baf8a046fc1b60ad79755f545f4448627b7acaf403000894f5641e78d3f946dfca29ec617f0660dcd6e8d8827e67e1022a245c595d86e60fbd176bf721b171bbe5ecaf4ae671b9f3dd3920146e6ad431bd8fc431820e19454b6ca209723d80fdbee187fca9c937c979206ae97be55f6ba7366a5608770a11d537396485eb0a66586385f4d4cf3905d1fc90831c3e136d5d513fa22be285193142994a3ed477145bacdcbdd791e8b3b88b0d4f1d18b27382550a818c4fd8884bf36f677c6c3ff5677406e510911e696af75e5b3f859bef699bdd16e6215fdb98d874025eada50").unwrap();
		let tag = decode("0fa4cb2bab84336409aa4349ab99a8bd").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_64() {
		let key =
			decode("923d4b086b9e43b986f7b65e4cea6113a3d8aabefa89323c5e4d5b6f158bb7e0").unwrap();
		let nonce = decode("00000000a0f73297b87f5deb").unwrap();
		let aad = decode("c853a8b39c0dc597d562f123cd221e4104b65423a062a4f4ba890ba344feb84290f61817e23330c365f58c3583ce08360d3c1171982ead5496d525ac878f23a57480a6ee39d4e65afd6268245bb982a2545fa1195427cdbbcd404cdad5198f55cce2a5a028fae435f71b15921d066e8d43766c32b2f2c3f57c0674e129607dcd3703eca529414adaee79d81fed432153cceb6f3fc53404810d8ec878f7d94be5d379d0e0e1aa9bc404b4b5d396038a9d76a5ce53c9f3759b8e50fb331858ca58cee81bfc3ee58baef5d19c402a3dc8b36370ec1ace5a4aa2527fb94b4f933a4ab8ccaaf6a5af5a779eae5667c2a24ab027e781c8d4f30c377aa5885a2fdaf6507d18cd824a847c35368b4ea984d2c3c3824a5b8ba3042e1852504a21a3").unwrap();
		let input = decode("21435e8d5c8edf0684f58c2cba4070c10b4801adf46b6c4d322eb3990a38a9ad338ad704b9df6597f3e68d66cd5b56290c8466db2231e56d6bcb9c44e1bd081f42ca2a894dad369df2bd0d2c63d6c881732d6ea22bb22b5bc9a62eaffa1b094d0845f6b966d2cb095e7b3b8bcbc15e707449d35c8df4aea30c3b7243e977fffd59c80f1c5c9af4bb5a54b9c786fbbe8d21b2b906a87a786caed841a34a3e0cc0ac3209d83c58afba19edd63622dd261532d2cfb0b49d527d8eaa0887a087f5129d897f665264b229f860363d71a88b7d49c8dc6360182b357b0662391bb41337f46010ac32b9fada2d60a2efcb99365d3b27b7ac396900d1c821d0df8b86cc9cc1f2673259a33efea610bf8e1d00d7e9db2afea21da8f58c55f799999d").unwrap();
		let output = decode("f2e21052eebbb86a4f5e803360855d8632aa727dca6f5e79dd74d7aff106e442001928d113005b030f8446f8eff2ee951db663978abe43090dd5ad2c51ba97a0ecf988c607d95e486d02524f690fa3c28d5c48c1f75c1f555e7b43fe7e46f2ca2b9fdb408ec4ba18b6cdde2af673183cb7b1a3c23ae77eddd4cac75e1ea14743fc571f8d31ce2e96787524cd48aadaa474181c096a032184574ddc25a6e0ac8441c212bc36298708e33c963ae931e6c6241d1affeef7b6ef759495df44b6ab647447693cf703569e69aa72f1def9a342b8978c1edea9703a421ca75b92cac4de14b88c693200022b8a2ed22b1c4678b99f4d695e080dd1196d7168e14f0d0f8ff880d742e97b9f6d00af1f7118e10b77c5ef3ea6c52f84a20fd6ea46dc").unwrap();
		let tag = decode("9bd8b7743c056bb2334833afd6143e18").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_65() {
		let key =
			decode("df73adab2768559ea983cce85453fe81d79be3b3c57f202b31b94d6635cf2e4b").unwrap();
		let nonce = decode("00000000e7a87e6bf6b5a354").unwrap();
		let aad = decode("f833e5ab4f8bc89167f80f576b1d6b22cdd0e30721f5f735799746cf645b6eff531d4c7b03584f3dfcb73cbd35ac42736216dc7f0de098a4f42c61ceb4b227ee288e47d697a0a76afc762f084e8fdbf9351c28340c324771c109a469341ab10ca10483ed2af5e878d7d3dc2bced2f72da3d1a25852b103ee9878e8158eb4309c1ce528f3a178ace153b6d3ae0af0d577cb3cb1540489e80427f792217ad8a09b84f027fca7ceb651b4264e98e94b4cb8a37b133390897233e8ba9103628d05b9609e8552c4a4b11e3f2fa8d56af36957390e88cba44656be3edace798cf8cdf7771bac338a256bc3cba6df97728f222f423ca7c6d149c9372d66163a98f79a234b00d4b75fb2ec860dcc2d1998105e4b9c01d68f079f3e0aa21cc534047fc7b858f8").unwrap();
		let input = decode("0032a37abf661faa18c587fd2aa88885c061deeba81105dd221969bed5d59c7204b09b1a8c4c8de3b9f748c7fc70626ebeaca060233a57b102221b1bf0f3d9fdaaad3d2b1439c24d08f9c67f49f3c47128f92ee530abf4c4f4573bc60ae4b38109f55bca3ca9e1ba9f9fd6e34ba0d174892977a53356e1f5c88c614fe3ff3b3dd0818e7a2285412e3b37444bbe8a80942efcfd03958809a6966cda9430b2f0c9e552f4bced6e19eb3e85fc5758bd7b588297ccbed37ed94c3adc8c08ea8b058462aac9d57a939ec711bc4ecfec944d2b653b7cfc7b02a65d7057c9fdadd51b9da8cc4a3c68dae9da8b9c5319c1a2baa3d6c891c5ac4a39461484b5a01abc64df447ada24c04a4363e605eaccf339a9aa515e724206206da6d22bbd2f52e64cd7c895").unwrap();
		let output = decode("b842eadfdf431c135bd6581d3eccae54e2267d8890036aa33dfe2d2d9715c44625441210a3a0d666d708d30588fe851ec36e10d8fa3584ed77b095149494b7c54379d62c8935e1d2b9a8f47e4759ad0b3437fdf2cc2fb6c5ea25ad10e0bdc9dc5b0517fc237eb783cc461c46665e2b1d1a5b8008dbf409ea2a63fea0276de23a32c99d92a498807a0f95e208fc6262321a78aafaf0cc3f833fff37bd4efa66f6023a25cdc6702cee3912799563d908a5183c9956a06aa71085d855dc7c809ed6e2889592b361ab3ab39060f8e419152187a794a19c2a1128882201900ea2cd597860674bf78d9720643df8701676718fd201baed4935a88e50558daf86edd08a9ab227ac7afae55c974b68de8dacad4a4d79b13ed6dfe74017a4cb9148e033436fb6").unwrap();
		let tag = decode("ee1ec36804e1d5cdbddb52608c711fd8").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_66() {
		let key =
			decode("55a4be2448b464c2ea52a2f2664ed6aba865c14ea1fea77f4689331fd105c8d4").unwrap();
		let nonce = decode("00000000db37c0a405b4626d").unwrap();
		let aad = decode("fd6a3fdd879f8880843eac20ae01c1b9dc3487d270a806572088ef2ddc1f1e0de495e71d4813bf5c501ad31e5d791c4b5b3a0a71b63fdddcc8de4b056064ef467989ecccc5d0160d403bf3a025d4892b3b1de3e062bc3581d4410f273338311eb4637529e4a680a6e4a5e26e308630a5b6d49ead6d543f8f2bf9050aa94ce091318721e1d8b96e279f34b9759b65037bec4bf6ccda6929705aeeeebe49e327e4d7a916620c9faf3765120658af34c53fbb97ec07657b3f088fcbdc401aa7949ddeda34d885018c2c23f4f0bb8218bf0d4fc90643658b4d8834f4a8c08e590c2a790995baa9e77627c342d283e454f84fcc05be15e9627a2d9be340c9d72f222bbdfc47905f56616cd9f936d49e4732f319f020513340fb8b22828db251b102b6b137c9533936d6").unwrap();
		let input = decode("d266e66272e5d3462081b004cb42429c8b9741e9f678153754d726f6f9aa513464763c5e793b482fe512fece97585f1426120d4cefb3d0a8cc0a8db4bde93fc72c78f44d4fecca14650c660d3e285b327e7cdd813063e7e867b8a2d059a41bab70432b7f857199894da90dca3fe5272bae1ec694a1a07b60b05df275784d4975637e4673109f3ba846dfd1a048b202ed8e89973be608b91ee4743b1e759900f1443038951fe6189e806638985f3c16338c3c60695df58e621154d79bb973859c4558e9dca90470f77c73f004443ad5db0717abbe43266f90e57397b83ac34d1fef2e897e2483d5bcdcb627abd64b0d1aef525835f25e76d6e9158232cdde6dce970b59f58de8a98e653be32fb58edabbcefa5065d73afdf1c9c4fbf50c1022bd22bfcb98e4b422").unwrap();
		let output = decode("bd11ed07b7b4b30eeaf25d6a41a549cca0a5aee71f990ac566a37265d7af2ce3c03703427ee0b2755c2bdfc29f9d826aec6ee4ad28af48079ac23db16580b97424f3a4e35cc23625d39f95699d9ff5143e9a2bc26fcfee4f125f5aa2d968ccfc2faaf9db3c28850f6757f735cbc50c94c498bcde4f23bffafa8dd5f70d1a011e35eb26e905d4e68848fedebeb197be595c085ba33f11ba8398258445051751888e9bba111f800f31b37c447074ca6dce6d54b4dfad6cee5138643d4f6ac045e8047248924e88ea4294c7878bc22c9b41924ce301f22693c33733107bf1ba85e34806c5e4366ea66fc52a5f89dd9bf213239158b3d4d2600dde696c61d76c398b9bf10de9118e812e891c8f3355c0ecc6405f79bc32a58905e37888a1d8395fbedc3ac54eca569f").unwrap();
		let tag = decode("296a397d280d026fc3627f4718971be9").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_67() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_68() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_69() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_70() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c2").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_71() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_72() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f3").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_73() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_74() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f37465").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_75() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_76() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a84").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_77() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a8413").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_78() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a841386").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_79() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a84138648").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_80() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a84138648a5").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}

	#[test]
	fn boringssl_test_case_81() {
		let key =
			decode("c66e89fbab01208f6a60847f4f34b38d27b554c119cf8d9e0b118aa7266ab865").unwrap();
		let nonce = decode("000000005d9856060c54ab06").unwrap();
		let aad = decode("85c112a1efe0a20ef3a550526a7afbc98f6367ebbede4e703099abd78f51").unwrap();
		let input = decode("f9e3e9b5ed07b2080db8c1ffc37e4a6cb3cd544608921e18610d00b17c6e").unwrap();
		let output =
			decode("b5cc754f6dd19ef2d66f90e6bc9a322ddf216ef248cbe76b5ab6dd53bc36").unwrap();
		let tag = decode("d3f7b9c295f374651a84138648a591").unwrap();

		chacha20_poly1305_test_runner(&key, &nonce, &aad, &tag, &input, &output).unwrap();
	}
}
