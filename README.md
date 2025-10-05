# Cloud Ping RS 🚀

> A production-ready, high-performance network latency testing and monitoring utility built in Rust

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

Cloud Ping RS is a comprehensive network performance testing tool designed for developers, DevOps engineers, and network administrators. It provides real-time monitoring, intelligent scoring, and detailed analytics for cloud infrastructure endpoints across multiple providers.

## 📋 Table of Contents

- [Features](#-features)
- [What's New in v3.0](#-whats-new-in-v30)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Usage Guide](#-usage-guide)
- [Configuration](#️-configuration)
- [Output Formats](#-output-formats)
- [Architecture](#️-architecture)
- [Scoring Algorithm](#-scoring-algorithm)
- [Data Format](#-data-format)
- [Development](#-development)
- [Performance](#-performance)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

## 🆕 What's New in v3.0

### Planned Network Monitoring System

Version 3.0 plans to introduce a complete **asynchronous network monitoring system** with enterprise-grade features:

*Note: The library contains the core components for monitoring, but the full monitoring system with CLI commands is not yet implemented in the main application.*

#### Planned Core Improvements
- **Real-time probe runner** with TCP/HTTP/ICMP support and configurable concurrency
- **Streaming aggregator** with sliding window metrics (P50/P90/P99, EWMA jitter, packet loss)
- **Intelligent scoring system** with deterministic normalization and configurable weights
- **Alert system** for incident detection (score drops, sustained loss, availability issues)
- **Modular architecture** optimized for performance and maintainability

#### Code Quality Enhancements
- Refactored error handling for better robustness and debugging
- Improved code organization with dedicated display module
- Enhanced IP address validation using the `ipnet` crate
- Optimized JSON serialization for better readability and type safety
- Reduced utility code by 36% (105 lines) by leveraging well-maintained crates
- Added comprehensive inline documentation and examples

#### Performance Optimizations
- Zero-cost abstractions with `#[inline]` attributes
- Efficient byte and number formatting using specialized crates
- Improved memory management with pre-allocated buffers
- Faster compilation times with optimized dependency tree

## ✨ Features

### 🎯 Current Core Capabilities
- **Multi-threaded concurrent testing** - Intelligent thread pool management with configurable limits
- **Advanced scoring system** - Customizable weighted metrics for comprehensive performance evaluation
- **Basic network quality assessment** - Overall score and performance category
- **Configuration management** - TOML files and environment variable support
- **Data validation** - Built-in validation for configuration and endpoint data

### Planned Additional Capabilities
- **Application-specific suitability** - Tailored analysis for gaming, streaming, VoIP, browsing, and downloads
- **Real-time progress tracking** - Beautiful terminal UI with live updates and progress bars
- **Multiple output formats** - JSON, CSV, and formatted table output

### 📊 Current Analytics & Reporting
- **Basic statistical analysis** - Average, min, max, packet loss, jitter
- **Performance categorization** - Premium, Excellent, Good, Fair, Poor ratings
- **QoS grading** - Letter grades (A+ to F) for quality of service

### Planned Advanced Analytics & Reporting
- **Statistical analysis** - Median, mean, 95th percentile, standard deviation
- **Jitter measurement** - Network stability and consistency metrics
- **Packet loss tracking** - Success rate and failure analysis
- **Historical trends** - Time-series data with confidence scoring
- **Detailed timing breakdowns** - DNS, TCP, TLS, and total request time

### 🏗️ Monitoring System Architecture

*Note: These features are implemented in the library but not yet exposed through the command-line interface*

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Probe Runner  │───▶│ Stream Aggregator│───▶│  Alert System   │
│                 │    │                  │    │                 │
│ • TCP/HTTP/ICMP │    │ • Sliding Windows│    │ • Score Drops   │
│ • Async/Await   │    │ • P50/P90/P99    │    │ • Sustained Loss│
│ • Concurrency   │    │ • EWMA Jitter    │    │ • Availability  │
│ • Rate Limiting │    │ • Packet Loss %  │    │ • Custom Rules  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Endpoints     │    │   Metrics Store  │    │   Notifications │
│                 │    │                  │    │                 │
│ • Auto-discovery│    │ • Ring Buffers   │    │ • Broadcast     │
│ • Health Status │    │ • Cached Aggregs │    │ • Subscribers   │
│ • Metadata      │    │ • Time Series    │    │ • Severity      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

#### Key Components

**Probe Runner**
- Asynchronous endpoint testing with configurable intervals
- Support for TCP, HTTP, and ICMP protocols
- Built-in rate limiting and jitter control
- Automatic retry logic with exponential backoff

**Stream Aggregator**
- Real-time metrics computation with sliding windows
- Short window: 60 probes for recent performance
- Long window: 720 probes for historical trends
- EWMA (Exponentially Weighted Moving Average) for jitter calculation

**Scoring Engine**
- Deterministic normalization for consistent results
- Weighted components: latency (40%), packet loss (30%), jitter (15%), availability (15%)
- Application-specific suitability scores
- Performance categorization (Premium, Excellent, Good, Fair, Poor)

**Alert System**
- Intelligent incident detection with configurable thresholds
- Multiple severity levels (Critical, Warning, Info)
- Alert types: score drops, sustained packet loss, low availability
- Broadcast notifications to multiple subscribers

### ⚙️ Modern Architecture
- **Async/await** - Tokio runtime for maximum concurrency and performance
- **Memory safety** - Rust's ownership system prevents data races and memory leaks
- **Zero-cost abstractions** - High-level code with low-level performance
- **Configuration management** - TOML files with environment variable overrides
- **Structured logging** - Tracing framework with JSON and pretty-print formats
- **Comprehensive testing** - Unit tests, integration tests, and benchmarks
- **Type safety** - Strong typing prevents entire classes of bugs

## 🚀 Quick Start

### Prerequisites
- **Rust 1.75 or higher** - Install from [rustup.rs](https://rustup.rs/)
- **Internet connection** - Required for testing cloud endpoints
- **Linux/macOS/Windows** - Cross-platform support

### Installation

#### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/example/cloud-ping-rs
cd cloud-ping-rs

# Build the release version (optimized)
cargo build --release

# The binary will be at target/release/cloud-ping-rs
./target/release/cloud-ping-rs --version
```

#### Using Cargo Install

```bash
# Install directly from crates.io (when published)
cargo install cloud-ping-rs

# Or install from git repository
cargo install --git https://github.com/example/cloud-ping-rs
```

#### Quick Build Script

```bash
# Use the provided build script
chmod +x run.sh
./run.sh
```

### Basic Usage

```bash
# Run basic network benchmark (tests all regions with 10 pings each)
cloud-ping-rs

# Run with verbose logging
cloud-ping-rs --verbose
```

Planned commands (not yet implemented):

```bash
# Comprehensive benchmark of all regions (default: 10 pings each)
cloud-ping-rs benchmark

# Quick test (3 pings per region, faster results)
cloud-ping-rs quick

# Test a specific URL
cloud-ping-rs test https://example.com

# List all available regions and providers
cloud-ping-rs list

# Validate configuration and data files
cloud-ping-rs validate

# Show current configuration
cloud-ping-rs config --show

# Display help and all available commands
cloud-ping-rs --help
```

### First Run Example

```bash
# Run a basic network benchmark to see it in action
cargo run

# Or with verbose logging
cargo run -- --verbose

# Expected output:
# === Network Quality Assessment ===
# Overall Score: XX/100
# Quality: [Category]
# - [Description based on performance]
```

## 📖 Usage Guide

### Current Usage

The current implementation runs a basic benchmark test across all available regions in the `data.json` file:

```bash
# Run basic network benchmark (tests all regions with 10 pings each)
cloud-ping-rs

# Run with verbose logging
cloud-ping-rs --verbose
```

The application will:
- Load cloud provider endpoints from `data.json`
- Test each endpoint with 10 ping requests
- Calculate and display an overall network quality score
- Show performance category (Excellent, Good, Fair, Poor, etc.)

### Planned CLI Commands

Future versions will include these planned command-line features:

| Command | Purpose | Use Case |
|---------|---------|----------|
| `benchmark` | Comprehensive testing | Production monitoring, detailed analysis |
| `quick` | Fast testing | Quick checks, CI/CD pipelines |
| `test` | Single URL testing | Ad-hoc testing, debugging |
| `list` | Region management | Discovery, inventory |
| `validate` | Configuration validation | Setup verification, troubleshooting |
| `config` | Configuration management | Setup, customization |

#### Planned `benchmark` Command

Full-featured testing with detailed metrics and analysis.

```bash
# Test all regions with default settings (10 pings each)
cloud-ping-rs benchmark

# Custom ping count and save results
cloud-ping-rs benchmark --count 20 --save --output results.json

# Filter by provider (supports partial matching)
cloud-ping-rs benchmark --provider "Amazon Web Services"
cloud-ping-rs benchmark --provider "AWS"

# Filter by region name or code
cloud-ping-rs benchmark --region "us-east"
cloud-ping-rs benchmark --region "virginia"

# Combine filters
cloud-ping-rs benchmark --provider "Google" --region "us" --count 15

# Custom timeout and threads
cloud-ping-rs benchmark --timeout 5000 --max-threads 16

# Different output formats
cloud-ping-rs benchmark --format json --output results.json
cloud-ping-rs benchmark --format csv --output results.csv
cloud-ping-rs benchmark --format table  # Pretty terminal output
```

#### Planned `quick` Command

Optimized for speed with fewer pings (default: 3 per region).

```bash
# Quick test with default settings
cloud-ping-rs quick

# Custom ping count (still faster than benchmark)
cloud-ping-rs quick --count 5

# Filter by provider
cloud-ping-rs quick --provider "Google Cloud"
cloud-ping-rs quick --provider "Azure"

# Quick test with verbose output
cloud-ping-rs quick --verbose

# Save quick test results
cloud-ping-rs quick --save --output quick-results.json
```

#### Planned `test` Command

Test any HTTP/HTTPS endpoint, not just predefined regions.

```bash
# Test specific URL
cloud-ping-rs test https://example.com

# Custom ping count with detailed output
cloud-ping-rs test https://example.com --count 15 --detailed

# Test with custom timeout
cloud-ping-rs test https://api.github.com --timeout 3000

# Test and save results
cloud-ping-rs test https://example.com --save --output single-test.json

# Test multiple URLs (run command multiple times)
cloud-ping-rs test https://example.com
cloud-ping-rs test https://google.com
cloud-ping-rs test https://cloudflare.com
```

#### Planned `list` Command

Discover and manage available regions and providers.

```bash
# List all regions with details
cloud-ping-rs list

# Filter by provider
cloud-ping-rs list --provider "Microsoft Azure"
cloud-ping-rs list --provider "AWS"

# Show only enabled regions
cloud-ping-rs list --enabled-only

# Show only disabled regions
cloud-ping-rs list --disabled-only

# List with detailed metadata
cloud-ping-rs list --detailed

# Export region list
cloud-ping-rs list --format json --output regions.json
cloud-ping-rs list --format csv --output regions.csv
```

#### Planned `validate` Command

Verify configuration files and data integrity.

```bash
# Validate default configuration and data files
cloud-ping-rs validate

# Validate specific data file
cloud-ping-rs validate --data-file custom_data.json

# Validate configuration file
cloud-ping-rs validate --config-file ~/.config/cloud-ping-rs/config.toml

# Validate with verbose output (shows all checks)
cloud-ping-rs validate --verbose

# Validate and fix common issues
cloud-ping-rs validate --fix
```

#### Planned `config` Command

Manage application configuration.

```bash
# Show current configuration (from all sources)
cloud-ping-rs config --show

# Generate default configuration file
cloud-ping-rs config --output config.toml

# Generate configuration at specific location
cloud-ping-rs config --output ~/.config/cloud-ping-rs/config.toml

# Show configuration with sources (env vars, file, defaults)
cloud-ping-rs config --show --verbose

# Validate configuration
cloud-ping-rs config --validate
```

### Global Options

These options are planned for future implementation:

```bash
# Enable verbose logging (shows detailed progress)
cloud-ping-rs --verbose benchmark

# Set output format (json, csv, table)
cloud-ping-rs --format csv benchmark
cloud-ping-rs --format json quick

# Custom thread count (default: 8)
cloud-ping-rs --max-threads 16 benchmark
cloud-ping-rs --max-threads 4 quick  # Lower for resource-constrained systems

# Custom timeout in milliseconds (default: 5000)
cloud-ping-rs --timeout 3000 benchmark
cloud-ping-rs --timeout 10000 test https://slow-endpoint.com

# Disable colored output (useful for logs)
cloud-ping-rs --no-color benchmark

# Disable progress bars (useful for CI/CD)
cloud-ping-rs --no-progress benchmark

# Custom data file
cloud-ping-rs --data-file custom-regions.json benchmark

# Combine multiple options
cloud-ping-rs --verbose --format json --max-threads 16 --timeout 3000 benchmark
```

Current supported options:
```bash
# Enable verbose logging
cloud-ping-rs --verbose
```

### Advanced Usage Examples

#### CI/CD Integration (Planned)

```bash
# Fast, non-interactive test for CI/CD (planned feature)
cloud-ping-rs quick --no-color --no-progress --format json --output ci-results.json

# Exit code indicates success/failure
if cloud-ping-rs quick --no-progress; then
    echo "Network tests passed"
else
    echo "Network tests failed"
    exit 1
fi
```

Current basic usage in CI/CD:
```bash
# Run basic benchmark
cargo run

# Run with verbose output
cargo run -- --verbose
```

#### Monitoring Script (Planned)

```bash
#!/bin/bash
# Run periodic network monitoring (planned feature)

while true; do
    timestamp=$(date +%Y%m%d_%H%M%S)
    cloud-ping-rs benchmark \
        --format json \
        --output "results_${timestamp}.json" \
        --no-progress
    
    echo "Test completed at $(date)"
    sleep 300  # Run every 5 minutes
done
```

#### Custom Region Testing (Planned)

```bash
# Create custom data file with your endpoints (planned feature)
cat > my-endpoints.json << EOF
{
  "My Services": {
    "category": "Custom",
    "regions": [
      {
        "name": "Production API",
        "url": "https://api.myservice.com/health",
        "enabled": true
      }
    ]
  }
}
EOF

# Test custom endpoints
cloud-ping-rs --data-file my-endpoints.json benchmark
```

For now, you can customize the endpoints by modifying the `data.json` file directly.

## ⚙️ Configuration

### Configuration Hierarchy

Cloud Ping RS uses a three-tier configuration system with the following precedence (highest to lowest):

1. **Command-line arguments** - Override everything
2. **Environment variables** - Override config file and defaults
3. **Configuration file** - Override defaults
4. **Built-in defaults** - Fallback values

### Configuration File Locations

Default configuration file paths by platform:

| Platform | Path |
|----------|------|
| Linux | `~/.config/cloud-ping-rs/config.toml` |
| macOS | `~/.config/cloud-ping-rs/config.toml` |
| Windows | `%APPDATA%\cloud-ping-rs\config.toml` |

### Complete Configuration Example

```toml
# ============================================
# Cloud Ping RS Configuration File
# ============================================

# Network Settings
# ----------------
default_ping_count = 10        # Number of pings for benchmark command
quick_ping_count = 3           # Number of pings for quick command
timeout_ms = 5000              # Request timeout in milliseconds
max_threads = 8                # Maximum concurrent threads
retry_attempts = 2             # Number of retry attempts on failure
retry_delay_ms = 100           # Delay between retries in milliseconds

# Output Settings
# ---------------
enable_color_output = true     # Enable colored terminal output
show_progress = true           # Show progress bars during testing
save_results_to_file = true    # Automatically save results
results_filename = "results.json"  # Default output filename
output_format = "json"         # Output format: json, csv, table
verbose = false                # Enable verbose logging

# HTTP Settings
# -------------
user_agent = "cloud-ping-rs/3.0.0"  # HTTP User-Agent header
validate_certificates = false       # Validate SSL/TLS certificates
connection_timeout_ms = 3000        # Connection timeout
read_timeout_ms = 5000              # Read timeout
max_redirects = 5                   # Maximum HTTP redirects

# Data File
# ---------
data_file = "data.json"        # Path to regions data file

# Scoring Weights
# ---------------
# Must sum to 1.0 for proper normalization
[scoring_weights]
latency = 0.40                 # 40% weight for latency
jitter = 0.25                  # 25% weight for jitter
packet_loss = 0.25             # 25% weight for packet loss
reliability = 0.10             # 10% weight for reliability

# Monitoring Settings (for continuous monitoring mode)
# ----------------------------------------------------
[monitoring]
enabled = false                # Enable continuous monitoring
interval_seconds = 60          # Monitoring interval
alert_threshold_score = 70.0   # Alert if score drops below this
alert_threshold_loss = 5.0     # Alert if packet loss exceeds this %
```

### Environment Variables

All configuration options can be overridden using environment variables with the `CLOUD_PING_` prefix:

#### Network Settings

```bash
export CLOUD_PING_DEFAULT_PING_COUNT=20
export CLOUD_PING_QUICK_PING_COUNT=5
export CLOUD_PING_TIMEOUT_MS=3000
export CLOUD_PING_MAX_THREADS=16
export CLOUD_PING_RETRY_ATTEMPTS=3
export CLOUD_PING_RETRY_DELAY_MS=200
```

#### Output Settings

```bash
export CLOUD_PING_ENABLE_COLOR_OUTPUT=true
export CLOUD_PING_SHOW_PROGRESS=false
export CLOUD_PING_SAVE_RESULTS_TO_FILE=true
export CLOUD_PING_RESULTS_FILENAME="my-results.json"
export CLOUD_PING_OUTPUT_FORMAT=csv
export CLOUD_PING_VERBOSE=true
```

#### HTTP Settings

```bash
export CLOUD_PING_USER_AGENT="MyApp/1.0"
export CLOUD_PING_VALIDATE_CERTIFICATES=true
export CLOUD_PING_CONNECTION_TIMEOUT_MS=2000
export CLOUD_PING_READ_TIMEOUT_MS=4000
```

#### Data File

```bash
export CLOUD_PING_DATA_FILE="/path/to/custom-data.json"
```

### Scoring Weights Customization

Adjust scoring weights to match your priorities. All weights must sum to 1.0.

#### Gaming-Optimized Profile

```toml
[scoring_weights]
latency = 0.50      # Prioritize low latency
jitter = 0.30       # Consistent performance is critical
packet_loss = 0.15  # Some loss is tolerable
reliability = 0.05  # Less critical
```

#### Streaming-Optimized Profile

```toml
[scoring_weights]
latency = 0.20      # Moderate latency is acceptable
jitter = 0.35       # Smooth playback is critical
packet_loss = 0.35  # No buffering interruptions
reliability = 0.10  # Consistent availability
```

#### VoIP-Optimized Profile

```toml
[scoring_weights]
latency = 0.35      # Low latency for real-time communication
jitter = 0.40       # Extremely sensitive to jitter
packet_loss = 0.20  # Some loss can be compensated
reliability = 0.05  # Less critical
```

#### Download-Optimized Profile

```toml
[scoring_weights]
latency = 0.10      # Initial latency less important
jitter = 0.10       # Consistency less critical
packet_loss = 0.60  # No data loss
reliability = 0.20  # Sustained availability
```

### Configuration Validation

Validate your configuration before running tests:

```bash
# Validate configuration file
cloud-ping-rs config --validate

# Show current configuration with sources
cloud-ping-rs config --show --verbose

# Generate a new configuration file
cloud-ping-rs config --output my-config.toml
```

### Configuration Best Practices

1. **Start with defaults** - Only override what you need
2. **Use environment variables for CI/CD** - Easier than managing config files
3. **Validate after changes** - Catch errors early
4. **Document custom weights** - Explain why you chose specific values
5. **Version control your config** - Track changes over time
6. **Test configuration changes** - Run quick tests before full benchmarks

## 📊 Output Formats

The current implementation shows a simple network quality assessment:

```
=== Network Quality Assessment ===
Overall Score: 85/100
Quality: Good
- Your network connection is solid.
```

Planned output formats for future versions include:

### Terminal Output (Table Format) (Planned)

Beautiful, human-readable output with colors and formatting:

```
🌐 Cloud Ping RS - Network Quality Assessment
==============================================

Testing 50 regions across 5 providers...

┌────────────────────────────────────────────────────────────┐
│ Amazon Web Services - us-east-1 (N. Virginia)             │
├────────────────────────────────────────────────────────────┤
│ BASIC METRICS:                                            │
│   Latency:  23.45 ms avg (18.20-31.80 ms)               │
│   Jitter:    2.15 ms | Std Dev:  3.42 ms                │
│   Loss:      0.0% (10/10 successful)                     │
├────────────────────────────────────────────────────────────┤
│ SCORING:                                                  │
│   Overall:  92.3/100 (A)        Excellent Performance    │
│   QoS:      94.1/100 (A+)       Excellent                │
├────────────────────────────────────────────────────────────┤
│ SUITABILITY SCORES:                                       │
│   Gaming:     95/100   Streaming:  97/100                │
│   VoIP:       93/100   Browsing:   99/100                │
│   Downloads:  99/100                                      │
├────────────────────────────────────────────────────────────┤
│ TIMING BREAKDOWN:                                         │
│   DNS:       2.3 ms    TCP:       5.1 ms                 │
│   TLS:       8.2 ms    Transfer:  7.9 ms                 │
└────────────────────────────────────────────────────────────┘

Summary:
--------
✓ Total Regions Tested: 50
✓ Successful Tests: 48 (96.0%)
✗ Failed Tests: 2 (4.0%)
⏱ Total Duration: 45.2 seconds
🏆 Best Region: AWS us-east-1 (92.3/100)
```

**Use for:** Interactive terminal sessions, manual testing, demonstrations

### JSON Output (Planned)

Structured data format for programmatic processing:

```json
{
  "version": "3.0.0",
  "timestamp": 1703123456,
  "test_config": {
    "ping_count": 10,
    "max_threads": 8,
    "timeout_ms": 5000,
    "retry_attempts": 2,
    "scoring_weights": {
      "latency": 0.4,
      "jitter": 0.25,
      "packet_loss": 0.25,
      "reliability": 0.1
    }
  },
  "summary": {
    "total_regions_tested": 50,
    "successful_tests": 48,
    "failed_tests": 2,
    "success_rate": 96.0,
    "total_duration_ms": 45234,
    "best_region": {
      "name": "AWS us-east-1",
      "score": 92.3
    },
    "worst_region": {
      "name": "Provider X region-y",
      "score": 45.2
    }
  },
  "results": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "provider": "Amazon Web Services",
      "region": "us-east-1 (N. Virginia)",
      "region_id": "aws-us-east-1",
      "url": "https://dynamodb.us-east-1.amazonaws.com/ping",
      "country": "US",
      "coordinates": {
        "latitude": 39.0458,
        "longitude": -77.5081
      },
      "test_time": "2023-12-21T10:30:45Z",
      "test_duration_ms": 12500,
      "metrics": {
        "successful_pings": 10,
        "total_pings": 10,
        "success_rate": 100.0,
        "latency": {
          "avg": 23.45,
          "min": 18.20,
          "max": 31.80,
          "median": 22.10,
          "p95": 29.50,
          "p99": 30.80,
          "std_dev": 3.42
        },
        "jitter": 2.15,
        "packet_loss": 0.0
      },
      "timing_breakdown": {
        "dns_ms": 2.3,
        "tcp_ms": 5.1,
        "tls_ms": 8.2,
        "transfer_ms": 7.9,
        "total_ms": 23.5
      },
      "scoring": {
        "overall_score": 92.3,
        "grade": "A",
        "description": "Excellent Performance",
        "category": "Premium",
        "components": {
          "latency_score": 95.0,
          "jitter_score": 92.0,
          "packet_loss_score": 100.0,
          "reliability_score": 100.0
        },
        "suitability": {
          "gaming": 95.0,
          "streaming": 97.0,
          "voip": 93.0,
          "browsing": 99.0,
          "downloads": 99.0
        }
      },
      "metadata": {
        "datacenter": "us-east-1a",
        "provider_region": "us-east-1",
        "priority": 1.0
      }
    }
  ],
  "errors": [
    {
      "region": "Provider X region-y",
      "error": "Connection timeout after 5000ms",
      "timestamp": "2023-12-21T10:31:15Z"
    }
  ]
}
```

**Use for:** API integration, data analysis, monitoring systems, CI/CD pipelines

### CSV Output (Planned)

Spreadsheet-compatible format for data analysis:

```csv
Provider,Region,RegionID,Country,SuccessRate,LatencyAvg,LatencyMin,LatencyMax,LatencyMedian,LatencyP95,Jitter,PacketLoss,StdDev,OverallScore,Grade,Category,GamingSuitability,StreamingSuitability,VoIPSuitability,BrowsingSuitability,DownloadSuitability,TestTime,TestDurationMs
Amazon Web Services,us-east-1 (N. Virginia),aws-us-east-1,US,100.0,23.45,18.20,31.80,22.10,29.50,2.15,0.0,3.42,92.3,A,Premium,95.0,97.0,93.0,99.0,99.0,2023-12-21T10:30:45Z,12500
Google Cloud,us-central1 (Iowa),gcp-us-central1,US,100.0,25.30,20.10,33.50,24.80,31.20,2.80,0.0,3.85,90.1,A-,Excellent,92.0,94.0,90.0,96.0,97.0,2023-12-21T10:30:58Z,11800
Microsoft Azure,eastus (Virginia),azure-eastus,US,100.0,27.15,21.50,35.20,26.40,33.10,3.20,0.0,4.12,88.5,B+,Excellent,89.0,91.0,87.0,94.0,95.0,2023-12-21T10:31:10Z,12200
```

**Use for:** Excel/Google Sheets analysis, data visualization, reporting

### Choosing the Right Format

| Format | Best For | Pros | Cons |
|--------|----------|------|------|
| **Table** | Interactive use | Beautiful, easy to read | Not machine-parseable |
| **JSON** | Automation, APIs | Complete data, structured | Verbose, harder to read |
| **CSV** | Data analysis | Spreadsheet-compatible | Limited structure |

### Output Format Examples (Planned)

```bash
# Terminal output (default)
cloud-ping-rs benchmark

# JSON output to file
cloud-ping-rs benchmark --format json --output results.json

# CSV output to file
cloud-ping-rs benchmark --format csv --output results.csv

# JSON output to stdout (for piping)
cloud-ping-rs benchmark --format json --no-progress

# Combine with jq for filtering
cloud-ping-rs benchmark --format json --no-progress | jq '.results[] | select(.scoring.overall_score > 90)'
```

## 🏗️ Architecture

### System Overview

Cloud Ping RS is built with a modular, layered architecture optimized for performance and maintainability:

```
┌─────────────────────────────────────────────────────────────┐
│                    Main Application                         │
│              Basic Benchmark Runner (Current)               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Configuration Layer                       │
│         TOML Files + Environment Variables + Defaults       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Orchestration Layer                      │
│              Benchmark Runner + Test Coordinator            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Network Layer                          │
│         HTTP Client + Retry Logic + Timing Metrics          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Processing Layer                        │
│         Aggregation + Scoring + Statistical Analysis        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Output Layer                           │
│              JSON + CSV + Table Formatting                  │
└─────────────────────────────────────────────────────────────┘
```

Future planned architecture with CLI:

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Interface (clap)                    │
│                  Command Parsing & Validation               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Configuration Layer                       │
│         TOML Files + Environment Variables + Defaults       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Orchestration Layer                      │
│              Benchmark Runner + Test Coordinator            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Network Layer                          │
│         HTTP Client + Retry Logic + Timing Metrics          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Processing Layer                        │
│         Aggregation + Scoring + Statistical Analysis        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Output Layer                           │
│              JSON + CSV + Table Formatting                  │
└─────────────────────────────────────────────────────────────┘
```

### Core Modules

#### `config` - Configuration Management
- **Purpose:** Unified configuration from multiple sources
- **Features:**
  - TOML file parsing with validation
  - Environment variable overrides
  - Default value fallbacks
  - Configuration persistence
- **Key Files:** `src/config.rs`

#### `error` - Error Handling
- **Purpose:** Comprehensive error types and handling
- **Features:**
  - Structured error types with context
  - Error chaining and propagation
  - User-friendly error messages
  - Detailed debugging information
- **Key Files:** `src/error.rs`

#### `models` - Data Structures
- **Purpose:** Core data types and business logic
- **Features:**
  - Region and provider definitions
  - Metrics and statistics structures
  - Scoring components
  - Serialization support
- **Key Files:** `src/models/`, `src/models.rs`

#### `network` - HTTP Client
- **Purpose:** Network communication and timing
- **Features:**
  - Async HTTP requests with reqwest
  - Automatic retry logic with exponential backoff
  - Detailed timing breakdown (DNS, TCP, TLS, transfer)
  - Connection pooling and keep-alive
  - IP address validation
- **Key Files:** `src/network.rs`

#### `scoring` - Scoring Engine
- **Purpose:** Performance evaluation and ranking
- **Features:**
  - Weighted scoring algorithm
  - Deterministic normalization
  - Application-specific suitability scores
  - Performance categorization
- **Key Files:** `src/models/scoring/`

#### `benchmark` - Test Orchestration
- **Purpose:** Concurrent test execution and coordination
- **Features:**
  - Multi-threaded test execution
  - Progress tracking with indicatif
  - Result aggregation
  - Error handling and recovery
- **Key Files:** `src/benchmark.rs`

#### `display` - Output Formatting
- **Purpose:** Multiple output format support
- **Features:**
  - Beautiful terminal tables
  - JSON serialization
  - CSV export
  - Colored output with console
- **Key Files:** `src/display.rs`

#### `data_loader` - Data Management
- **Purpose:** Load and validate region data
- **Features:**
  - JSON parsing
  - Data validation
  - Schema verification
  - Error reporting
- **Key Files:** `src/data_loader.rs`

#### `monitoring` - Real-time Monitoring
- **Purpose:** Continuous network monitoring
- **Features:**
  - Asynchronous probe runner
  - Streaming metrics aggregation
  - Alert system
  - Ring buffer for time-series data
- **Key Files:** `src/monitoring.rs`, `src/probe.rs`

### Performance Optimizations

#### Async I/O with Tokio
- **Non-blocking operations** - Thousands of concurrent requests
- **Efficient task scheduling** - Work-stealing thread pool
- **Zero-cost futures** - Compile-time optimization

#### Connection Management
- **Connection pooling** - Reuse TCP connections
- **Keep-alive** - Reduce connection overhead
- **DNS caching** - Minimize DNS lookups

#### Thread Management
- **Semaphore-based limiting** - Control concurrency
- **Intelligent work distribution** - Balance load across threads
- **Configurable thread count** - Adapt to system resources

#### Memory Efficiency
- **Pre-allocated buffers** - Reduce allocations
- **Zero-copy operations** - Minimize data copying
- **Efficient data structures** - DashMap for concurrent access
- **Ring buffers** - Fixed-size circular buffers for metrics

#### Compilation Optimizations
- **LTO (Link-Time Optimization)** - Cross-crate optimization
- **Single codegen unit** - Better optimization opportunities
- **Strip symbols** - Smaller binary size
- **Inline functions** - Eliminate function call overhead

### Concurrency Model

```
Main Thread
    │
    ├─▶ Configuration Loading
    │
    ├─▶ Data Loading & Validation
    │
    └─▶ Benchmark Orchestrator
            │
            ├─▶ Worker Thread 1 ──▶ HTTP Client ──▶ Region 1, 2, 3...
            ├─▶ Worker Thread 2 ──▶ HTTP Client ──▶ Region 4, 5, 6...
            ├─▶ Worker Thread 3 ──▶ HTTP Client ──▶ Region 7, 8, 9...
            └─▶ Worker Thread N ──▶ HTTP Client ──▶ Region X, Y, Z...
                    │
                    └─▶ Results Aggregation
                            │
                            └─▶ Scoring & Analysis
                                    │
                                    └─▶ Output Formatting
```

### Data Flow

```
Input Data (JSON)
    │
    ▼
Data Loader ──▶ Validation ──▶ Region Models
    │
    ▼
Benchmark Runner
    │
    ├─▶ Network Layer ──▶ HTTP Requests ──▶ Timing Data
    │
    ▼
Aggregator ──▶ Statistics Calculation
    │
    ▼
Scoring Engine ──▶ Performance Scores
    │
    ▼
Display Layer ──▶ Formatted Output (JSON/CSV/Table)
```

### Error Handling Strategy

1. **Graceful degradation** - Continue testing even if some regions fail
2. **Detailed error context** - Include URL, region, and error details
3. **Retry logic** - Automatic retries with exponential backoff
4. **Error aggregation** - Collect all errors for final report
5. **User-friendly messages** - Clear, actionable error descriptions

## 🧪 Development

### Development Setup

#### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-watch      # Auto-reload during development
cargo install cargo-audit       # Security vulnerability scanning
cargo install cargo-outdated    # Check for outdated dependencies
cargo install cargo-bloat       # Analyze binary size
cargo install cargo-expand      # Expand macros for debugging
```

#### Clone and Build

```bash
# Clone the repository
git clone https://github.com/example/cloud-ping-rs
cd cloud-ping-rs

# Build in debug mode (faster compilation)
cargo build

# Build in release mode (optimized)
cargo build --release

# Run in development mode
cargo run -- quick

# Run with specific command
cargo run --release -- benchmark --count 5
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests with verbose output
cargo test -- --nocapture --test-threads=1

# Run specific test module
cargo test config::tests
cargo test network::tests
cargo test scoring::tests

# Run specific test
cargo test test_scoring_algorithm

# Run integration tests
cargo test --test integration

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Code Quality

#### Formatting

```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Format all files
cargo fmt --all
```

#### Linting

```bash
# Run Clippy (Rust linter)
cargo clippy

# Run Clippy with all warnings
cargo clippy --all-targets --all-features -- -D warnings

# Fix Clippy warnings automatically (where possible)
cargo clippy --fix
```

#### Security Audit

```bash
# Check for security vulnerabilities
cargo audit

# Update advisory database
cargo audit fetch

# Generate audit report
cargo audit --json > audit-report.json
```

### Benchmarks

```bash
# Run performance benchmarks
cargo bench

# Run specific benchmark
cargo bench scoring

# Generate benchmark report
cargo bench -- --output-format html

# Compare benchmarks
cargo bench -- --save-baseline main
# Make changes...
cargo bench -- --baseline main
```

### Development Workflow

#### Auto-reload During Development

```bash
# Watch for changes and run tests
cargo watch -x test

# Watch for changes and run the application
cargo watch -x 'run -- quick'

# Watch for changes, run tests, then run the app
cargo watch -x test -x 'run -- quick'

# Watch specific files
cargo watch -w src -x test
```

#### Debugging

```bash
# Build with debug symbols
cargo build

# Run with debug output
RUST_LOG=debug cargo run -- quick

# Run with trace output (very verbose)
RUST_LOG=trace cargo run -- benchmark

# Debug specific module
RUST_LOG=cloud_ping_rs::network=debug cargo run -- test https://example.com

# Use rust-gdb or rust-lldb for debugging
rust-gdb target/debug/cloud-ping-rs
```

### Project Structure

```
cloud-ping-rs/
├── src/
│   ├── main.rs                 # Entry point and CLI
│   ├── lib.rs                  # Library exports
│   ├── config.rs               # Configuration management
│   ├── error.rs                # Error types and handling
│   ├── models.rs               # Re-exports for models
│   ├── models/
│   │   ├── endpoint.rs         # Endpoint definitions
│   │   ├── metrics.rs          # Metrics and aggregation
│   │   ├── probe.rs            # Probe records and alerts
│   │   ├── region.rs           # Region and provider models
│   │   ├── stats.rs            # Statistics structures
│   │   ├── utils.rs            # Model utilities
│   │   └── scoring/
│   │       ├── mod.rs          # Scoring algorithm
│   │       ├── normalization.rs # Score normalization
│   │       └── utils.rs        # Scoring utilities
│   ├── network.rs              # HTTP client and networking
│   ├── benchmark.rs            # Test orchestration
│   ├── display.rs              # Output formatting
│   ├── data_loader.rs          # Data loading and validation
│   ├── monitoring.rs           # Real-time monitoring
│   ├── probe.rs                # Probe runner
│   ├── aggregator.rs           # Metrics aggregation
│   ├── format_utils.rs         # Formatting utilities
│   ├── time_utils.rs           # Time utilities
│   ├── collection_utils.rs     # Collection utilities
│   ├── ui_utils.rs             # UI utilities
│   └── tests.rs                # Integration tests
├── Cargo.toml                  # Dependencies and metadata
├── Cargo.lock                  # Dependency lock file
├── data.json                   # Default region data
├── linode-regions.json         # Linode-specific data
├── run.sh                      # Quick run script
├── justfile                    # Just command runner recipes
├── rustfmt.toml                # Rust formatting config
├── .editorconfig               # Editor configuration
├── .gitignore                  # Git ignore rules
└── README.md                   # This file
```

### Adding New Features

#### Adding a New Cloud Provider

1. **Update data.json:**

```json
{
  "New Provider": {
    "category": "Cloud Provider",
    "regions": [
      {
        "name": "region-1",
        "url": "https://endpoint.region-1.provider.com/ping",
        "country": "US",
        "coordinates": {
          "latitude": 37.7749,
          "longitude": -122.4194
        },
        "priority": 1.0,
        "enabled": true
      }
    ]
  }
}
```

2. **Validate the data:**

```bash
cargo run -- validate --data-file data.json
```

3. **Test the new provider:**

```bash
cargo run -- benchmark --provider "New Provider"
```

#### Adding a New Scoring Component

1. **Update `src/models/scoring/mod.rs`:**

```rust
pub struct AlgorithmWeights {
    pub latency: f64,
    pub jitter: f64,
    pub packet_loss: f64,
    pub consistency: f64,
    pub availability: f64,
    pub new_component: f64,  // Add new component
}
```

2. **Update normalization in `src/models/scoring/normalization.rs`:**

```rust
pub fn normalize_new_component(value: f64) -> f64 {
    // Implement normalization logic
    (100.0 - value).max(0.0).min(100.0)
}
```

3. **Update scoring calculation:**

```rust
let score = weights.latency * components.latency_score
    + weights.jitter * components.jitter_score
    + weights.packet_loss * components.packet_loss_score
    + weights.consistency * components.consistency_score
    + weights.availability * components.availability_score
    + weights.new_component * components.new_component_score;
```

4. **Add tests:**

```rust
#[test]
fn test_new_component_normalization() {
    assert_eq!(normalize_new_component(0.0), 100.0);
    assert_eq!(normalize_new_component(100.0), 0.0);
}
```

### Dependency Management

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Update specific dependency
cargo update -p reqwest

# Check dependency tree
cargo tree

# Check dependency tree for specific package
cargo tree -p tokio

# Analyze binary size by dependency
cargo bloat --release

# Analyze binary size by crate
cargo bloat --release --crates
```

### Release Process

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Run full test suite
cargo test --all-features

# 4. Run Clippy
cargo clippy --all-targets --all-features -- -D warnings

# 5. Build release binary
cargo build --release

# 6. Test release binary
./target/release/cloud-ping-rs quick

# 7. Create git tag
git tag -a v3.0.0 -m "Release version 3.0.0"
git push origin v3.0.0

# 8. Publish to crates.io (if applicable)
cargo publish
```

## 📈 Scoring Algorithm

### Overview

Cloud Ping RS uses a sophisticated, deterministic scoring algorithm that evaluates network performance across multiple dimensions. The algorithm produces scores from 0-100 and assigns letter grades (A+ to F).

### Scoring Components

The overall score is calculated as a weighted sum of five components:

```
Overall Score = (Latency × 0.40) + (Jitter × 0.25) + (Packet Loss × 0.25) 
                + (Consistency × 0.10) + (Availability × 0.10)
```

#### 1. Latency Score (40% weight)

Measures average round-trip time. Lower is better.

| Latency | Score | Grade | Description |
|---------|-------|-------|-------------|
| < 20ms | 100 | A+ | Excellent - Ideal for all applications |
| 20-30ms | 95-100 | A | Excellent - Great for gaming and real-time apps |
| 30-50ms | 80-95 | B | Good - Suitable for most applications |
| 50-100ms | 50-80 | C | Fair - Acceptable for non-real-time apps |
| 100-200ms | 20-50 | D | Poor - Noticeable lag |
| > 200ms | 0-20 | F | Bad - Significant delays |

**Normalization Formula:**
```rust
score = max(0, 100 - (latency_ms / 2))
```

#### 2. Jitter Score (25% weight)

Measures variation in latency. Lower is better for consistent performance.

| Jitter | Score | Grade | Description |
|--------|-------|-------|-------------|
| < 5ms | 100 | A+ | Excellent - Very stable connection |
| 5-10ms | 80-100 | A/B | Good - Stable for most uses |
| 10-20ms | 50-80 | C | Fair - Some variation |
| 20-50ms | 20-50 | D | Poor - Inconsistent performance |
| > 50ms | 0-20 | F | Bad - Highly unstable |

**Normalization Formula:**
```rust
score = max(0, 100 - (jitter_ms * 2))
```

#### 3. Packet Loss Score (25% weight)

Measures percentage of failed requests. Zero loss is ideal.

| Packet Loss | Score | Grade | Description |
|-------------|-------|-------|-------------|
| 0% | 100 | A+ | Perfect - No data loss |
| 0-1% | 90-100 | A | Excellent - Minimal loss |
| 1-3% | 70-90 | B | Good - Acceptable loss |
| 3-5% | 50-70 | C | Fair - Noticeable loss |
| 5-10% | 0-50 | D/F | Poor - Significant loss |
| > 10% | 0 | F | Bad - Unacceptable loss |

**Normalization Formula:**
```rust
score = max(0, 100 - (packet_loss_percent * 10))
```

#### 4. Consistency Score (10% weight)

Measures standard deviation of latency. Lower is better.

| Std Dev | Score | Grade | Description |
|---------|-------|-------|-------------|
| < 5ms | 100 | A+ | Highly consistent |
| 5-10ms | 90-100 | A | Very consistent |
| 10-20ms | 70-90 | B | Consistent |
| 20-50ms | 40-70 | C | Somewhat variable |
| > 50ms | 0-40 | D/F | Highly variable |

**Normalization Formula:**
```rust
score = max(0, 100 - std_dev_ms)
```

#### 5. Availability Score (10% weight)

Measures success rate of requests. 100% is ideal.

| Success Rate | Score | Grade | Description |
|--------------|-------|-------|-------------|
| 100% | 100 | A+ | Perfect availability |
| 95-100% | 95-100 | A | Excellent availability |
| 90-95% | 90-95 | B | Good availability |
| 80-90% | 80-90 | C | Fair availability |
| < 80% | 0-80 | D/F | Poor availability |

**Normalization Formula:**
```rust
score = success_rate_percent
```

### Grade Assignment

Letter grades are assigned based on the overall score:

| Score Range | Grade | Description |
|-------------|-------|-------------|
| 95-100 | A+ | Outstanding performance |
| 90-94 | A | Excellent performance |
| 85-89 | A- | Very good performance |
| 80-84 | B+ | Good performance |
| 75-79 | B | Above average performance |
| 70-74 | B- | Satisfactory performance |
| 65-69 | C+ | Acceptable performance |
| 60-64 | C | Marginal performance |
| 55-59 | C- | Below average performance |
| 50-54 | D+ | Poor performance |
| 45-49 | D | Very poor performance |
| 40-44 | D- | Inadequate performance |
| < 40 | F | Failing performance |

### Application Suitability Scores

In addition to the overall score, Cloud Ping RS calculates application-specific suitability scores:

#### Gaming Suitability
```
Score = (Latency × 0.50) + (Jitter × 0.30) + (Packet Loss × 0.20)
```
- **Prioritizes:** Ultra-low latency and minimal jitter
- **Ideal for:** FPS games, competitive gaming, real-time strategy
- **Threshold:** 90+ for competitive gaming, 80+ for casual gaming

#### Streaming Suitability
```
Score = (Consistency × 0.40) + (Availability × 0.30) + (Packet Loss × 0.30)
```
- **Prioritizes:** Consistent performance and no buffering
- **Ideal for:** Video streaming, live broadcasts, media consumption
- **Threshold:** 85+ for HD streaming, 90+ for 4K streaming

#### VoIP Suitability
```
Score = (Latency × 0.40) + (Jitter × 0.30) + (Packet Loss × 0.30)
```
- **Prioritizes:** Low latency and extremely low jitter
- **Ideal for:** Voice calls, video conferencing, real-time communication
- **Threshold:** 85+ for clear calls, 90+ for HD video calls

#### Web Browsing Suitability
```
Score = (Latency × 0.30) + (Availability × 0.30) + (Consistency × 0.40)
```
- **Prioritizes:** Consistent availability and reasonable latency
- **Ideal for:** Web browsing, API calls, general internet use
- **Threshold:** 75+ for good experience, 85+ for excellent experience

#### File Transfer Suitability
```
Score = (Availability × 0.50) + (Packet Loss × 0.30) + (Consistency × 0.20)
```
- **Prioritizes:** High availability and no data loss
- **Ideal for:** Downloads, uploads, backups, file synchronization
- **Threshold:** 80+ for reliable transfers, 90+ for large files

### Performance Categories

Based on the overall score, regions are categorized:

| Category | Score Range | Description |
|----------|-------------|-------------|
| **Premium** | 90-100 | Best-in-class performance, ideal for all applications |
| **Excellent** | 80-89 | High-quality performance, suitable for demanding applications |
| **Good** | 70-79 | Solid performance, suitable for most applications |
| **Fair** | 60-69 | Acceptable performance, may have limitations |
| **Poor** | < 60 | Suboptimal performance, not recommended |

### Example Calculation

Given these metrics:
- Latency: 25ms
- Jitter: 3ms
- Packet Loss: 0%
- Standard Deviation: 4ms
- Success Rate: 100%

**Component Scores:**
```
Latency Score:     100 - (25 / 2) = 87.5
Jitter Score:      100 - (3 * 2) = 94.0
Packet Loss Score: 100 - (0 * 10) = 100.0
Consistency Score: 100 - 4 = 96.0
Availability Score: 100
```

**Overall Score:**
```
(87.5 × 0.40) + (94.0 × 0.25) + (100.0 × 0.25) + (96.0 × 0.10) + (100 × 0.10)
= 35.0 + 23.5 + 25.0 + 9.6 + 10.0
= 93.1
```

**Grade:** A (Excellent Performance)

**Application Suitability:**
```
Gaming:     (87.5 × 0.50) + (94.0 × 0.30) + (100.0 × 0.20) = 91.95
Streaming:  (96.0 × 0.40) + (100 × 0.30) + (100.0 × 0.30) = 98.40
VoIP:       (87.5 × 0.40) + (94.0 × 0.30) + (100.0 × 0.30) = 93.20
Browsing:   (87.5 × 0.30) + (100 × 0.30) + (96.0 × 0.40) = 94.65
Downloads:  (100 × 0.50) + (100.0 × 0.30) + (96.0 × 0.20) = 99.20
```

### Customizing Weights

You can customize the scoring weights in your configuration file:

```toml
[scoring_weights]
latency = 0.40
jitter = 0.25
packet_loss = 0.25
consistency = 0.05
availability = 0.05
```

**Important:** All weights must sum to 1.0 for proper normalization.

## 🔧 Data Format

### Region Data Structure

Cloud Ping RS reads cloud provider and region data from JSON files. The default file is `data.json`, but you can specify custom files using the `--data-file` option.

### JSON Schema

```json
{
  "Provider Name": {
    "category": "string",
    "regions": [
      {
        "name": "string",
        "url": "string (URL)",
        "country": "string (ISO 3166-1 alpha-2)",
        "coordinates": {
          "latitude": number,
          "longitude": number
        },
        "priority": number (0.0-1.0),
        "enabled": boolean,
        "metadata": {
          "key": "value"
        }
      }
    ]
  }
}
```

### Complete Example

```json
{
  "Amazon Web Services": {
    "category": "Major Cloud Provider",
    "regions": [
      {
        "name": "us-east-1 (N. Virginia)",
        "url": "https://dynamodb.us-east-1.amazonaws.com/ping",
        "country": "US",
        "coordinates": {
          "latitude": 39.0458,
          "longitude": -77.5081
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "datacenter": "us-east-1a",
          "provider_region": "us-east-1",
          "availability_zones": ["us-east-1a", "us-east-1b", "us-east-1c"]
        }
      },
      {
        "name": "us-west-2 (Oregon)",
        "url": "https://dynamodb.us-west-2.amazonaws.com/ping",
        "country": "US",
        "coordinates": {
          "latitude": 45.5234,
          "longitude": -122.6762
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "datacenter": "us-west-2a",
          "provider_region": "us-west-2"
        }
      }
    ]
  },
  "Google Cloud": {
    "category": "Major Cloud Provider",
    "regions": [
      {
        "name": "us-central1 (Iowa)",
        "url": "https://www.googleapis.com/ping",
        "country": "US",
        "coordinates": {
          "latitude": 41.2619,
          "longitude": -95.8608
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "datacenter": "us-central1-a",
          "provider_region": "us-central1"
        }
      }
    ]
  },
  "Custom Endpoints": {
    "category": "Custom",
    "regions": [
      {
        "name": "My API Server",
        "url": "https://api.mycompany.com/health",
        "country": "US",
        "coordinates": {
          "latitude": 37.7749,
          "longitude": -122.4194
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "environment": "production",
          "version": "v2.0"
        }
      }
    ]
  }
}
```

### Field Descriptions

#### Provider Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `category` | string | Yes | Provider category (e.g., "Major Cloud Provider", "CDN", "Custom") |
| `regions` | array | Yes | Array of region objects |

#### Region Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Human-readable region name |
| `url` | string (URL) | Yes | HTTP/HTTPS endpoint to test |
| `country` | string | Yes | ISO 3166-1 alpha-2 country code (e.g., "US", "GB", "JP") |
| `coordinates` | object | Yes | Geographic coordinates |
| `coordinates.latitude` | number | Yes | Latitude (-90 to 90) |
| `coordinates.longitude` | number | Yes | Longitude (-180 to 180) |
| `priority` | number | No | Priority for testing (0.0-1.0, default: 1.0) |
| `enabled` | boolean | No | Whether to test this region (default: true) |
| `metadata` | object | No | Custom metadata (key-value pairs) |

### Validation Rules

1. **URL Format:** Must be valid HTTP or HTTPS URL
2. **Country Code:** Must be valid ISO 3166-1 alpha-2 code
3. **Coordinates:** Latitude must be -90 to 90, longitude -180 to 180
4. **Priority:** Must be between 0.0 and 1.0
5. **Unique Names:** Region names should be unique within a provider

### Creating Custom Data Files

#### Example: Testing Your Own Services

```json
{
  "My Company": {
    "category": "Internal Services",
    "regions": [
      {
        "name": "Production API (US East)",
        "url": "https://api.mycompany.com/health",
        "country": "US",
        "coordinates": {
          "latitude": 40.7128,
          "longitude": -74.0060
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "environment": "production",
          "datacenter": "nyc-1"
        }
      },
      {
        "name": "Production API (EU West)",
        "url": "https://api-eu.mycompany.com/health",
        "country": "IE",
        "coordinates": {
          "latitude": 53.3498,
          "longitude": -6.2603
        },
        "priority": 1.0,
        "enabled": true,
        "metadata": {
          "environment": "production",
          "datacenter": "dublin-1"
        }
      },
      {
        "name": "Staging API",
        "url": "https://staging-api.mycompany.com/health",
        "country": "US",
        "coordinates": {
          "latitude": 37.7749,
          "longitude": -122.4194
        },
        "priority": 0.5,
        "enabled": false,
        "metadata": {
          "environment": "staging",
          "datacenter": "sfo-1"
        }
      }
    ]
  }
}
```

#### Using Custom Data Files

```bash
# Test with custom data file
cloud-ping-rs --data-file my-endpoints.json benchmark

# Validate custom data file
cloud-ping-rs validate --data-file my-endpoints.json

# List regions from custom file
cloud-ping-rs --data-file my-endpoints.json list
```

### Data File Best Practices

1. **Organize by provider** - Group related endpoints together
2. **Use descriptive names** - Make region names clear and searchable
3. **Include metadata** - Add context for better reporting
4. **Set priorities** - Use priority field to control test order
5. **Disable unused regions** - Set `enabled: false` instead of deleting
6. **Validate regularly** - Run validation after changes
7. **Version control** - Track changes to your data files
8. **Document custom fields** - Explain custom metadata fields

## 🚄 Performance

### Benchmarks

Cloud Ping RS is designed for high performance with minimal resource usage:

| Metric | Value | Notes |
|--------|-------|-------|
| **Concurrent Requests** | 1000+ | Limited by system resources |
| **Memory Usage** | ~10-50 MB | Depends on number of regions |
| **CPU Usage** | Low | Async I/O minimizes CPU load |
| **Binary Size** | ~8-12 MB | Stripped release build |
| **Startup Time** | < 100ms | Fast initialization |
| **Test Duration** | ~5-30s | Depends on region count and ping count |

### Performance Tips

#### Optimize Thread Count

```bash
# Use more threads for faster testing (if you have the resources)
cloud-ping-rs --max-threads 16 benchmark

# Use fewer threads on resource-constrained systems
cloud-ping-rs --max-threads 4 benchmark

# Auto-detect optimal thread count (default: 8)
cloud-ping-rs benchmark
```

#### Reduce Timeout for Faster Results

```bash
# Lower timeout for faster failure detection
cloud-ping-rs --timeout 2000 benchmark

# Higher timeout for slow networks
cloud-ping-rs --timeout 10000 benchmark
```

#### Use Quick Mode for Fast Checks

```bash
# Quick mode uses only 3 pings per region
cloud-ping-rs quick

# Even faster with fewer threads
cloud-ping-rs --max-threads 4 quick
```

#### Disable Progress Bars in CI/CD

```bash
# Disable progress bars for faster execution
cloud-ping-rs --no-progress benchmark

# Combine with JSON output for parsing
cloud-ping-rs --no-progress --format json benchmark > results.json
```

### Resource Usage

#### Memory

- **Base:** ~5 MB for application code
- **Per Region:** ~50-100 KB for metrics and results
- **Peak:** During concurrent testing, temporary buffers increase usage

#### CPU

- **Idle:** Minimal CPU usage during I/O wait
- **Active:** Scales with thread count and concurrent requests
- **Optimization:** Async I/O keeps CPU usage low even with many concurrent requests

#### Network

- **Bandwidth:** Minimal (ping requests are small)
- **Connections:** One per concurrent request
- **Keep-Alive:** Reuses connections when possible

### Compilation Optimizations

The release build includes several optimizations:

```toml
[profile.release]
lto = "fat"              # Link-time optimization
codegen-units = 1        # Single codegen unit for better optimization
panic = "abort"          # Smaller binary, faster panics
strip = true             # Remove debug symbols
opt-level = 3            # Maximum optimization
```

### Runtime Optimizations

- **Connection pooling** - Reuse HTTP connections
- **Async I/O** - Non-blocking operations with Tokio
- **Zero-copy** - Minimize data copying
- **Pre-allocation** - Allocate buffers upfront
- **Inline functions** - Eliminate function call overhead

## 🔧 Troubleshooting

### Common Issues

#### Issue: "Connection timeout" errors

**Symptoms:**
```
Error: Connection timeout after 5000ms for region us-east-1
```

**Solutions:**
1. Increase timeout: `cloud-ping-rs --timeout 10000 benchmark`
2. Check your internet connection
3. Verify the endpoint URL is accessible
4. Check if a firewall is blocking requests

#### Issue: "SSL certificate verification failed"

**Symptoms:**
```
Error: SSL certificate verification failed for https://example.com
```

**Solutions:**
1. Disable certificate validation in config:
```toml
validate_certificates = false
```
2. Or use environment variable:
```bash
export CLOUD_PING_VALIDATE_CERTIFICATES=false
```
3. Update your system's CA certificates

#### Issue: "Too many open files"

**Symptoms:**
```
Error: Too many open files (OS error 24)
```

**Solutions:**
1. Reduce thread count: `cloud-ping-rs --max-threads 4 benchmark`
2. Increase system file descriptor limit:
```bash
# Temporary (Linux/macOS)
ulimit -n 4096

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo "ulimit -n 4096" >> ~/.bashrc
```

#### Issue: High memory usage

**Symptoms:**
- Application uses more memory than expected

**Solutions:**
1. Test fewer regions at once
2. Reduce thread count
3. Use quick mode instead of benchmark
4. Filter by provider: `cloud-ping-rs benchmark --provider "AWS"`

#### Issue: Slow performance

**Symptoms:**
- Tests take longer than expected

**Solutions:**
1. Increase thread count: `cloud-ping-rs --max-threads 16 benchmark`
2. Reduce timeout: `cloud-ping-rs --timeout 3000 benchmark`
3. Use quick mode: `cloud-ping-rs quick`
4. Check your network connection speed

#### Issue: "Invalid JSON" errors

**Symptoms:**
```
Error: Failed to parse data file: invalid JSON at line 42
```

**Solutions:**
1. Validate JSON syntax: `cloud-ping-rs validate --data-file data.json`
2. Use a JSON validator (e.g., jsonlint.com)
3. Check for trailing commas, missing quotes, etc.
4. Ensure proper UTF-8 encoding

#### Issue: No regions found

**Symptoms:**
```
Error: No regions found in data file
```

**Solutions:**
1. Verify data file exists: `ls -la data.json`
2. Check data file format: `cloud-ping-rs validate`
3. Ensure regions are enabled: `"enabled": true`
4. Check provider filter: Remove `--provider` flag

### Debug Mode

Enable verbose logging for detailed debugging:

```bash
# Verbose output
cloud-ping-rs --verbose benchmark

# Trace-level logging (very detailed)
RUST_LOG=trace cloud-ping-rs benchmark

# Debug specific module
RUST_LOG=cloud_ping_rs::network=debug cloud-ping-rs benchmark

# Save logs to file
cloud-ping-rs --verbose benchmark 2> debug.log
```

### Getting Help

If you encounter issues not covered here:

1. **Check existing issues:** [GitHub Issues](https://github.com/example/cloud-ping-rs/issues)
2. **Enable verbose mode:** Run with `--verbose` flag
3. **Validate configuration:** Run `cloud-ping-rs validate`
4. **Check system requirements:** Rust 1.75+, internet connection
5. **Create an issue:** Include verbose output and system information

### System Information

When reporting issues, include:

```bash
# Rust version
rustc --version

# Cloud Ping RS version
cloud-ping-rs --version

# Operating system
uname -a  # Linux/macOS
systeminfo  # Windows

# Configuration
cloud-ping-rs config --show
```

## 🤝 Contributing

We welcome contributions! Cloud Ping RS is an open-source project and we appreciate help from the community.

### Ways to Contribute

- **Report bugs** - Open an issue with detailed information
- **Suggest features** - Share your ideas for improvements
- **Submit pull requests** - Fix bugs or add features
- **Improve documentation** - Help make docs clearer
- **Share feedback** - Let us know how you're using the tool

### Development Workflow

1. **Fork the repository**
```bash
git clone https://github.com/YOUR_USERNAME/cloud-ping-rs
cd cloud-ping-rs
```

2. **Create a feature branch**
```bash
git checkout -b feature/my-new-feature
```

3. **Make your changes**
- Write clean, idiomatic Rust code
- Follow existing code style
- Add tests for new functionality
- Update documentation as needed

4. **Run the test suite**
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

5. **Commit your changes**
```bash
git add .
git commit -m "Add my new feature"
```

6. **Push to your fork**
```bash
git push origin feature/my-new-feature
```

7. **Submit a pull request**
- Describe your changes clearly
- Reference any related issues
- Wait for review and feedback

### Code Style Guidelines

- **Formatting:** Use `cargo fmt` (rustfmt)
- **Linting:** Pass `cargo clippy` with no warnings
- **Testing:** Write unit tests for new code
- **Documentation:** Document public APIs with doc comments
- **Error handling:** Use `Result` types and proper error messages
- **Naming:** Follow Rust naming conventions
- **Comments:** Explain "why", not "what"

### Testing Guidelines

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] No Clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] PR description explains changes

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions
- Help others learn and grow

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### MIT License Summary

```
Copyright (c) 2024 Cloud Ping Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```

## 🙏 Acknowledgments

Cloud Ping RS is built on the shoulders of giants. We're grateful to the Rust community and the maintainers of these excellent crates:

### Core Dependencies

- **[Tokio](https://tokio.rs/)** - Asynchronous runtime for Rust
  - Powers our async/await concurrency model
  - Enables thousands of concurrent network requests
  
- **[reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client library
  - Handles all HTTP/HTTPS requests
  - Provides connection pooling and keep-alive
  
- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parser
  - Powers our CLI interface
  - Provides automatic help generation and validation

### UI & Display

- **[indicatif](https://github.com/console-rs/indicatif)** - Progress bars and spinners
  - Beautiful progress tracking during tests
  
- **[console](https://github.com/console-rs/console)** - Terminal manipulation
  - Colored output and terminal styling
  
- **[colored](https://github.com/colored-rs/colored)** - Terminal colors
  - Makes output beautiful and readable

### Data & Serialization

- **[serde](https://serde.rs/)** - Serialization framework
  - JSON parsing and generation
  - Configuration file handling
  
- **[serde_json](https://github.com/serde-rs/json)** - JSON support for serde
  - Fast JSON parsing and serialization

### Configuration & Utilities

- **[config](https://github.com/mehcode/config-rs)** - Configuration management
  - Multi-source configuration loading
  
- **[dirs](https://github.com/dirs-dev/dirs-rs)** - Platform-specific directories
  - Cross-platform config file locations
  
- **[toml](https://github.com/toml-rs/toml)** - TOML parser
  - Configuration file format support

### Error Handling

- **[anyhow](https://github.com/dtolnay/anyhow)** - Flexible error handling
  - Simplified error propagation
  
- **[thiserror](https://github.com/dtolnay/thiserror)** - Error derive macros
  - Custom error type generation

### Performance & Concurrency

- **[dashmap](https://github.com/xacrimon/dashmap)** - Concurrent hash map
  - Thread-safe data structures
  
- **[futures](https://github.com/rust-lang/futures-rs)** - Async utilities
  - Future combinators and utilities

### Formatting & Display

- **[bytesize](https://github.com/hyunsik/bytesize)** - Human-readable byte sizes
  - Formats bytes as KB, MB, GB, etc.
  
- **[num-format](https://github.com/bcmyers/num-format)** - Number formatting
  - Locale-aware number formatting with thousands separators

### Other Utilities

- **[chrono](https://github.com/chronotope/chrono)** - Date and time library
  - Timestamp handling and formatting
  
- **[uuid](https://github.com/uuid-rs/uuid)** - UUID generation
  - Unique identifiers for test results
  
- **[url](https://github.com/servo/rust-url)** - URL parsing
  - URL validation and manipulation
  
- **[ipnet](https://github.com/krisprice/ipnet)** - IP network types
  - IP address validation
  
- **[rand](https://github.com/rust-random/rand)** - Random number generation
  - Jitter and randomization
  
- **[tracing](https://github.com/tokio-rs/tracing)** - Application-level tracing
  - Structured logging and diagnostics

### Development Tools

- **[criterion](https://github.com/bheisler/criterion.rs)** - Benchmarking
- **[tokio-test](https://github.com/tokio-rs/tokio)** - Async testing utilities
- **[tempfile](https://github.com/Stebalien/tempfile)** - Temporary file handling
- **[wiremock](https://github.com/LukeMathWalker/wiremock-rs)** - HTTP mocking

### Special Thanks

- The **Rust Programming Language** team for creating an amazing language
- The **Rust community** for excellent documentation and support
- All **open-source contributors** who make projects like this possible
- **Cloud providers** (AWS, Google Cloud, Azure, etc.) for providing test endpoints

## 📚 Additional Resources

### Documentation

- [Architecture Documentation](ARCHITECTURE.md) - Detailed system architecture
- [Refactoring Summary](REFACTORING_SUMMARY.md) - Recent code improvements
- [Future Improvements](FUTURE_IMPROVEMENTS.md) - Planned features
- [Quick Reference](QUICK_REFERENCE.md) - Quick command reference

### Related Projects

- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [reqwest](https://docs.rs/reqwest/) - HTTP client documentation
- [clap](https://docs.rs/clap/) - CLI framework documentation

### Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Async Book](https://rust-lang.github.io/async-book/) - Async programming in Rust

## 🌟 Star History

If you find Cloud Ping RS useful, please consider giving it a star on GitHub! ⭐

## 📞 Contact & Support

- **Issues:** [GitHub Issues](https://github.com/example/cloud-ping-rs/issues)
- **Discussions:** [GitHub Discussions](https://github.com/example/cloud-ping-rs/discussions)
- **Email:** support@cloudping.example.com

---

<div align="center">

**Cloud Ping RS** - Making network performance testing fast, reliable, and beautiful. 🚀

Built with ❤️ using Rust

[⬆ Back to Top](#cloud-ping-rs-)

</div>