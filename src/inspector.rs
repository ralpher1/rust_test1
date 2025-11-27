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
        let usage_percent = if self.capacity > 0 {
            self.length as f64 / self.capacity as f64 * 100.0
        } else {
            0.0
        };

        let filled_blocks = ((self.length as f64 / self.capacity.max(1) as f64) * 20.0) as usize;
        let empty_blocks = 20 - filled_blocks;
        let memory_bar = format!(
            "{}{}",
            "█".repeat(filled_blocks).bright_green(),
            "░".repeat(empty_blocks).bright_black()
        );

        write!(
            f,
            "{}",
            format!(
                "┌──────────────────────────────────────────────────────────────┐\n\
                 │ {} String Memory Layout                                  │\n\
                 ├──────────────────────────────────────────────────────────────┤\n\
                 │                                                              │\n\
                 │ {} STACK (String metadata - 24 bytes on 64-bit)           │\n\
                 │   ┌────────────────────────────────────────────┐           │\n\
                 │   │ Object @ {:#018x}          │           │\n\
                 │   │ ├─ ptr:  {:#018x}          │           │\n\
                 │   │ ├─ len:  {} bytes                         │           │\n\
                 │   │ └─ cap:  {} bytes                         │           │\n\
                 │   └────────────────────────────────────────────┘           │\n\
                 │            │                                                │\n\
                 │            └──> points to HEAP                              │\n\
                 │                                                              │\n\
                 │ {} HEAP (actual string data)                               │\n\
                 │   Address: {:#018x}                                       │\n\
                 │   Status:  {}                                              │\n\
                 │                                                              │\n\
                 │   Memory Usage:                                              │\n\
                 │   [{}] {:.1}%                            │\n\
                 │                                                              │\n\
                 │   {} Used:   {} bytes                                      │\n\
                 │   {} Total:  {} bytes                                      │\n\
                 │   {} Waste:  {} bytes (unused capacity)                    │\n\
                 │                                                              │\n\
                 ├──────────────────────────────────────────────────────────────┤\n\
                 │ {} {}                                                    │\n\
                 └──────────────────────────────────────────────────────────────┘",
                "🔍".bright_cyan(),
                "📦".bright_green(),
                self.object_ptr,
                self.data_ptr,
                self.length.to_string().bright_cyan(),
                self.capacity.to_string().bright_yellow(),
                "💾".bright_yellow(),
                self.data_ptr,
                if self.is_heap_allocated { "Heap Allocated ✓".bright_green() } else { "Static Memory".bright_blue() },
                memory_bar,
                usage_percent,
                "✓".bright_green(),
                self.length.to_string().bright_cyan(),
                "📊".bright_yellow(),
                self.capacity.to_string().bright_yellow(),
                "⚠".bright_red(),
                self.capacity.saturating_sub(self.length).to_string().bright_red(),
                "📝".bright_white(),
                self.description.bright_white().bold()
            )
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
    let is_static = is_static_str(s, None);

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

/// Attempts to determine if a &str points to static memory (string literal)
///
/// This function compares the pointer of `s` to a known static string reference.
/// If `static_ref` is provided, it uses `std::ptr::eq` to check if `s` points to the same memory.
/// Note: Without a reference to the original static string, it is not possible to reliably detect
/// if a &str is static. This function does not attempt any platform-dependent heuristics.
pub fn is_static_str(s: &str, static_ref: Option<&'static str>) -> bool {
    match static_ref {
        Some(reference) => std::ptr::eq(s, reference),
        None => {
            // Cannot reliably detect static strings without a reference.
            false
        }
    }
}

/// Prints a beautiful comparison of memory layout between two strings
pub fn compare_memory_layout(
    info1: &StringMemoryInfo,
    info2: &StringMemoryInfo,
    operation: &str,
) {
    println!("\n");
    println!("{}", "╔════════════════════════════════════════════════════════════════════════╗".bright_yellow().bold());
    println!("{}", format!("║ {} {:<63} ║", "🔄".bright_white(), operation.bright_white().bold()).bright_yellow().bold());
    println!("{}", "╠════════════════════════════════════════════════════════════════════════╣".bright_yellow().bold());

    // BEFORE state
    println!("{}", "║                                                                        ║".bright_yellow().bold());
    println!("{}", format!("║  {}  BEFORE:                                                          ║", "⏪".bright_cyan()).bright_yellow().bold());
    println!("{}", "║  ┌──────────────────────────────────────────────────────────────┐     ║".bright_yellow().bold());
    println!("{}", format!("║  │ Ptr: {:#018x}  Len: {:>4}  Cap: {:>4}  │     ║",
        info1.data_ptr,
        info1.length.to_string().bright_cyan(),
        info1.capacity.to_string().bright_yellow()).bright_yellow().bold());
    println!("{}", "║  └──────────────────────────────────────────────────────────────┘     ║".bright_yellow().bold());
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    // Arrow showing transformation
    let ptr_changed = info1.data_ptr != info2.data_ptr;
    let capacity_changed = info1.capacity != info2.capacity;
    let length_changed = info1.length != info2.length;

    let arrow_color = if ptr_changed {
        colored::Color::BrightRed
    } else {
        colored::Color::BrightGreen
    };

    println!("{}", format!("║                           {}  {}                              ║",
        "│".color(arrow_color),
        if ptr_changed { "NEW ALLOCATION!" } else { "in-place" }.color(arrow_color).bold()).bright_yellow().bold());
    println!("{}", format!("║                           {}                                       ║", "▼".color(arrow_color)).bright_yellow().bold());
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    // AFTER state
    println!("{}", format!("║  {}  AFTER:                                                           ║", "⏩".bright_green()).bright_yellow().bold());
    println!("{}", "║  ┌──────────────────────────────────────────────────────────────┐     ║".bright_yellow().bold());
    println!("{}", format!("║  │ Ptr: {:#018x}  Len: {:>4}  Cap: {:>4}  │     ║",
        info2.data_ptr,
        info2.length.to_string().bright_cyan(),
        info2.capacity.to_string().bright_yellow()).bright_yellow().bold());
    println!("{}", "║  └──────────────────────────────────────────────────────────────┘     ║".bright_yellow().bold());
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    // Analysis section
    println!("{}", "╠════════════════════════════════════════════════════════════════════════╣".bright_yellow().bold());
    println!("{}", format!("║  {}  ANALYSIS:                                                        ║", "📊".bright_blue()).bright_yellow().bold());
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    if ptr_changed {
        println!("{}", format!("║  {} Pointer:  {:#018x} → {:#018x}  ║",
            "🔴".bright_red(),
            info1.data_ptr,
            info2.data_ptr).bright_yellow().bold());
        println!("{}", format!("║       {} Data was MOVED - new heap allocation!                      ║",
            "└─>".bright_red()).bright_yellow().bold());
    } else {
        println!("{}", format!("║  {} Pointer:  {} (same address)                     ║",
            "🟢".bright_green(),
            "UNCHANGED".bright_green().bold()).bright_yellow().bold());
        println!("{}", "║       └─> Modified in-place, zero-cost operation                       ║".bright_yellow().bold());
    }
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    if capacity_changed {
        let delta = info2.capacity as i64 - info1.capacity as i64;
        let delta_str = if delta > 0 {
            format!("+{}", delta).bright_green()
        } else {
            format!("{}", delta).bright_red()
        };

        println!("{}", format!("║  {} Capacity: {} → {} bytes (Δ {})          ║",
            "📈".bright_yellow(),
            info1.capacity.to_string().bright_white(),
            info2.capacity.to_string().bright_white(),
            delta_str).bright_yellow().bold());
        if info2.capacity > info1.capacity {
            println!("{}", format!("║       └─> Reallocation triggered - grew by {} bytes                 ║",
                delta).bright_yellow().bold());
        }
    } else {
        println!("{}", format!("║  {} Capacity: {} (no reallocation needed)                 ║",
            "✓".bright_green(),
            "UNCHANGED".bright_green().bold()).bright_yellow().bold());
    }
    println!("{}", "║                                                                        ║".bright_yellow().bold());

    if length_changed {
        let delta = info2.length as i64 - info1.length as i64;
        let delta_str = if delta > 0 {
            format!("+{}", delta).bright_green()
        } else {
            format!("{}", delta).bright_red()
        };

        println!("{}", format!("║  {} Length:   {} → {} bytes (Δ {})              ║",
            "📏".bright_cyan(),
            info1.length.to_string().bright_white(),
            info2.length.to_string().bright_white(),
            delta_str).bright_yellow().bold());
    } else {
        println!("{}", format!("║  {} Length:   {} (no data added/removed)                  ║",
            "✓".bright_green(),
            "UNCHANGED".bright_green().bold()).bright_yellow().bold());
    }

    println!("{}", "║                                                                        ║".bright_yellow().bold());
    println!("{}", "╚════════════════════════════════════════════════════════════════════════╝".bright_yellow().bold());
}

/// Displays the byte-level representation of a string
pub fn display_bytes(s: &str, label: &str) {
    println!("\n");
    println!("{}", "┌────────────────────────────────────────────────────────────────────┐".bright_magenta().bold());
    println!("{}", format!("│ {} {:<60} │", "🔬".bright_yellow(), label.bright_white().bold()).bright_magenta().bold());
    println!("{}", "├────────────────────────────────────────────────────────────────────┤".bright_magenta().bold());
    println!("{}", "│                                                                    │".bright_magenta().bold());

    // Show the string
    println!("{}", format!("│  String:      {:<54} │", format!("\"{}\"", s).bright_cyan()).bright_magenta().bold());
    println!("{}", "│                                                                    │".bright_magenta().bold());

    // Byte representation
    let bytes_str = format!("{:?}", s.as_bytes());
    if bytes_str.len() <= 54 {
        println!("{}", format!("│  UTF-8 Bytes: {:<54} │", bytes_str.bright_green()).bright_magenta().bold());
    } else {
        println!("{}", format!("│  UTF-8 Bytes: {:<54} │", &bytes_str[..54].bright_green()).bright_magenta().bold());
        println!("{}", format!("│               {:<54} │", &bytes_str[54..].bright_green()).bright_magenta().bold());
    }

    // Character representation
    let chars: Vec<char> = s.chars().collect();
    let chars_str = format!("{:?}", chars);
    if chars_str.len() <= 54 {
        println!("{}", format!("│  Characters:  {:<54} │", chars_str.bright_yellow()).bright_magenta().bold());
    } else {
        println!("{}", format!("│  Characters:  {:<54} │", &chars_str[..54].bright_yellow()).bright_magenta().bold());
        println!("{}", format!("│               {:<54} │", &chars_str[54..].bright_yellow()).bright_magenta().bold());
    }

    println!("{}", "│                                                                    │".bright_magenta().bold());
    println!("{}", format!("│  {} Byte count:  {:<48} │", "📏".bright_cyan(), s.len().to_string().bright_cyan().bold()).bright_magenta().bold());
    println!("{}", format!("│  {} Char count:  {:<48} │", "🔤".bright_yellow(), s.chars().count().to_string().bright_yellow().bold()).bright_magenta().bold());

    if s.len() != s.chars().count() {
        println!("{}", "│                                                                    │".bright_magenta().bold());
        println!("{}", format!("│  {} {:<61} │",
            "⚠️".bright_yellow(),
            "Multi-byte UTF-8 characters detected!".bright_yellow().bold()).bright_magenta().bold());

        // Show detailed breakdown
        println!("{}", "│                                                                    │".bright_magenta().bold());
        println!("{}", "│  Character Breakdown:                                              │".bright_magenta().bold());
        for (i, ch) in s.chars().enumerate().take(5) {
            let byte_len = ch.len_utf8();
            let marker = if byte_len > 1 { "⚡" } else { "·" };
            println!("{}", format!("│    {} [{}] '{}' = {} byte(s)                                │",
                marker.bright_yellow(),
                i,
                ch.to_string().bright_cyan(),
                byte_len.to_string().bright_green()
            ).bright_magenta().bold());
        }
        if s.chars().count() > 5 {
            println!("{}", "│    ... (and more)                                                  │".bright_magenta().bold());
        }
    }

    println!("{}", "│                                                                    │".bright_magenta().bold());
    println!("{}", "└────────────────────────────────────────────────────────────────────┘".bright_magenta().bold());
}
