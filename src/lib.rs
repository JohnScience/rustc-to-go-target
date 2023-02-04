pub fn convert(rustc_target: &str) -> Option<&'static str> {
    match rustc_target {
        "x86_64-linux-android" => Some("android/amd64"),
        "aarch64-linux-android" => Some("android/arm64"),
        "x86_64-apple-darwin" => Some("darwin/amd64"),
        "aarch64-apple-darwin" => Some("darwin/arm64"),
        "x86_64-unknown-dragonfly" => Some("dragonfly/amd64"),
        "x86_64-unknown-freebsd" => Some("freebsd/amd64"),
        "aarch64-unknown-freebsd" => Some("freebsd/arm64"),
        "x86_64-unknown-illumos" => Some("illumos/amd64"),
        // TODO: consider replacing with the following:
        // "x86_64-apple-ios" | "x86_64-apple-ios-macabi" => Some("ios/amd64"),
        "x86_64-apple-ios" => Some("ios/amd64"),
        // TODO: consider replacing with the following:
        // "aarch64-apple-ios" | "aarch64-apple-ios-macabi" | "aarch64-apple-ios-sim" => Some("ios/arm64")
        "aarch64-apple-ios" => Some("ios/arm64"),
        // According to this Go uses dynamic linking against C library (unless Cgo is used) :
        // https://www.reddit.com/r/golang/comments/8m4xrh/do_linux_golang_binaries_depend_on_libc/
        // I may be terribly wrong but I think that it's safe to assume that this should work fine:
        "x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-gnux32" | "x86_64-unknown-linux-musl" => Some("linux/amd64"),
        // TODO: consider whether the following can be supported as well:
        // "armeb-unknown-linux-gnueabi" | "armv4t-unknown-linux-gnueabi" | "armv5te-unknown-linux-gnueabi" | "armv5te-unknown-linux-musleabi" | "armv5te-unknown-linux-uclibceabi"
        "arm-unknown-linux-gnueabi" | "arm-unknown-linux-gnueabihf" | "arm-unknown-linux-musleabi" | "arm-unknown-linux-musleabihf" => Some("linux/arm"),
        "aarch64-unknown-linux-gnu" | "aarch64-unknown-linux-gnu_ilp32" | "aarch64-unknown-linux-musl" => Some("linux/arm64"),
        "mips-unknown-linux-gnu" | "mips-unknown-linux-musl" | "mips-unknown-linux-uclibc" => Some("linux/mips"),
        // TODO: double-check that "mips64-openwrt-linux-musl" is indeed supported
        "mips64-openwrt-linux-musl" | "mips64-unknown-linux-gnuabi64" | "mips64-unknown-linux-muslabi64" => Some("linux/mips64"),
        "mips64el-unknown-linux-gnuabi64" | "mips64el-unknown-linux-muslabi64" => Some("linux/mips64le"),
        "mipsel-unknown-linux-gnu" | "mipsel-unknown-linux-musl" | "mipsel-unknown-linux-uclibc" => Some("linux/mipsle"),
        "powerpc64-unknown-linux-gnu" | "powerpc64-unknown-linux-musl" => Some("linux/ppc64"),
        "powerpc64le-unknown-linux-gnu" | "powerpc64le-unknown-linux-musl" => Some("linux/ppc64le"),
        "s390x-unknown-linux-gnu" | "s390x-unknown-linux-musl" => Some("linux/s390x"),
        "x86_64-unknown-netbsd" => Some("netbsd/amd64"),
        "aarch64-unknown-netbsd" => Some("netbsd/arm64"),
        "x86_64-unknown-openbsd" => Some("openbsd/amd64"),
        "aarch64-unknown-openbsd" => Some("openbsd/arm64"),

        _ => None,
        
        // https://github.com/rust-lang/libc/pull/2278
        // todo!() => Some("aix/ppc64")
        //
        // At the time of writing, Rustc supports `i386-apple-ios` and `i686-linux-andorid`
        // but not the outdated `i386-linux-android`
        // todo!() => Some("android/386")
        //
        // TODO: consult with experts whether this is correct
        // "arm-linux-androideabi" | "armv7-linux-androideabi" => Some("android/arm"),
        //
        // TODO: learn more about `arm`, `arm64`, `armeb`, `armebv7r`, `armv4t`, `armv5te`, `armv6`,
        // `armv6k`, `armv7`, `armv7a`, `armv7k`, `armv7r`, `armv7s`.
        //
        // todo!() => Some("freebsd/386"),
        //
        // TODO: consult with experts whether this is correct
        // "armv6-unknown-freebsd" | "arm7-unknown-freebsd" => Some("freebsd/arm"),
        //
        // TODO: consult with experts whether this is correct
        // "riscv64gc-unknown-freebsd" => Some("freebsd/riscv64"),
        //
        // TODO: consult with experts whether this is correct
        // "wasm32-unknown-unknown" => Some("js/wasm"),
        //
        // todo!() => Some("linux/386"),
        //
        // At the moment of writing, `loongarch64` is not supported by Rustc:
        // https://github.com/rust-lang/rust/pull/96971
        // "loongarch64_unknown_linux_gnuf64" => Some("linux/loongarch64"),
        //
        // TODO: learn more about riscv64
        // todo!() => Some("linux/riscv64"),
        //
        // todo!() => Some("netbsd/386"),
        //
        // todo!() => Some("openbsd/386"),
        //
        // todo!() => Some("openbsd/arm"),
        //
        // todo!() => Some("openbsd/mips64"),
        //
        // todo!() => Some("plan9/386"),
        // todo!() => Some("plan9/amd64"),
        // todo!() => Some("plan9/arm"),
        //
        // TODO: consult with experts whether this is correct
        // "x86_64-pc-solaris" | "x86_64-sun-solaris" => Some("solaris/amd64"),
        //
        // todo!() => Some("windows/386"),
        // 
        // TODO: consult with experts whether this is correct
        // "x86_64-pc-windows-gnu" | "x86_64-pc-windows-gnullvm" | "x86_64-pc-windows-msvc" | "x86_64-uwp-windows-gnu" | "x86_64-uwp-windows-msvc" => Some("windows/amd64"),
        //
        // todo!() => Some("windows/arm"),
        //
        // "aarch64-pc-windows-gnullvm" | "aarch64-pc-windows-msvc" => Some("windows/arm64"),
    }
}