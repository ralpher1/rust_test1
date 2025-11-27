//! # String Inspector Module
//!
//! This module provides deep introspection capabilities for Rust strings.
//! It reveals the hidden memory layout, allocation details, and internal
//! representation of various string types.

use colored::Colorize;
use std::borrow::Cow;
use std::fmt;

/// Represents detailed memory information about a string
#[derive(Debug, Clone)]
pub struct StringMemoryInfo {
    /// The memory address where the string data lives (heap pointer)
    pub data_ptr: usize,
    /// The memory address of the String/&str object itself (stack location)
    pub object_ptr: usize,
    /// Current length in bytes
    pub length: usize,
    /// Allocated capacity in bytes (heap allocation size)
    pub capacity: usize,
    /// Whether this string is heap-allocated
    pub is_heap_allocated: bool,
    /// Human-readable description
    pub description: String,
}

impl fmt::Display for StringMemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "┌─ String Memory Layout\n\
                 │ Object Location (stack): {:#x}\n\
                 │ Data Location   (heap):  {:#x}\n\
                 │ Length:                  {} bytes\n\
                 │ Capacity:                {} bytes\n\
                 │ Heap Allocated:          {}\n\
                 │ Wasted Space:            {} bytes\n\
                 └─ {}",
                self.object_ptr,
                self.data_ptr,
                self.length,
                self.capacity,
                if self.is_heap_allocated { "Yes ✓" } else { "No ✗" },
                self.capacity.saturating_sub(self.length),
                self.description
            )
            .cyan()
        )
    }
}

/// Inspects a `String` and returns detailed memory information
///
/// # How it works:
/// - `String` in Rust is a struct with three fields: ptr, len, cap
/// - The object itself lives on the stack (24 bytes on 64-bit systems)
/// - The actual character data lives on the heap
/// - We use raw pointers to examine the memory layout
pub fn inspect_string(s: &String, description: &str) -> StringMemoryInfo {
    StringMemoryInfo {
        // Pointer to the heap-allocated buffer
        data_ptr: s.as_ptr() as usize,
        // Pointer to the String object on the stack
        object_ptr: s as *const String as usize,
        length: s.len(),
        capacity: s.capacity(),
        is_heap_allocated: true,
        description: description.to_string(),
    }
}

/// Inspects a string slice `&str`
///
/// # How it works:
/// - `&str` is a "fat pointer": contains both data pointer and length
/// - It's 16 bytes on 64-bit systems (8 bytes ptr + 8 bytes len)
/// - The data it points to could be anywhere: stack, heap, or static memory
/// - String literals live in the binary's read-only data section
pub fn inspect_str(s: &str, description: &str) -> StringMemoryInfo {
    // Determine if this is a string literal (in static memory) or elsewhere
    let is_static = is_static_str(s);

    StringMemoryInfo {
        data_ptr: s.as_ptr() as usize,
        object_ptr: s as *const str as *const () as usize,
        length: s.len(),
        capacity: s.len(), // &str has no separate capacity
        is_heap_allocated: !is_static,
        description: format!(
            "{} | Location: {}",
            description,
            if is_static { "Static (Binary)" } else { "Dynamic" }
        ),
    }
}

/// Inspects a `Box<str>` - heap-allocated, immutable string
///
/// # How it works:
/// - `Box<str>` is like `&str` but owns its data on the heap
/// - No extra capacity (unlike String) - exactly sized
/// - More memory efficient than String when size is fixed
pub fn inspect_boxed_str(s: &Box<str>, description: &str) -> StringMemoryInfo {
    StringMemoryInfo {
        data_ptr: s.as_ptr() as usize,
        object_ptr: s.as_ref() as *const str as *const () as usize,
        length: s.len(),
        capacity: s.len(),
        is_heap_allocated: true,
        description: format!("{} | Type: Box<str> (immutable)", description),
    }
}

/// Inspects a `Cow<str>` - Clone on Write smart pointer
///
/// # How it works:
/// - Cow can be either Borrowed or Owned
/// - Borrowed: just wraps a &str (no allocation)
/// - Owned: wraps a String (heap allocated)
/// - Delays allocation until mutation is needed
pub fn inspect_cow(s: &Cow<str>, description: &str) -> StringMemoryInfo {
    let (is_owned, data_ptr, capacity) = match s {
        Cow::Borrowed(str_ref) => {
            (false, str_ref.as_ptr() as usize, str_ref.len())
        }
        Cow::Owned(string) => {
            (true, string.as_ptr() as usize, string.capacity())
        }
    };

    StringMemoryInfo {
        data_ptr,
        object_ptr: s as *const Cow<str> as *const () as usize,
        length: s.len(),
        capacity,
        is_heap_allocated: is_owned,
        description: format!(
            "{} | Cow: {}",
            description,
            if is_owned { "Owned" } else { "Borrowed" }
        ),
    }
}

/// Attempts to determine if a &str points to static memory
///
/// This is a heuristic - we check if the pointer is in a "reasonable"
/// static range. This works for string literals in the binary.
fn is_static_str(s: &str) -> bool {
    let ptr = s.as_ptr() as usize;
    // String literals typically live in low memory addresses
    // This is platform-dependent but works for demonstration
    ptr < 0x600000000000
}

/// Prints a beautiful comparison of memory layout between two strings
pub fn compare_memory_layout(
    info1: &StringMemoryInfo,
    info2: &StringMemoryInfo,
    operation: &str,
) {
    println!("\n{}", format!("╔═══ {} ═══╗", operation).bright_yellow().bold());

    println!("\n{}", "BEFORE:".bright_green().bold());
    println!("{}", info1);

    println!("\n{}", "AFTER:".bright_magenta().bold());
    println!("{}", info2);

    // Analysis
    let ptr_changed = info1.data_ptr != info2.data_ptr;
    let capacity_changed = info1.capacity != info2.capacity;
    let length_changed = info1.length != info2.length;

    println!("\n{}", "ANALYSIS:".bright_blue().bold());

    if ptr_changed {
        println!("  {} Data was {} - NEW heap allocation!",
            "➜".bright_red(),
            "MOVED".bright_red().bold());
        println!("    Old address: {:#x}", info1.data_ptr);
        println!("    New address: {:#x}", info2.data_ptr);
    } else {
        println!("  {} Data pointer {} - modified in-place",
            "➜".bright_green(),
            "UNCHANGED".bright_green().bold());
    }

    if capacity_changed {
        println!("  {} Capacity changed: {} → {} bytes",
            "➜".bright_yellow(),
            info1.capacity,
            info2.capacity);
        if info2.capacity > info1.capacity {
            println!("    Reallocation occurred (grew by {} bytes)",
                info2.capacity - info1.capacity);
        }
    }

    if length_changed {
        println!("  {} Length changed: {} → {} bytes (Δ {})",
            "➜".bright_cyan(),
            info1.length,
            info2.length,
            info2.length as i64 - info1.length as i64);
    }

    println!("{}", "╚═══════════════════════════╝".bright_yellow().bold());
}

/// Displays the byte-level representation of a string
pub fn display_bytes(s: &str, label: &str) {
    println!("\n{} {}", "📊".bright_yellow(), label.bright_white().bold());
    println!("  UTF-8 bytes: {:?}", s.as_bytes());
    println!("  Characters:  {:?}", s.chars().collect::<Vec<_>>());
    println!("  Byte count:  {}", s.len());
    println!("  Char count:  {}", s.chars().count());

    if s.len() != s.chars().count() {
        println!("  {} Multi-byte UTF-8 characters detected!",
            "⚠".bright_yellow());
    }
}
