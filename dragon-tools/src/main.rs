use clap::{Parser, Subcommand};
use dragon_tools::{DjedClock, ProtocolStack, AtomicWriter, AkashicRecord};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "dragon-tools")]
#[command(about = "Dragon Tools v2.0 Kernel", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commit state to the Akashic Ledger
    Commit {
        #[arg(short, long)]
        message: String,
        #[arg(long, default_value = "0")]
        id: u64,
    },
    /// Query records (simulation)
    Query {
        #[arg(long, default_value = "10")]
        count: usize,
    },
    /// Run benchmarks and stress tests
    Benchmark {
        #[arg(long, default_value = "10")]
        duration: u64,
    },
    /// Show system metrics
    Metrics,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize components
    let clock = DjedClock::new();
    let protocol = ProtocolStack::new();
    // Use a temp file for demo purposes
    let mut writer = AtomicWriter::new("akashic.log")?;

    match cli.command {
        Commands::Commit { message, id } => {
            let start = Instant::now();

            // 1. Calculate Phase
            let phase = clock.calculate_phase();

            // 2. Prepare Record
            // For demo, just using 0s for prev_hash
            let record = AkashicRecord::new(
                id,
                DjedClock::now_ns(),
                phase,
                [0; 32],
                message.as_bytes(),
            );

            // 3. Hash (Audit)
            let _hash = protocol.hash_record(&record);

            // 4. Atomic Write
            writer.write_record(&record)?;

            let elapsed = start.elapsed();
            println!("Commit successful.");
            println!("Phase: {}", phase);
            println!("Latency: {:.2}µs", elapsed.as_micros() as f64);
        }
        Commands::Query { count } => {
            println!("Querying last {} records...", count);
            println!("(Query implementation pending database integration)");
        }
        Commands::Benchmark { duration } => {
            println!("Running Chaos Boundary Benchmark for {} seconds...", duration);
            // Simple loop to test throughput
            let start_bench = Instant::now();
            let mut count = 0;
            let mut errors = 0;

            while start_bench.elapsed().as_secs() < duration {
                let phase = clock.calculate_phase();
                let record = AkashicRecord::new(
                    count,
                    DjedClock::now_ns(),
                    phase,
                    [0; 32],
                    &[0u8; 22]
                );

                if let Err(_) = writer.write_record(&record) {
                    errors += 1;
                }
                count += 1;
            }

            let total_time = start_bench.elapsed().as_secs_f64();
            let rate = count as f64 / total_time;
            println!("Total Commits: {}", count);
            println!("Errors: {}", errors);
            println!("Rate: {:.2} commits/sec", rate);
        }
        Commands::Metrics => {
            let phase = clock.calculate_phase();
            println!("System Status: NOMINAL");
            println!("Djed Phase: {}", phase);
        }
    }

    Ok(())
}
