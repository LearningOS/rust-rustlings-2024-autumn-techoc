//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间戳
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let timestamp = since_the_epoch.as_secs().to_string();

    // 设置环境变量 TEST_FOO 为当前的 Unix 时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 tests8 中的 "pass" 功能标志
    println!("cargo:rustc-cfg=pass");
}
