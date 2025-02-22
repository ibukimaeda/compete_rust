use std::time::Instant;

use cargo_snippet::snippet;

#[snippet(":timer")]
#[macro_export]
macro_rules! timer {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(
            "計測開始から{}.{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        result
    }};
}
