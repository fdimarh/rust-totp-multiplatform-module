macos:
	@cargo build --release --lib --target aarch64-apple-darwin
	@cargo build --release --lib --target x86_64-apple-darwin
	@$(RM) -rf libs/libtotp_ios-macos.a
	@$(RM) -rf libs/libtotp_ios-maccatalyst.a
	@lipo -create -output libs/libtotp_ios-macos.a \
		 target/aarch64-apple-darwin/release/libtotp_ios.a \
		 target/x86_64-apple-darwin/release/libtotp_ios.a

ios:
	@cargo build --release --lib --target aarch64-apple-ios
	@cargo build --release --lib --target aarch64-apple-ios-sim
	@cargo build --release --lib --target x86_64-apple-ios
	@$(RM) -rf libs/libtotp_ios.a
	@$(RM) -rf libs/libtotp_ios-sim.a
	@cp target/aarch64-apple-ios/release/libtotp_ios.a libs/libtotp_ios.a
	@lipo -create -output libs/libtotp_ios-sim.a \
		target/aarch64-apple-ios-sim/release/libtotp_ios.a \
		target/x86_64-apple-ios/release/libtotp_ios.a