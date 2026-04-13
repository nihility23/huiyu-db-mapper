use deadpool_sqlite::{Config, Pool, Runtime};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

struct PressureTestResult {
    concurrency: usize,
    total_requests: usize,
    total_time: Duration,
    qps: f64,
    avg_latency: Duration,
    p95_latency: Duration,
    p99_latency: Duration,
    error_count: usize,
}

async fn pressure_test(
    pool: Arc<Pool>,
    concurrency: usize,
    total_requests: usize,
) -> PressureTestResult {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let start = Instant::now();

    let mut latencies = Vec::with_capacity(total_requests);
    let mut errors = 0;

    let mut handles = vec![];
    for _ in 0..total_requests {
        let pool = pool.clone();
        let sem = semaphore.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let task_start = Instant::now();

            let _ = execute_query(pool).await;

            let latency = task_start.elapsed();
            ( latency)
        }));
    }

    // 收集结果
    for handle in handles {
        match handle.await {
            Ok((latency)) => {
                latencies.push(latency);
            }
            Err(_) => errors += 1,
        }
    }

    let total_time = start.elapsed();

    // 计算百分位数
    latencies.sort();
    let p95 = latencies[(latencies.len() as f64 * 0.95) as usize];
    let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
    let avg = latencies.iter().sum::<Duration>() / latencies.len() as u32;

    PressureTestResult {
        concurrency,
        total_requests,
        total_time,
        qps: total_requests as f64 / total_time.as_secs_f64(),
        avg_latency: avg,
        p95_latency: p95,
        p99_latency: p99,
        error_count: errors,
    }
}

async fn execute_query(pool: Arc<Pool>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get().await?;

    conn.interact(|conn| {
        let mut stmt = conn.prepare("SELECT * FROM t_app WHERE id = ?")?;
        let mut rows = stmt.query([&"88c1f0c8843448e589fe0854f96b93a4"])?;
        let _ = rows.next()?;
        Ok::<_, rusqlite::Error>(())
    }).await??;

    Ok(())
}

#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 创建连接池
    let mut cfg = Config::new("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db");
    if let Some(pool_config) = cfg.pool.as_mut() {
        pool_config.max_size = 50; // 设置足够大的连接池
    }
    let pool = Arc::new(cfg.create_pool(Runtime::Tokio1)?);

    println!("🚀 开始压力测试...\n");
    println!("{:-<80}", "");
    println!("{:>10} {:>15} {:>15} {:>15} {:>15} {:>10}",
             "并发数", "QPS", "平均延迟", "P95延迟", "P99延迟", "错误数");
    println!("{:-<80}", "");

    // 测试不同的并发数
    let concurrencies = vec![1, 2, 5, 10, 20, 30, 50, 80, 100];
    let total_requests = 1000; // 每个并发级别发送1000个请求

    for &concurrency in &concurrencies {
        let result = pressure_test(pool.clone(), concurrency, total_requests).await;

        println!(
            "{:>10} {:>15.0} {:>15?} {:>15?} {:>15?} {:>10}",
            result.concurrency,
            result.qps,
            result.avg_latency,
            result.p95_latency,
            result.p99_latency,
            result.error_count
        );

        // 等待一下，让系统恢复
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}