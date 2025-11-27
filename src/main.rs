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

use colored::Colorize;
use inspector::*;
use std::borrow::Cow;
use tokio::task;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use transformer::*;

/// Prints a fancy section header
fn print_section(title: &str) {
    println!("\n\n");
    println!(
        "{}",
        format!("╔═══════════════════════════════════════════════════════════╗")
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        format!("║  {:<56} ║", title)
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        format!("╚═══════════════════════════════════════════════════════════╝")
            .bright_cyan()
            .bold()
    );
}

/// Demonstrates basic string types and their memory layout
#[tracing::instrument]
async fn demo_string_types() {
    print_section("1. STRING TYPES AND MEMORY LAYOUT");

    info!("Creating various string types...");

    // String literal - stored in binary's read-only data section
    let literal: &'static str = "Rust";
    let info_literal = inspect_str(literal, "String literal (&'static str)");
    println!("\n{}", info_literal);

    // Owned String - heap allocated with capacity for growth
    let owned = String::from("Rustacean");
    let info_owned = inspect_string(&owned, "Owned String (heap-allocated)");
    println!("\n{}", info_owned);

    // Box<str> - heap allocated but immutable, no extra capacity
    let boxed: Box<str> = "Ferris".into();
    let info_boxed = inspect_boxed_str(&boxed, "Boxed str");
    println!("\n{}", info_boxed);

    // Demonstrate size differences
    println!("\n{}", "📏 SIZE ANALYSIS:".bright_yellow().bold());
    println!(
        "  &str:      {} bytes (fat pointer: ptr + len)",
        std::mem::size_of::<&str>()
    );
    println!(
        "  String:    {} bytes (ptr + len + cap)",
        std::mem::size_of::<String>()
    );
    println!(
        "  Box<str>:  {} bytes (fat pointer: ptr + len)",
        std::mem::size_of::<Box<str>>()
    );
    println!(
        "  Cow<str>:  {} bytes (enum: tag + fat pointer)",
        std::mem::size_of::<Cow<str>>()
    );

    display_bytes(literal, "Byte representation of 'Rust'");
}

/// Demonstrates ownership, moves, and clones
#[tracing::instrument]
async fn demo_ownership() {
    print_section("2. OWNERSHIP, MOVES, AND CLONES");

    info!("Demonstrating ownership mechanics...");

    let original = String::from("Hello, Rust!");
    let info_original = inspect_string(&original, "Original String");
    println!("\n{}", info_original);

    // Move - transfers ownership, no copy
    info!("⚠ About to MOVE the string...");
    let moved = original;
    // Note: `original` is now invalid - compiler prevents use
    let info_moved = inspect_string(&moved, "After MOVE");

    compare_memory_layout(&info_original, &info_moved, "MOVE Operation");

    println!(
        "\n{} The pointer addresses are identical!",
        "💡 INSIGHT:".bright_green().bold()
    );
    println!("   Move is zero-cost - just transfers ownership.");
    println!("   No data was copied, no new allocation happened.");

    // Clone - creates a new heap allocation and copies data
    info!("🔄 About to CLONE the string...");
    let cloned = moved.clone();
    let info_cloned = inspect_string(&cloned, "After CLONE");

    compare_memory_layout(
        &info_moved,
        &info_cloned,
        "CLONE Operation (allocation + copy)",
    );

    println!(
        "\n{} The data pointers are different!",
        "💡 INSIGHT:".bright_green().bold()
    );
    println!("   Clone created a NEW heap allocation.");
    println!("   All bytes were copied to the new location.");
    println!(
        "   Cost: {} bytes allocated + {} bytes copied",
        cloned.capacity(),
        cloned.len()
    );
}

/// Demonstrates capacity and reallocation
#[tracing::instrument]
async fn demo_capacity_and_growth() {
    print_section("3. CAPACITY MANAGEMENT AND REALLOCATION");

    info!("Exploring how String manages capacity...");

    // Create with exact capacity
    let mut s = String::with_capacity(8);
    println!(
        "\n{} Created String::with_capacity(8)",
        "✓".bright_green()
    );

    let info_empty = inspect_string(&s, "Empty String with capacity 8");
    println!("{}", info_empty);

    // Add data within capacity
    s.push_str("Rust");
    let info_rust = inspect_string(&s, "After adding 'Rust' (4 bytes)");

    compare_memory_layout(&info_empty, &info_rust, "Within Capacity Push");

    // Exceed capacity - forces reallocation
    warn!("⚠ About to exceed capacity - reallocation will occur!");

    s.push_str("!!!!!"); // 5 more bytes = 9 total (exceeds 8)

    let info_reallocated = inspect_string(&s, "After exceeding capacity");

    compare_memory_layout(&info_rust, &info_reallocated, "Reallocation Triggered");

    println!(
        "\n{} Reallocation details:",
        "💡 INSIGHT:".bright_green().bold()
    );
    println!("   Old capacity: {} bytes", info_rust.capacity);
    println!("   New capacity: {} bytes", info_reallocated.capacity);
    println!(
        "   Growth strategy: typically doubles capacity"
    );
    println!("   Performance cost: O(n) copy of all existing data");
}

/// Demonstrates Clone-on-Write (Cow) optimization
#[tracing::instrument]
async fn demo_clone_on_write() {
    print_section("4. CLONE-ON-WRITE (COW) OPTIMIZATION");

    info!("Demonstrating Cow<str> for efficient conditional ownership...");

    let static_str = "Ferris the Crab";

    // Cow starts borrowed - zero cost
    let cow_borrowed: Cow<str> = Cow::Borrowed(static_str);
    let info_borrowed = inspect_cow(&cow_borrowed, "Cow::Borrowed (zero-cost)");
    println!("\n{}", info_borrowed);

    println!(
        "\n{} Cow::Borrowed points to the original string",
        "✓".bright_green()
    );
    println!("   No allocation, no copy - just a reference");

    // Convert to owned when needed
    let mut cow_owned = cow_borrowed.clone();
    cow_owned.to_mut().push_str(" 🦀");

    let info_owned = inspect_cow(&cow_owned, "Cow::Owned (after mutation)");

    compare_memory_layout(
        &info_borrowed,
        &info_owned,
        "Cow: Borrowed → Owned (lazy allocation)",
    );

    println!(
        "\n{} Cow delayed allocation until mutation!",
        "💡 INSIGHT:".bright_green().bold()
    );
    println!("   Use case: API that might or might not modify data");
    println!("   Benefit: Zero cost when no modification needed");
}

/// Demonstrates async string processing
#[tracing::instrument]
async fn demo_async_operations() {
    print_section("5. ASYNCHRONOUS STRING PROCESSING");

    info!("Spawning multiple async tasks...");

    let input = String::from("Async");

    // Spawn multiple async tasks concurrently
    let task1 = task::spawn(async_process_string(
        input.clone(),
        "Task 1: Database Fetch",
    ));

    let task2 = task::spawn(async_process_string(
        input.clone(),
        "Task 2: API Call",
    ));

    let task3 = task::spawn(async_process_string(
        input.clone(),
        "Task 3: File Read",
    ));

    println!("\n{} Launched 3 concurrent tasks", "🚀".bright_yellow());
    println!("   Each task is running independently on the Tokio runtime");

    // Await all tasks
    let results = tokio::join!(task1, task2, task3);

    println!("\n{} All tasks completed!", "✓".bright_green().bold());

    match results {
        (Ok(r1), Ok(r2), Ok(r3)) => {
            println!("   Task 1: {}", r1);
            println!("   Task 2: {}", r2);
            println!("   Task 3: {}", r3);
        }
        _ => error!("Some tasks failed!"),
    }

    println!(
        "\n{} Async runtime details:",
        "💡 INSIGHT:".bright_green().bold()
    );
    println!("   Runtime: Tokio (work-stealing scheduler)");
    println!("   Tasks are lightweight (not OS threads)");
    println!("   Concurrent execution without blocking");
}

/// Demonstrates string transformations with timing
#[tracing::instrument]
async fn demo_transformations() {
    print_section("6. STRING TRANSFORMATIONS WITH TIMING");

    info!("Performing various string transformations...");

    let mut manipulator = StringManipulator::new();

    // Reverse
    let input = "Hello, World!";
    let result = manipulator.reverse(input);
    println!("\n{} REVERSE", "🔄".bright_cyan());
    println!("   Input:  '{}'", input);
    println!("   Output: '{}'", result.value);
    result.display_timing();

    display_bytes(&result.value, "Reversed bytes");

    // Uppercase (demonstrates Unicode case mapping)
    let unicode_input = "Straße"; // German street
    let result = manipulator.to_upper(unicode_input);
    println!("\n{} UPPERCASE (Unicode-aware)", "🔤".bright_cyan());
    println!("   Input:  '{}' ({} bytes)", unicode_input, unicode_input.len());
    println!(
        "   Output: '{}' ({} bytes)",
        result.value,
        result.value.len()
    );
    result.display_timing();

    if result.value.len() != unicode_input.len() {
        println!(
            "   {} Case conversion changed byte length!",
            "⚠".bright_yellow()
        );
        println!("      'ß' (1 char, 2 bytes) → 'SS' (2 chars, 2 bytes)");
    }

    // Repeat
    let result = manipulator.repeat("Rust ", 5);
    println!("\n{} REPEAT", "🔁".bright_cyan());
    println!("   Pattern: 'Rust '");
    println!("   Count:   5");
    println!("   Output:  '{}'", result.value);
    println!(
        "   Capacity: {} bytes (pre-allocated, no reallocation)",
        result.value.capacity()
    );
    result.display_timing();

    // Interleave
    let result = manipulator.interleave("RUST", "rust");
    println!("\n{} INTERLEAVE", "🔀".bright_cyan());
    println!("   String 1: 'RUST'");
    println!("   String 2: 'rust'");
    println!("   Output:   '{}'", result.value);
    result.display_timing();

    println!(
        "\n{} Total operations: {}",
        "📊".bright_yellow(),
        manipulator.operations_count.to_string().bright_green().bold()
    );
}

/// Demonstrates UTF-8 and Unicode handling
#[tracing::instrument]
async fn demo_unicode() {
    print_section("7. UNICODE AND UTF-8 HANDLING");

    info!("Exploring UTF-8 encoding...");

    // ASCII - 1 byte per character
    let ascii = "Rust";
    display_bytes(ascii, "ASCII string (1 byte/char)");

    // Multi-byte UTF-8
    let emoji = "🦀🚀";
    display_bytes(emoji, "Emoji (4 bytes/char)");

    // Mixed
    let mixed = "Rust 🦀";
    display_bytes(mixed, "Mixed ASCII + Emoji");

    // Demonstrate the danger of byte indexing
    println!("\n{} BYTE vs CHAR indexing:", "⚠".bright_yellow().bold());
    println!("   String: '{}'", mixed);
    println!("   Length in bytes: {}", mixed.len());
    println!("   Length in chars: {}", mixed.chars().count());

    println!("\n   Character iteration:");
    for (i, ch) in mixed.chars().enumerate() {
        println!("      chars[{}] = '{}' ({} bytes)", i, ch, ch.len_utf8());
    }

    warn!("   ⚠ mixed[5] would PANIC! Use .chars().nth(n) instead");
}

/// Main entry point - sets up logging and runs all demonstrations
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta().bold());
    println!(
        "{}",
        "║                                                                   ║"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        "║        🦀  THE INTROSPECTIVE STRING LABORATORY  🦀                ║"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        "║                                                                   ║"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        "║  A Self-Aware Rust Program Teaching String Internals             ║"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        "║                                                                   ║"
            .bright_magenta()
            .bold()
    );
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta().bold());

    info!("Starting introspective string laboratory...");

    // Run all demonstrations
    demo_string_types().await;
    demo_ownership().await;
    demo_capacity_and_growth().await;
    demo_clone_on_write().await;
    demo_async_operations().await;
    demo_transformations().await;
    demo_unicode().await;

    print_section("✨ LABORATORY SESSION COMPLETE ✨");

    println!("\n{} Key Takeaways:", "📚".bright_green().bold());
    println!("   1. String is heap-allocated, growable, and owned");
    println!("   2. &str is a borrowed slice, can point to stack, heap, or static memory");
    println!("   3. Moves are zero-cost, clones allocate and copy");
    println!("   4. Capacity management affects performance (reallocation is O(n))");
    println!("   5. Cow<str> delays allocation until mutation");
    println!("   6. Rust is UTF-8 aware - chars ≠ bytes");
    println!("   7. Async operations are lightweight and concurrent");

    println!("\n{} Rust guarantees:", "🛡️".bright_blue().bold());
    println!("   ✓ Memory safety without garbage collection");
    println!("   ✓ Thread safety enforced at compile time");
    println!("   ✓ Zero-cost abstractions");
    println!("   ✓ No null pointer exceptions");
    println!("   ✓ No data races");

    info!("Laboratory session complete!");
}
