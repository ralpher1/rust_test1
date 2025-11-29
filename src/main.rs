//! # The Introspective String Laboratory
//!
//! An educational Rust program that performs string manipulations while
//! providing deep introspection into memory layout, ownership, and async behavior.
//!
//! ## Key Concepts Demonstrated:
//! - String vs &str vs Box<str> vs Cow<str>
//! - Stack vs heap allocation
//! - Ownership, borrowing, and lifetimes
//! - Capacity management and reallocation
//! - Async/await with Tokio
//! - Structured logging with tracing
//! - Unicode and UTF-8 handling
//!
//! ## Architecture:
//! - `inspector`: Low-level memory introspection utilities
//! - `transformer`: Async string transformation operations
//! - `main`: Orchestrates demonstrations with rich logging

mod inspector;
mod transformer;
mod visual;
mod spectacular;

use colored::Colorize;
use inspector::*;
use std::borrow::Cow;
use tokio::task;
use tracing::{error, info, warn, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use transformer::*;
use visual::*;
use spectacular::*;

// Removed old print_section - using visual::print_section_header now

/// Demonstrates basic string types and their memory layout
#[tracing::instrument]
async fn demo_string_types() {
    let mut perf = PerformanceTracker::new("String Types Demo");

    print_section_header(1, "STRING TYPES AND MEMORY LAYOUT", "📚");
    print_spectacular_banner("✨ Exploring the String Universe ✨");

    info!("Creating various string types...");
    fancy_spinner("Analyzing string types", 300);
    perf.checkpoint("Section initialization");

    // String literal - stored in binary's read-only data section
    println!("\n{}", "  ➤ Creating string literal...".bright_cyan());
    let literal: &'static str = "Rust";
    let info_literal = inspect_str(literal, "String literal (&'static str)", Some(literal));
    println!("\n{}", info_literal);

    // Owned String - heap allocated with capacity for growth
    println!("\n{}", "  ➤ Creating owned String...".bright_cyan());
    let owned = String::from("Rustacean");
    let info_owned = inspect_string(&owned, "Owned String (heap-allocated)");
    println!("\n{}", info_owned);

    // Box<str> - heap allocated but immutable, no extra capacity
    println!("\n{}", "  ➤ Creating Box<str>...".bright_cyan());
    let boxed: Box<str> = "Ferris".into();
    let info_boxed = inspect_boxed_str(&boxed, "Boxed str");
    println!("\n{}", info_boxed);

    // Demonstrate size differences
    print_table(
        &["Type", "Size (bytes)", "Structure", "Use Case"],
        &[
            vec![
                "&str".to_string(),
                std::mem::size_of::<&str>().to_string(),
                "fat pointer (ptr + len)".to_string(),
                "Borrowed string slice".to_string(),
            ],
            vec![
                "String".to_string(),
                std::mem::size_of::<String>().to_string(),
                "ptr + len + cap".to_string(),
                "Owned, growable".to_string(),
            ],
            vec![
                "Box<str>".to_string(),
                std::mem::size_of::<Box<str>>().to_string(),
                "fat pointer (ptr + len)".to_string(),
                "Owned, immutable".to_string(),
            ],
            vec![
                "Cow<str>".to_string(),
                std::mem::size_of::<Cow<str>>().to_string(),
                "enum (tag + pointer)".to_string(),
                "Clone-on-write".to_string(),
            ],
        ],
    );

    display_bytes(literal, "Byte representation of 'Rust'");

    print_summary(
        "String Types Summary",
        &[
            "&str is a view into string data (no ownership)",
            "String owns its data and can grow",
            "Box<str> owns data but cannot grow (fixed size)",
            "All types are UTF-8 encoded",
        ],
    );

    perf.checkpoint("Summary displayed");
    perf.finish();

    print_3d_box(
        "💡 Pro Tip",
        &[
            "Choose &str for borrowing, String for ownership!",
            "Box<str> is perfect for immutable heap strings.",
        ],
    );

    prompt_continue();
}

/// Demonstrates ownership, moves, and clones
#[tracing::instrument]
async fn demo_ownership() {
    let mut perf = PerformanceTracker::new("Ownership Demo");

    print_section_header(2, "OWNERSHIP, MOVES, AND CLONES", "🔐");
    print_spectacular_banner("🔄 The Dance of Ownership 🔄");

    info!("Demonstrating ownership mechanics...");
    perf.checkpoint("Demo started");

    println!("\n{}", "  ➤ Creating original string...".bright_cyan());
    let original = String::from("Hello, Rust!");
    let info_original = inspect_string(&original, "Original String");
    println!("\n{}", info_original);

    // Move - transfers ownership, no copy
    info!("⚠ About to MOVE the string...");
    fancy_spinner("Performing zero-cost move operation", 400);
    perf.checkpoint("Move operation");

    let moved = original;
    // Note: `original` is now invalid - compiler prevents use
    let info_moved = inspect_string(&moved, "After MOVE");

    compare_memory_layout(&info_original, &info_moved, "MOVE Operation");

    print_insight(
        "The pointer addresses are identical!\n\
         Move is zero-cost - just transfers ownership.\n\
         No data was copied, no new allocation happened.\n\
         The compiler prevents use of 'original' after the move.",
    );

    // Clone - creates a new heap allocation and copies data
    info!("🔄 About to CLONE the string...");
    fancy_spinner("Allocating new memory and copying data", 400);
    perf.checkpoint("Clone operation");

    let cloned = moved.clone();
    let info_cloned = inspect_string(&cloned, "After CLONE");

    compare_memory_layout(
        &info_moved,
        &info_cloned,
        "CLONE Operation (allocation + copy)",
    );

    print_insight(&format!(
        "The data pointers are different!\n\
         Clone created a NEW heap allocation.\n\
         All bytes were copied to the new location.\n\
         Cost: {} bytes allocated + {} bytes copied",
        cloned.capacity(),
        cloned.len()
    ));

    print_data_flow(&[
        ("Original String", "Created with data on heap"),
        ("Move Operation", "Ownership transferred (zero-cost)"),
        ("Clone Operation", "New allocation + data copy"),
        ("Result", "Two independent strings in memory"),
    ]);

    print_summary(
        "Ownership Summary",
        &[
            "Move transfers ownership without copying data",
            "Clone creates a new heap allocation and copies all bytes",
            "Compiler prevents use-after-move errors",
            "Choose clone only when you truly need independent copies",
        ],
    );

    perf.checkpoint("Summary completed");
    perf.finish();

    display_memory_snapshot("Memory Usage After Demo", 24000, 1048576);

    prompt_continue();
}

/// Demonstrates capacity and reallocation
#[tracing::instrument]
async fn demo_capacity_and_growth() {
    let mut perf = PerformanceTracker::new("Capacity Management Demo");

    print_section_header(3, "CAPACITY MANAGEMENT AND REALLOCATION", "📊");
    print_spectacular_banner("📈 Capacity Growth Visualization 📈");

    info!("Exploring how String manages capacity...");
    perf.checkpoint("Initialization");

    // Create with exact capacity
    println!(
        "\n{}",
        "  ➤ Creating String with capacity 8...".bright_cyan()
    );
    let mut s = String::with_capacity(8);

    let info_empty = inspect_string(&s, "Empty String with capacity 8");
    println!("{}", info_empty);

    print_meter("Capacity Usage", 0.0, 8.0, "bytes");

    // Add data within capacity
    println!(
        "\n{}",
        "  ➤ Adding 'Rust' (4 bytes) within capacity...".bright_cyan()
    );
    fancy_spinner("Writing to existing buffer", 300);
    perf.checkpoint("Within-capacity write");

    s.push_str("Rust");
    let info_rust = inspect_string(&s, "After adding 'Rust' (4 bytes)");

    compare_memory_layout(&info_empty, &info_rust, "Within Capacity Push");

    print_meter("Capacity Usage", 4.0, 8.0, "bytes");

    print_insight(
        "No reallocation occurred!\n\
         The data was written into the pre-allocated buffer.\n\
         The pointer address remains the same.",
    );

    // Exceed capacity - forces reallocation
    warn!("⚠ About to exceed capacity - reallocation will occur!");
    glitch_effect("⚠️  REALLOCATION IMMINENT  ⚠️", 5);
    println!(
        "\n{}",
        "  ➤ Adding '!!!!!' (5 more bytes = 9 total)...".bright_yellow()
    );
    fancy_spinner("Triggering reallocation", 300);
    perf.checkpoint("Reallocation triggered");

    s.push_str("!!!!!"); // 5 more bytes = 9 total (exceeds 8)

    let info_reallocated = inspect_string(&s, "After exceeding capacity");

    compare_memory_layout(&info_rust, &info_reallocated, "Reallocation Triggered");

    print_meter(
        "Capacity Usage",
        9.0,
        info_reallocated.capacity as f64,
        "bytes",
    );

    print_warning(
        "REALLOCATION OCCURRED!\n\
         When capacity is exceeded, String must:\n\
         1. Allocate a new, larger buffer (usually 2x size)\n\
         2. Copy all existing data to the new buffer\n\
         3. Free the old buffer\n\
         This is an O(n) operation!",
    );

    print_insight(&format!(
        "Reallocation details:\n\
         Old capacity: {} bytes\n\
         New capacity: {} bytes\n\
         Growth factor: {:.2}x\n\
         Performance cost: O(n) - all {} bytes were copied",
        info_rust.capacity,
        info_reallocated.capacity,
        info_reallocated.capacity as f64 / info_rust.capacity as f64,
        info_rust.length
    ));

    print_data_flow(&[
        ("Empty String", "8 bytes allocated, 0 used"),
        ("Add 'Rust'", "4 bytes used, no reallocation"),
        ("Add '!!!!!'", "Exceeds capacity → triggers reallocation"),
        (
            "New Buffer",
            &format!("{} bytes allocated (2x growth)", info_reallocated.capacity),
        ),
    ]);

    print_summary(
        "Capacity Management Summary",
        &[
            "Pre-allocate capacity when final size is known",
            "Reallocations are expensive (O(n) copy operation)",
            "String typically doubles capacity when growing",
            "Use String::with_capacity() to avoid reallocations",
        ],
    );

    perf.checkpoint("Demo complete");
    perf.finish();

    print_timeline(&[
        ("0ms", "String created with capacity 8"),
        ("+50ms", "Added 'Rust' - no reallocation"),
        ("+100ms", "Added '!!!!!' - REALLOCATION!"),
        ("+150ms", "New capacity: 16 bytes"),
    ]);

    prompt_continue();
}

/// Demonstrates Clone-on-Write (Cow) optimization
#[tracing::instrument]
async fn demo_clone_on_write() {
    let mut perf = PerformanceTracker::new("Clone-on-Write Demo");

    print_section_header(4, "CLONE-ON-WRITE (COW) OPTIMIZATION", "🐄");
    print_spectacular_banner("🐄 The Power of Lazy Allocation 🐄");

    info!("Demonstrating Cow<str> for efficient conditional ownership...");
    perf.checkpoint("Demo started");

    let static_str = "Ferris the Crab";

    // Cow starts borrowed - zero cost
    println!(
        "\n{}",
        "  ➤ Creating Cow::Borrowed (zero-cost wrapper)...".bright_cyan()
    );
    fancy_spinner("Wrapping borrowed reference", 300);
    perf.checkpoint("Cow::Borrowed created");

    let cow_borrowed: Cow<str> = Cow::Borrowed(static_str);
    let info_borrowed = inspect_cow(&cow_borrowed, "Cow::Borrowed (zero-cost)");
    println!("\n{}", info_borrowed);

    print_insight(
        "Cow::Borrowed is just a reference wrapper!\n\
         No allocation occurred.\n\
         No data was copied.\n\
         Points directly to the original string.",
    );

    // Convert to owned when needed
    println!(
        "\n{}",
        "  ➤ Mutating Cow (triggers conversion to Owned)...".bright_yellow()
    );
    fancy_spinner("Allocating heap memory for mutation", 400);
    perf.checkpoint("Cow converted to Owned");

    let mut cow_owned = cow_borrowed.clone();
    cow_owned.to_mut().push_str(" 🦀");

    let info_owned = inspect_cow(&cow_owned, "Cow::Owned (after mutation)");

    compare_memory_layout(
        &info_borrowed,
        &info_owned,
        "Cow: Borrowed → Owned (lazy allocation)",
    );

    print_data_flow(&[
        ("Start", "Cow::Borrowed wraps a reference"),
        ("Clone", "Still borrowed - cheap clone"),
        ("to_mut() call", "Triggers allocation and copy"),
        ("Mutation", "Now Cow::Owned with independent data"),
    ]);

    print_insight(
        "Cow delayed allocation until mutation!\n\
         The allocation only happened when we called to_mut().\n\
         If we never mutated, it would stay borrowed (zero-cost).",
    );

    print_summary(
        "Cow<str> Summary",
        &[
            "Cow delays allocation until mutation is needed",
            "Perfect for APIs that might or might not modify data",
            "Zero-cost when no modification occurs",
            "Automatically converts Borrowed → Owned on mutation",
        ],
    );

    perf.checkpoint("Summary displayed");
    perf.finish();

    display_operation_stats(&[
        ("Borrowed Creation", 0.001),
        ("Clone (still borrowed)", 0.001),
        ("to_mut() Call", 0.05),
        ("Mutation", 0.002),
    ]);

    prompt_continue();
}

/// Demonstrates async string processing
#[tracing::instrument]
async fn demo_async_operations() {
    let mut perf = PerformanceTracker::new("Async Operations Demo");

    print_section_header(5, "ASYNCHRONOUS STRING PROCESSING", "⚡");
    print_spectacular_banner("⚡ Concurrent Task Execution ⚡");

    info!("Spawning multiple async tasks...");
    perf.checkpoint("Task spawning initiated");

    let input = String::from("Async");

    println!("\n{}", "  ➤ Spawning concurrent tasks...".bright_cyan());
    animate_thinking("Launching async runtime", 300);

    // Spawn multiple async tasks concurrently
    let task1 = task::spawn(async_process_string(
        input.clone(),
        "Task 1: Database Fetch",
    ));

    let task2 = task::spawn(async_process_string(input.clone(), "Task 2: API Call"));

    let task3 = task::spawn(async_process_string(input.clone(), "Task 3: File Read"));

    println!(
        "\n{} {} concurrent tasks running",
        "🚀".bright_yellow(),
        "3".bright_green().bold()
    );

    let pb = show_operation_progress("Waiting for async tasks", 3);

    // Await all tasks
    let results = tokio::join!(task1, task2, task3);
    pb.finish_with_message("All tasks completed!");

    println!("\n{}", "✓ All async tasks finished!".bright_green().bold());

    perf.checkpoint("All tasks completed");

    particle_burst(40, "🎉 TASKS COMPLETE! 🎉");

    match results {
        (Ok(r1), Ok(r2), Ok(r3)) => {
            println!(
                "   {} Task 1 result: {}",
                "▸".bright_cyan(),
                r1.bright_white()
            );
            println!(
                "   {} Task 2 result: {}",
                "▸".bright_cyan(),
                r2.bright_white()
            );
            println!(
                "   {} Task 3 result: {}",
                "▸".bright_cyan(),
                r3.bright_white()
            );
        }
        _ => error!("Some tasks failed!"),
    }

    print_insight(
        "Async tasks in Rust:\n\
         - Lightweight: Not OS threads, managed by runtime\n\
         - Concurrent: Run independently, can overlap I/O\n\
         - Work-stealing: Tokio scheduler balances load\n\
         - Zero-cost: Compiled to state machines",
    );

    print_summary(
        "Async Processing Summary",
        &[
            "Tasks are lightweight (not OS threads)",
            "Tokio runtime uses work-stealing scheduler",
            "Concurrent execution without blocking",
            "Perfect for I/O-bound operations",
        ],
    );

    perf.checkpoint("Demo finalized");
    perf.finish();

    print_live_dashboard(&[
        ("Active Tasks", "3", 100.0),
        ("Completed Tasks", "3", 100.0),
        ("CPU Usage", "Low", 25.0),
        ("Memory Efficiency", "High", 15.0),
    ]);

    prompt_continue();
}

/// Demonstrates string transformations with timing
#[tracing::instrument]
async fn demo_transformations() {
    let mut perf = PerformanceTracker::new("String Transformations Demo");

    print_section_header(6, "STRING TRANSFORMATIONS WITH TIMING", "🔧");
    print_spectacular_banner("🔧 String Transformation Magic 🔧");

    info!("Performing various string transformations...");
    perf.checkpoint("Demo started");

    let mut manipulator = StringManipulator::new();

    // Reverse
    println!("\n{}", "  ➤ Reversing string...".bright_cyan());
    let input = "Hello, World!";
    let result = manipulator.reverse(input);
    println!(
        "\n{} {} operation",
        "🔄".bright_cyan(),
        "REVERSE".bright_cyan().bold()
    );
    print_comparison("Reverse Operation", "Input", input, "Output", &result.value);
    result.display_timing();

    display_bytes(&result.value, "Reversed bytes");

    // Uppercase (demonstrates Unicode case mapping)
    println!(
        "\n{}",
        "  ➤ Converting to uppercase (Unicode-aware)...".bright_cyan()
    );
    let unicode_input = "Straße"; // German street
    let result = manipulator.to_upper(unicode_input);
    println!(
        "\n{} {} operation",
        "🔤".bright_cyan(),
        "UPPERCASE".bright_cyan().bold()
    );
    print_comparison(
        "Unicode Case Mapping",
        "Input",
        unicode_input,
        "Output",
        &result.value,
    );
    result.display_timing();

    if result.value.len() != unicode_input.len() {
        print_warning(&format!(
            "Case conversion changed byte length!\n\
             'ß' (1 char, 2 bytes) → 'SS' (2 chars, 2 bytes)\n\
             Input:  {} bytes\n\
             Output: {} bytes",
            unicode_input.len(),
            result.value.len()
        ));
    }

    // Repeat
    println!("\n{}", "  ➤ Repeating pattern...".bright_cyan());
    let result = manipulator.repeat("Rust ", 5);
    println!(
        "\n{} {} operation",
        "🔁".bright_cyan(),
        "REPEAT".bright_cyan().bold()
    );
    println!("   Pattern: {}", "'Rust '".bright_yellow());
    println!("   Count:   {}", "5".bright_green());
    println!(
        "   Output:  {}",
        format!("'{}'", result.value).bright_white()
    );
    print_meter(
        "Capacity Efficiency",
        result.value.len() as f64,
        result.value.capacity() as f64,
        "bytes",
    );
    result.display_timing();

    // Interleave
    println!("\n{}", "  ➤ Interleaving strings...".bright_cyan());
    let result = manipulator.interleave("RUST", "rust");
    println!(
        "\n{} {} operation",
        "🔀".bright_cyan(),
        "INTERLEAVE".bright_cyan().bold()
    );
    println!("   String 1: {}", "'RUST'".bright_yellow());
    println!("   String 2: {}", "'rust'".bright_green());
    println!(
        "   Output:   {}",
        format!("'{}'", result.value).bright_white()
    );
    result.display_timing();

    print_summary(
        &format!(
            "Transformations Complete - {} operations performed",
            manipulator.operations_count
        ),
        &[
            "Reverse: O(n) - iterates through all characters",
            "Uppercase: Unicode-aware, may change byte length",
            "Repeat: Pre-allocates capacity for efficiency",
            "Interleave: Demonstrates character-by-character processing",
        ],
    );

    perf.checkpoint("All transformations complete");
    perf.finish();

    print_memory_grid(b"Rust transformations!", "Transformation Output Bytes");

    prompt_continue();
}

/// Demonstrates UTF-8 and Unicode handling
#[tracing::instrument]
async fn demo_unicode() {
    let mut perf = PerformanceTracker::new("Unicode Demo");

    print_section_header(7, "UNICODE AND UTF-8 HANDLING", "🌍");
    print_spectacular_banner("🌍 The Universal Character Set 🌍");

    info!("Exploring UTF-8 encoding...");
    perf.checkpoint("Demo initialization");

    // ASCII - 1 byte per character
    println!("\n{}", "  ➤ Examining ASCII characters...".bright_cyan());
    let ascii = "Rust";
    display_bytes(ascii, "ASCII string (1 byte/char)");

    // Multi-byte UTF-8
    println!(
        "\n{}",
        "  ➤ Examining emoji (multi-byte UTF-8)...".bright_cyan()
    );
    let emoji = "🦀🚀";
    display_bytes(emoji, "Emoji (4 bytes/char)");

    // Mixed
    println!("\n{}", "  ➤ Examining mixed ASCII + Emoji...".bright_cyan());
    let mixed = "Rust 🦀";
    display_bytes(mixed, "Mixed ASCII + Emoji");

    // Demonstrate the danger of byte indexing
    println!(
        "\n{}",
        "╔════ BYTE vs CHAR INDEXING ════╗".bright_yellow().bold()
    );
    println!("   String: {}", format!("'{}'", mixed).bright_cyan());
    println!(
        "   Byte length: {} bytes",
        mixed.len().to_string().bright_red()
    );
    println!(
        "   Char length: {} chars",
        mixed.chars().count().to_string().bright_green()
    );

    print_table(
        &["Index", "Character", "UTF-8 Bytes", "Byte Count"],
        &mixed
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                vec![
                    i.to_string(),
                    format!("'{}'", ch),
                    format!("{:?}", ch.to_string().as_bytes()),
                    ch.len_utf8().to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    );

    print_warning(
        "DANGER: Byte indexing with mixed[n] can PANIC!\n\
         \n\
         ❌ mixed[5] would panic (not on char boundary)\n\
         ✅ mixed.chars().nth(n) is safe\n\
         ✅ Always iterate with .chars() for Unicode strings",
    );

    print_insight(
        "UTF-8 is a variable-length encoding:\n\
         - ASCII characters: 1 byte (backward compatible)\n\
         - Most Latin/Cyrillic: 2 bytes\n\
         - Most Asian scripts: 3 bytes\n\
         - Emoji and rare chars: 4 bytes",
    );

    print_summary(
        "Unicode/UTF-8 Summary",
        &[
            "Rust strings are always valid UTF-8",
            "Characters (chars) ≠ Bytes - use .chars() to iterate",
            "Direct byte indexing can panic - use safe methods",
            "UTF-8 is space-efficient for Western text",
        ],
    );

    perf.checkpoint("Demo complete");
    perf.finish();

    dramatic_reveal("🎓 You now understand Rust's Unicode handling!", 50);

    prompt_continue();
}

/// Main entry point - sets up logging and runs all demonstrations
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(false)
        .with_line_number(false)
        .init();

    // Spectacular startup sequence
    spectacular_startup_animation();

    // Welcome banner
    print_animated_logo();
    print_gradient_header("🦀  THE INTROSPECTIVE STRING LABORATORY  🦀");

    println!(
        "\n{}",
        "┌────────────────────────────────────────────────────────────────────┐"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│                                                                    │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│  Welcome to an interactive journey through Rust's string internals│"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│                                                                    │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│  You will learn:                                                   │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • How Rust manages string memory                                │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • The cost of moves vs clones                                   │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • Capacity management and reallocation                          │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • Clone-on-write optimizations                                  │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • Asynchronous string processing                                │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│    • UTF-8 and Unicode handling                                    │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "│                                                                    │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "└────────────────────────────────────────────────────────────────────┘"
            .bright_blue()
            .bold()
    );

    info!("Starting introspective string laboratory...");
    rainbow_separator();
    fancy_spinner("Initializing laboratory environment", 500);

    display_memory_snapshot("System Memory Status", 512000, 8388608);
    println!();

    // Run all demonstrations
    demo_string_types().await;
    demo_ownership().await;
    demo_capacity_and_growth().await;
    demo_clone_on_write().await;
    demo_async_operations().await;
    demo_transformations().await;
    demo_unicode().await;

    // Final summary
    print_section_header(8, "LABORATORY SESSION COMPLETE", "✨");

    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════════════════╗"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║                      🎓 KEY TAKEAWAYS                                 ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════════════╣"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║                                                                      ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  1️⃣  String is heap-allocated, growable, and owned                   ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  2️⃣  &str is a borrowed slice (stack/heap/static memory)             ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  3️⃣  Moves are zero-cost, clones allocate and copy                   ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  4️⃣  Capacity management affects performance (realloc is O(n))       ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  5️⃣  Cow<str> delays allocation until mutation                       ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  6️⃣  Rust is UTF-8 aware - characters ≠ bytes                        ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║  7️⃣  Async operations are lightweight and concurrent                 ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║                                                                      ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════╝"
            .bright_green()
            .bold()
    );

    println!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════════════════╗"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║                      🛡️  RUST GUARANTEES                              ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════════════╣"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║                                                                      ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║  ✅ Memory safety without garbage collection                         ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║  ✅ Thread safety enforced at compile time                           ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║  ✅ Zero-cost abstractions                                           ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║  ✅ No null pointer exceptions                                       ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║  ✅ No data races                                                    ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "║                                                                      ║"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════╝"
            .bright_blue()
            .bold()
    );

    println!("\n");
    rainbow_separator();
    particle_burst(40, "✨ SESSION COMPLETE ✨");
    println!();

    pulse_text("🎉 Thank you for exploring Rust's string internals! 🎉", 3);

    println!(
        "\n{}",
        rainbow_text("Keep learning, keep building, and keep being awesome! 🦀")
    );

    println!("\n");
    dramatic_reveal("🔥 You are now a Rust String Master! 🔥", 40);
    println!();

    rainbow_separator();

    info!("Laboratory session complete!");
    glitch_effect(">>> SHUTTING DOWN <<<", 6);
    println!();
}
