//! # Async String Transformer Module
//!
//! This module provides asynchronous string transformation operations.
//! Each transformation is instrumented with tracing and timing data,
//! showing the cost and behavior of different string operations.

use colored::Colorize;
use std::borrow::Cow;
use std::time::Instant;
use tracing::{debug, info, instrument, span, Level};

/// Represents the result of a timed string operation
pub struct TimedResult<T> {
    pub value: T,
    pub duration_nanos: u128,
    pub operation: String,
}

impl<T> TimedResult<T> {
    pub fn display_timing(&self) {
        let duration_str = if self.duration_nanos < 1_000 {
            format!("{} ns", self.duration_nanos)
        } else if self.duration_nanos < 1_000_000 {
            format!("{:.2} μs", self.duration_nanos as f64 / 1_000.0)
        } else {
            format!("{:.2} ms", self.duration_nanos as f64 / 1_000_000.0)
        };

        println!(
            "  ⏱  {} took {}",
            self.operation.bright_cyan(),
            duration_str.bright_yellow().bold()
        );
    }
}

/// Macro to time an operation and wrap it in TimedResult
macro_rules! timed {
    ($op_name:expr, $block:expr) => {{
        let start = Instant::now();
        let result = $block;
        let duration = start.elapsed().as_nanos();
        TimedResult {
            value: result,
            duration_nanos: duration,
            operation: $op_name.to_string(),
        }
    }};
}

/// Simulates an async string processing task
///
/// In real-world scenarios, this might be:
/// - Fetching data from a database
/// - Making an HTTP request
/// - Reading from a file
/// - Performing CPU-intensive transformation
#[instrument(skip(input), fields(input_len = input.len()))]
pub async fn async_process_string(input: String, task_name: &str) -> String {
    let span = span!(Level::INFO, "async_task", task = task_name);
    let _enter = span.enter();

    info!("Starting async task: {}", task_name);
    debug!("Input length: {} bytes", input.len());

    // Simulate async work (in real app, this would be actual I/O)
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let result = format!("[{}]", input);

    info!("Completed async task: {}", task_name);
    debug!("Output length: {} bytes", result.len());

    result
}

/// Demonstrates String vs &str ownership and borrowing
///
/// Key concepts:
/// - String is owned (heap-allocated, growable)
/// - &str is borrowed (reference to string data)
/// - Conversions have costs (allocation, copying)
#[instrument]
pub fn demonstrate_ownership() -> Vec<String> {
    info!("=== Ownership Demonstration ===");

    // 1. String literal - lives in binary, type is &'static str
    let literal: &'static str = "Hello";
    debug!("Created string literal at: {:p}", literal);

    // 2. Convert to String - allocates on heap, COPIES data
    let owned: String = literal.to_string();
    debug!("Converted to String at: {:p}", owned.as_ptr());

    // 3. Borrow as &str - no allocation, just references owned's data
    let borrowed: &str = &owned;
    debug!("Borrowed as &str, points to: {:p}", borrowed);

    // 4. Clone - creates NEW heap allocation, COPIES data
    let cloned: String = owned.clone();
    debug!("Cloned String at: {:p} (NEW allocation)", cloned.as_ptr());

    vec![owned, cloned]
}

/// Demonstrates capacity management and reallocation
///
/// Shows how String grows when capacity is exceeded
#[instrument]
pub fn demonstrate_capacity() -> String {
    info!("=== Capacity and Reallocation ===");

    // Start with exact capacity
    let mut s = String::with_capacity(5);
    info!("Created String with capacity: {}", s.capacity());
    debug!("Initial pointer: {:p}", s.as_ptr());

    // Fill to capacity - no reallocation
    s.push_str("Hello");
    info!("After 'Hello': len={}, cap={}", s.len(), s.capacity());
    debug!("Pointer: {:p} (same)", s.as_ptr());

    // Exceed capacity - triggers reallocation!
    let ptr_before = s.as_ptr() as usize;
    s.push_str(" World");
    let ptr_after = s.as_ptr() as usize;

    if ptr_before != ptr_after {
        info!("⚠ REALLOCATION occurred!");
        info!("Old pointer: {:#x}", ptr_before);
        info!("New pointer: {:#x}", ptr_after);
        info!("New capacity: {}", s.capacity());
    }

    s
}

/// Demonstrates Clone-on-Write (Cow) optimization
///
/// Cow delays allocation until modification is needed
#[instrument]
pub fn demonstrate_cow<'a>(input: &'a str, should_modify: bool) -> Cow<'a, str> {
    info!("=== Clone-on-Write Demonstration ===");

    // Start borrowed - no allocation yet
    let mut cow: Cow<str> = Cow::Borrowed(input);
    info!("Created Cow::Borrowed (no allocation)");

    if should_modify {
        // This triggers conversion to Owned
        info!("Modifying Cow - will trigger allocation");
        cow.to_mut().push_str(" [modified]");
        info!("Now Cow::Owned (allocated on heap)");
    } else {
        info!("No modification - stays Cow::Borrowed (zero-cost)");
    }

    cow
}

/// Performs various string manipulations with detailed tracking
pub struct StringManipulator {
    pub operations_count: usize,
}

impl StringManipulator {
    pub fn new() -> Self {
        Self {
            operations_count: 0,
        }
    }

    /// Reverses a string (demonstrates Unicode handling)
    #[instrument(skip(self))]
    pub fn reverse(&mut self, s: &str) -> TimedResult<String> {
        self.operations_count += 1;

        let result = timed!("reverse", {
            // Note: We reverse by characters, not bytes (Unicode-aware)
            s.chars().rev().collect::<String>()
        });

        info!(
            "Reversed '{}' -> '{}' in {} ns",
            s, result.value, result.duration_nanos
        );

        result
    }

    /// Converts to uppercase (demonstrates case mapping complexity)
    #[instrument(skip(self))]
    pub fn to_upper(&mut self, s: &str) -> TimedResult<String> {
        self.operations_count += 1;

        let result = timed!("to_uppercase", {
            // Unicode case mapping can change byte length!
            // Example: "ß" (1 char, 2 bytes) -> "SS" (2 chars, 2 bytes)
            s.to_uppercase()
        });

        if result.value.len() != s.len() {
            info!(
                "⚠ Length changed during case conversion: {} -> {} bytes",
                s.len(),
                result.value.len()
            );
        }

        result
    }

    /// Repeats a string n times (demonstrates capacity planning)
    #[instrument(skip(self))]
    pub fn repeat(&mut self, s: &str, count: usize) -> TimedResult<String> {
        self.operations_count += 1;

        let result = timed!("repeat", {
            // Pre-allocate exact capacity - avoids reallocations
            let mut result = String::with_capacity(s.len() * count);
            for _ in 0..count {
                result.push_str(s);
            }
            result
        });

        info!(
            "Repeated '{}' {}x = {} bytes (capacity: {})",
            s,
            count,
            result.value.len(),
            result.value.capacity()
        );

        result
    }

    /// Interleaves two strings (demonstrates borrowing and building)
    #[instrument(skip(self))]
    pub fn interleave(&mut self, s1: &str, s2: &str) -> TimedResult<String> {
        self.operations_count += 1;

        let result = timed!("interleave", {
            let mut result = String::new();
            let mut chars1 = s1.chars();
            let mut chars2 = s2.chars();

            loop {
                match (chars1.next(), chars2.next()) {
                    (Some(c1), Some(c2)) => {
                        result.push(c1);
                        result.push(c2);
                    }
                    (Some(c1), None) => result.push(c1),
                    (None, Some(c2)) => result.push(c2),
                    (None, None) => break,
                }
            }

            result
        });

        info!(
            "Interleaved '{}' and '{}' -> '{}'",
            s1, s2, result.value
        );

        result
    }
}

impl Default for StringManipulator {
    fn default() -> Self {
        Self::new()
    }
}
