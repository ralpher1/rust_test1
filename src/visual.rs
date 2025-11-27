//! # Visual Utilities Module
//!
//! Provides rich visual components for an engaging educational experience:
//! - Progress bars and animations
//! - Interactive prompts
//! - Colorful diagrams and separators
//! - Enhanced terminal output

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

/// Creates a beautiful header with gradient-like effect
pub fn print_gradient_header(title: &str) {
    println!("\n");
    let colors = [
        colored::Color::Magenta,
        colored::Color::BrightMagenta,
        colored::Color::Cyan,
        colored::Color::BrightCyan,
    ];

    let border = "═".repeat(70);
    println!("{}", format!("╔{}╗", border).color(colors[0]).bold());
    println!("{}", format!("║{:^70}║", "").color(colors[1]).bold());
    println!("{}", format!("║{:^70}║", title).color(colors[2]).bold());
    println!("{}", format!("║{:^70}║", "").color(colors[3]).bold());
    println!("{}", format!("╚{}╝", border).color(colors[0]).bold());
    println!();
}

/// Creates an animated section header with icons
pub fn print_section_header(number: usize, title: &str, icon: &str) {
    println!("\n\n");

    // Animated reveal
    let full_title = format!("{} Section {}: {}", icon, number, title);

    // Top border
    let width = 75;
    println!("{}", "╔".bright_cyan().bold().to_string() + &"═".repeat(width).bright_cyan().bold().to_string() + &"╗".bright_cyan().bold().to_string());

    // Title with padding
    println!("{}", format!("║{:^width$}║", full_title, width = width).bright_yellow().bold());

    // Bottom border
    println!("{}", "╚".bright_cyan().bold().to_string() + &"═".repeat(width).bright_cyan().bold().to_string() + &"╝".bright_cyan().bold().to_string());
    println!();
}

/// Creates a visual memory diagram
pub fn print_memory_diagram(label: &str, ptr: usize, len: usize, cap: usize, data: &str) {
    println!("\n{}", format!("┌─── {} ───", label).bright_cyan().bold());

    // Stack visualization
    println!("│");
    println!("│ {}", "STACK:".bright_green().bold());
    println!("│   ┌────────────────────┐");
    println!("│   │ ptr:  {:#018x} │", ptr);
    println!("│   │ len:  {:^18} │", len);
    println!("│   │ cap:  {:^18} │", cap);
    println!("│   └────────────────────┘");
    println!("│        │");
    println!("│        │ points to");
    println!("│        ▼");

    // Heap visualization
    println!("│ {}", "HEAP:".bright_yellow().bold());
    println!("│   ┌{:─<48}┐", "");

    // Show data with capacity bars
    let filled = "█".repeat(len.min(40));
    let empty = "░".repeat((cap - len).min(40));
    println!("│   │ {} {} │", filled.bright_green(), empty.bright_black());
    println!("│   │ Data: {:40} │", format!("\"{}\"", data.chars().take(38).collect::<String>()));
    println!("│   │ Used: {} / {} bytes ({:.1}% full) │",
        len.to_string().bright_green(),
        cap.to_string().bright_yellow(),
        (len as f64 / cap as f64 * 100.0)
    );
    println!("│   └{:─<48}┘", "");
    println!("└{:─<50}", "");
}

/// Creates a comparison view with arrows
pub fn print_comparison(
    label: &str,
    before_label: &str,
    before_value: &str,
    after_label: &str,
    after_value: &str,
) {
    println!("\n{}", format!("╔═══ {} ═══╗", label).bright_magenta().bold());
    println!();

    // Before state
    println!("  {}", before_label.bright_cyan().bold());
    println!("  ┌──────────────────────────────┐");
    println!("  │ {:<28} │", before_value);
    println!("  └──────────────────────────────┘");

    // Arrow
    println!("           │");
    println!("           │ {}", "transformation".bright_yellow());
    println!("           ▼");

    // After state
    println!("  {}", after_label.bright_green().bold());
    println!("  ┌──────────────────────────────┐");
    println!("  │ {:<28} │", after_value);
    println!("  └──────────────────────────────┘");
    println!();
    println!("{}", "╚═══════════════════════════════════╝".bright_magenta().bold());
}

/// Shows a highlighted insight box
pub fn print_insight(insight: &str) {
    println!("\n{}", "┌─ 💡 KEY INSIGHT ─────────────────────────────────────────────┐".bright_green().bold());

    for line in insight.lines() {
        println!("{}", format!("│  {:<60}│", line).bright_white());
    }

    println!("{}", "└──────────────────────────────────────────────────────────────┘".bright_green().bold());
}

/// Shows a warning box
pub fn print_warning(warning: &str) {
    println!("\n{}", "┌─ ⚠️  WARNING ───────────────────────────────────────────────────┐".bright_yellow().bold());

    for line in warning.lines() {
        println!("{}", format!("│  {:<60}│", line).bright_yellow());
    }

    println!("{}", "└──────────────────────────────────────────────────────────────┘".bright_yellow().bold());
}

/// Shows an animated progress bar for operations
pub fn show_operation_progress(operation: &str, steps: usize) -> ProgressBar {
    let pb = ProgressBar::new(steps as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    pb.set_message(operation.to_string());
    pb
}

/// Simulates a thinking/processing animation
pub fn animate_thinking(message: &str, duration_ms: u64) {
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let iterations = duration_ms / 100;

    print!("  ");
    for i in 0..iterations {
        let frame = frames[i as usize % frames.len()];
        print!("\r  {} {} ", frame.bright_cyan().bold(), message.bright_white());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("\r  {} {}  ", "✓".bright_green().bold(), message.bright_white());
}

/// Creates a data flow visualization
pub fn print_data_flow(steps: &[(&str, &str)]) {
    println!("\n{}", "DATA FLOW:".bright_blue().bold());

    for (i, (step, description)) in steps.iter().enumerate() {
        let icon = if i == 0 {
            "◆"
        } else if i == steps.len() - 1 {
            "◉"
        } else {
            "◇"
        };

        let color = match i % 4 {
            0 => colored::Color::BrightGreen,
            1 => colored::Color::BrightCyan,
            2 => colored::Color::BrightMagenta,
            _ => colored::Color::BrightYellow,
        };

        println!("  {} {} {}", icon.color(color).bold(), step.color(color), description.bright_white());

        if i < steps.len() - 1 {
            println!("  {}", "│".bright_black());
        }
    }
}

/// Shows a summary box at the end of a section
pub fn print_summary(title: &str, points: &[&str]) {
    let border_len = 55_usize.saturating_sub(title.len());
    let top_border = format!("╔══ {} {}╗", title, "═".repeat(border_len));
    println!("\n{}", top_border.bright_blue().bold());

    for point in points {
        println!("{}", format!("║ ✓ {:<68}║", point).bright_white());
    }

    let bottom_border = format!("╚{}╝", "═".repeat(71));
    println!("{}", bottom_border.bright_blue().bold());
}

/// Interactive prompt to continue
pub fn prompt_continue() {
    println!();
    println!("{}", "─".repeat(75).bright_black());
    println!("{}", "  Press Enter to continue to the next section...".bright_white().dimmed());
    println!("{}", "─".repeat(75).bright_black());

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}

/// Creates a visual table for displaying structured data
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let col_widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let header_len = h.len();
            let max_row_len = rows.iter().map(|r| r.get(i).map(|s| s.len()).unwrap_or(0)).max().unwrap_or(0);
            header_len.max(max_row_len)
        })
        .collect();

    // Top border
    print!("  ┌");
    for (i, width) in col_widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < col_widths.len() - 1 {
            print!("┬");
        }
    }
    println!("┐");

    // Headers
    print!("  │");
    for (i, (header, width)) in headers.iter().zip(col_widths.iter()).enumerate() {
        print!(" {:<width$} ", header.bright_cyan().bold(), width = width);
        if i < headers.len() - 1 {
            print!("│");
        }
    }
    println!("│");

    // Separator
    print!("  ├");
    for (i, width) in col_widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < col_widths.len() - 1 {
            print!("┼");
        }
    }
    println!("┤");

    // Rows
    for row in rows {
        print!("  │");
        for (i, (cell, width)) in row.iter().zip(col_widths.iter()).enumerate() {
            print!(" {:<width$} ", cell, width = width);
            if i < row.len() - 1 {
                print!("│");
            }
        }
        println!("│");
    }

    // Bottom border
    print!("  └");
    for (i, width) in col_widths.iter().enumerate() {
        print!("{}", "─".repeat(width + 2));
        if i < col_widths.len() - 1 {
            print!("┴");
        }
    }
    println!("┘");
}

/// Creates a visual meter/gauge
pub fn print_meter(label: &str, value: f64, max_value: f64, unit: &str) {
    let percentage = (value / max_value * 100.0).min(100.0);
    let filled = (percentage / 100.0 * 30.0) as usize;
    let empty = 30 - filled;

    let bar_color = if percentage < 30.0 {
        colored::Color::BrightGreen
    } else if percentage < 70.0 {
        colored::Color::BrightYellow
    } else {
        colored::Color::BrightRed
    };

    println!(
        "  {} [{}{}] {:.1}% ({:.0} {})",
        label.bright_white().bold(),
        "█".repeat(filled).color(bar_color),
        "░".repeat(empty).bright_black(),
        percentage,
        value,
        unit
    );
}
