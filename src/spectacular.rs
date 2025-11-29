//! # Spectacular Logging and Performance Module
//!
//! This module provides MAXIMUM SPECTACULAR console output with:
//! - Real-time performance monitoring
//! - Enhanced logging with visual flair
//! - Memory tracking and visualization
//! - Animated effects and transitions
//! - Rainbow gradients and color effects

use colored::Colorize;
use chrono::Local;
use rand::Rng;
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

/// Performance metrics tracker
#[derive(Debug, Clone)]
pub struct PerformanceTracker {
    pub operation_name: String,
    pub start_time: Instant,
    pub memory_before: usize,
    pub checkpoints: Vec<(String, Duration)>,
}

impl PerformanceTracker {
    /// Create a new performance tracker
    pub fn new(operation_name: &str) -> Self {
        log_performance_start(operation_name);
        Self {
            operation_name: operation_name.to_string(),
            start_time: Instant::now(),
            memory_before: 0, // Would need actual memory tracking
            checkpoints: Vec::new(),
        }
    }

    /// Add a checkpoint
    pub fn checkpoint(&mut self, label: &str) {
        let elapsed = self.start_time.elapsed();
        self.checkpoints.push((label.to_string(), elapsed));
        log_checkpoint(label, elapsed);
    }

    /// Finish tracking and display results
    pub fn finish(&self) {
        let total_time = self.start_time.elapsed();
        log_performance_complete(&self.operation_name, total_time);
        display_performance_summary(self);
    }
}

/// Display a spectacular startup animation
pub fn spectacular_startup_animation() {
    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    println!("\n\n");

    // Matrix-style code rain effect (simplified)
    matrix_rain_effect(5);

    // Animated title reveal
    let title = "🚀 SPECTACULAR RUST STRING LABORATORY 🚀";
    print!("\n");
    for (i, ch) in title.chars().enumerate() {
        let color = colors[i % colors.len()];
        print!("{}", ch.to_string().color(color).bold());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(25));
    }
    println!("\n");

    // Pulsing effect
    for _ in 0..3 {
        print!("\r{}", "═".repeat(80).bright_cyan().bold());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        print!("\r{}", "═".repeat(80).bright_magenta().bold());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("\r{}", "═".repeat(80).bright_green().bold());

    // System initialization messages
    let init_messages = [
        "⚡ Initializing quantum string processors...",
        "🔥 Loading hyperdimensional memory analyzers...",
        "💎 Calibrating UTF-8 photon detectors...",
        "🌟 Engaging async warp drive...",
        "✨ Synchronizing reality matrices...",
        "🎯 Performance monitoring: ACTIVE",
    ];

    for msg in &init_messages {
        animate_loading_message(msg, 200);
    }

    println!("\n");
    rainbow_separator();
    println!("\n");
}

/// Matrix-style code rain effect
fn matrix_rain_effect(duration_iterations: u32) {
    let chars = "01アイウエオカキクケコサシスセソ";
    let mut rng = rand::thread_rng();

    for _ in 0..duration_iterations {
        let line: String = (0..80)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();

        println!("{}", line.bright_green().dimmed());
        thread::sleep(Duration::from_millis(50));
    }
}

/// Animate a loading message
fn animate_loading_message(msg: &str, duration_ms: u64) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let colors = [
        colored::Color::BrightCyan,
        colored::Color::BrightMagenta,
        colored::Color::BrightYellow,
    ];

    let iterations = duration_ms / 50;
    for i in 0..iterations {
        let frame = frames[i as usize % frames.len()];
        let color = colors[i as usize % colors.len()];
        print!("\r  {} {}   ", frame.color(color).bold(), msg.bright_white());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
    }
    println!("\r  {} {} ✓", "✓".bright_green().bold(), msg.bright_white());
}

/// Display a rainbow separator
pub fn rainbow_separator() {
    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    let pattern = "▓▒░";
    let mut output = String::new();

    for i in 0..(80 / pattern.len()) {
        let color = colors[i % colors.len()];
        output.push_str(&pattern.color(color).bold().to_string());
    }

    println!("{}", output);
}

/// Log the start of a performance-critical operation
pub fn log_performance_start(operation: &str) {
    let timestamp = Local::now().format("%H:%M:%S%.3f");
    println!(
        "\n{} {} {} {}",
        "┌─[".bright_cyan().bold(),
        timestamp.to_string().bright_yellow(),
        "]".bright_cyan().bold(),
        "─".repeat(60).bright_cyan()
    );
    println!(
        "{} {} {}",
        "│".bright_cyan().bold(),
        "⚡ STARTING:".bright_green().bold(),
        operation.bright_white().bold()
    );
}

/// Log a checkpoint during an operation
pub fn log_checkpoint(label: &str, elapsed: Duration) {
    let millis = elapsed.as_millis();
    let color = if millis < 100 {
        colored::Color::BrightGreen
    } else if millis < 500 {
        colored::Color::BrightYellow
    } else {
        colored::Color::BrightRed
    };

    println!(
        "{} {} {} {} {}",
        "│".bright_cyan().bold(),
        "  ├─>".color(color),
        label.bright_white(),
        format!("[+{:.3}ms]", millis).color(color).bold(),
        performance_bar(millis as f64, 1000.0, 20)
    );
}

/// Log the completion of an operation
pub fn log_performance_complete(operation: &str, total_time: Duration) {
    let millis = total_time.as_millis();
    let color = if millis < 500 {
        colored::Color::BrightGreen
    } else if millis < 2000 {
        colored::Color::BrightYellow
    } else {
        colored::Color::BrightRed
    };

    println!(
        "{} {} {} {}",
        "│".bright_cyan().bold(),
        "✓ COMPLETED:".color(color).bold(),
        operation.bright_white().bold(),
        format!("[Total: {:.3}ms]", millis).color(color).bold()
    );
    println!(
        "{}{}",
        "└─".bright_cyan().bold(),
        "─".repeat(75).bright_cyan()
    );
}

/// Create a performance bar visualization
fn performance_bar(value: f64, max: f64, width: usize) -> String {
    let percentage = (value / max).min(1.0);
    let filled = (percentage * width as f64) as usize;
    let empty = width - filled;

    let color = if percentage < 0.3 {
        colored::Color::BrightGreen
    } else if percentage < 0.7 {
        colored::Color::BrightYellow
    } else {
        colored::Color::BrightRed
    };

    format!(
        "{}{}",
        "█".repeat(filled).color(color),
        "░".repeat(empty).bright_black()
    )
}

/// Display a comprehensive performance summary
fn display_performance_summary(tracker: &PerformanceTracker) {
    let total_ms = tracker.start_time.elapsed().as_millis();

    println!("\n");
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════════════╗"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        format!(
            "║  {} {:<63} ║",
            "📊".bright_yellow(),
            "PERFORMANCE ANALYSIS".bright_white().bold()
        )
        .bright_magenta()
        .bold()
    );
    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════════════════╣"
            .bright_magenta()
            .bold()
    );
    println!(
        "{}",
        format!(
            "║  Operation: {:<61} ║",
            tracker.operation_name.bright_cyan()
        )
        .bright_magenta()
        .bold()
    );
    println!(
        "{}",
        format!(
            "║  Total Time: {:<58} ║",
            format!("{:.3}ms", total_ms).bright_green().bold()
        )
        .bright_magenta()
        .bold()
    );
    println!(
        "{}",
        format!(
            "║  Checkpoints: {:<57} ║",
            tracker.checkpoints.len().to_string().bright_yellow()
        )
        .bright_magenta()
        .bold()
    );

    if !tracker.checkpoints.is_empty() {
        println!(
            "{}",
            "╠══════════════════════════════════════════════════════════════════════════╣"
                .bright_magenta()
                .bold()
        );
        println!(
            "{}",
            "║  Checkpoint Timeline:                                                    ║"
                .bright_magenta()
                .bold()
        );
        println!(
            "{}",
            "║                                                                          ║"
                .bright_magenta()
                .bold()
        );

        for (i, (label, duration)) in tracker.checkpoints.iter().enumerate() {
            let ms = duration.as_millis();
            let percentage = (ms as f64 / total_ms as f64) * 100.0;
            println!(
                "{}",
                format!(
                    "║  {} {} {:<35} {:>8}ms ({:>5.1}%) ║",
                    format!("{:2}.", i + 1).bright_white(),
                    "▸".bright_cyan(),
                    label.bright_white(),
                    ms.to_string().bright_green(),
                    percentage
                )
                .bright_magenta()
                .bold()
            );
        }
    }

    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════════╝"
            .bright_magenta()
            .bold()
    );
}

/// Display a live memory usage visualization
pub fn display_memory_snapshot(label: &str, used_bytes: usize, total_bytes: usize) {
    let used_mb = used_bytes as f64 / 1_048_576.0;
    let total_mb = total_bytes as f64 / 1_048_576.0;
    let percentage = (used_bytes as f64 / total_bytes as f64) * 100.0;

    let filled = ((percentage / 100.0) * 40.0) as usize;
    let empty = 40 - filled;

    let color = if percentage < 50.0 {
        colored::Color::BrightGreen
    } else if percentage < 80.0 {
        colored::Color::BrightYellow
    } else {
        colored::Color::BrightRed
    };

    println!(
        "\n{} {} [{}{}] {:.1}% ({:.2} MB / {:.2} MB)",
        "💾".bright_cyan(),
        label.bright_white().bold(),
        "█".repeat(filled).color(color),
        "░".repeat(empty).bright_black(),
        percentage,
        used_mb,
        total_mb
    );
}

/// Glitch effect for dramatic moments
pub fn glitch_effect(text: &str, intensity: u8) {
    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightGreen,
        colored::Color::BrightBlue,
        colored::Color::BrightCyan,
        colored::Color::BrightMagenta,
    ];

    for _ in 0..intensity {
        let _color = colors[rand::thread_rng().gen_range(0..colors.len())];
        let selected_color = colors[rand::thread_rng().gen_range(0..colors.len())];
        print!("\r{}", text.color(selected_color).bold());
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
    }
    println!("\r{}", text.bright_white().bold());
}

/// Particle burst effect (text-based)
pub fn particle_burst(center_x: usize, message: &str) {
    let particles = ["*", "·", "°", "˚", "✧", "✦", "✨", "⭐"];
    let colors = [
        colored::Color::BrightYellow,
        colored::Color::BrightCyan,
        colored::Color::BrightMagenta,
    ];

    // Center message
    println!("{:>width$}", message.bright_white().bold(), width = center_x + message.len() / 2);

    // Burst animation
    for frame in 0..10 {
        let mut line = " ".repeat(80);
        for _ in 0..20 {
            let pos = rand::thread_rng().gen_range(center_x.saturating_sub(20)..center_x + 20).min(79);
            let particle = particles[rand::thread_rng().gen_range(0..particles.len())];
            let _color = colors[rand::thread_rng().gen_range(0..colors.len())];

            let mut chars: Vec<char> = line.chars().collect();
            if pos < chars.len() {
                chars[pos] = particle.chars().next().unwrap();
            }
            line = chars.into_iter().collect();
        }

        if frame < 5 {
            println!("{}", line.color(colors[frame % colors.len()]));
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/// Progress spinner with fancy effects
pub fn fancy_spinner(message: &str, duration_ms: u64) {
    let frames = [
        "◐", "◓", "◑", "◒",
    ];
    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    let iterations = duration_ms / 75;
    for i in 0..iterations {
        let frame = frames[i as usize % frames.len()];
        let color = colors[i as usize % colors.len()];
        let dots = ".".repeat((i as usize % 3) + 1);

        print!(
            "\r  {} {} {}   ",
            frame.color(color).bold(),
            message.bright_white(),
            dots.color(color)
        );
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(75));
    }
    println!("\r  {} {} ✓", "✓".bright_green().bold(), message.bright_white());
}

/// Display operation statistics in a cool ASCII graph
pub fn display_operation_stats(stats: &[(&str, f64)]) {
    println!("\n");
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════════════╗"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        format!(
            "║  {} {:<63} ║",
            "📈".bright_yellow(),
            "OPERATION STATISTICS".bright_white().bold()
        )
        .bright_blue()
        .bold()
    );
    println!(
        "{}",
        "╠══════════════════════════════════════════════════════════════════════════╣"
            .bright_blue()
            .bold()
    );

    let max_value = stats.iter().map(|(_, v)| *v).fold(0.0, f64::max);

    for (label, value) in stats {
        let bar_width = ((value / max_value) * 50.0) as usize;
        let bar = "█".repeat(bar_width);

        println!(
            "{}",
            format!(
                "║  {:<20} {:>10.2} {} ║",
                label.bright_cyan(),
                value.to_string().bright_green().bold(),
                bar.bright_yellow()
            )
            .bright_blue()
            .bold()
        );
    }

    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════════╝"
            .bright_blue()
            .bold()
    );
}

/// Rainbow text gradient effect
pub fn rainbow_text(text: &str) -> String {
    let colors = [
        colored::Color::BrightRed,
        colored::Color::BrightYellow,
        colored::Color::BrightGreen,
        colored::Color::BrightCyan,
        colored::Color::BrightBlue,
        colored::Color::BrightMagenta,
    ];

    text.chars()
        .enumerate()
        .map(|(i, ch)| {
            let color = colors[i % colors.len()];
            ch.to_string().color(color).bold().to_string()
        })
        .collect::<String>()
}

/// Pulsing text effect
pub fn pulse_text(text: &str, pulses: u8) {
    let colors = [
        colored::Color::BrightBlack,
        colored::Color::White,
        colored::Color::BrightWhite,
        colored::Color::White,
    ];

    for _ in 0..pulses {
        for color in &colors {
            print!("\r{}", text.color(*color).bold());
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!();
}
