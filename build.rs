use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取环境变量 TEST_FOO 的值
    let test_foo = env::var("TEST_FOO").expect("TEST_FOO environment variable is not set");

    // 将环境变量的值解析为 u64 类型的数字
    let test_foo: u64 = test_foo.parse().expect("TEST_FOO is not a valid u64 number");

    // 获取当前时间戳（从 UNIX_EPOCH 开始的秒数）
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 检查 TEST_FOO 是否在当前时间戳前后10秒内
    if timestamp < test_foo || timestamp >= test_foo + 10 {
        eprintln!("TEST_FOO is not within 10 seconds of the current time.");
        process::exit(1);
    }

    // 如果检查通过，告诉 Cargo 当 TEST_FOO 改变时要重新运行构建脚本
    println!("cargo:rerun-if-env-changed=TEST_FOO");
}