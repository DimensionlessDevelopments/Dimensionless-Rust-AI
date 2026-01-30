# Dimensionless-Rust-AI
Welcome to Dimensionless Rust Ai 🚀.This is an open source tutorial to show you the power of the Rust programming language &amp; AI. An advanced AI research agent built with Rust and cutting-edge technology. You will experience fast, reliable, and powerful AI-powered research at your fingertips.


🔍 AI Research Agent

A full-stack AI research agent built with **Rust** backend and **Leptos** WebAssembly frontend. This project combines a CLI tool for AI research with a modern web interface, demonstrating the complete AI agent architecture from LLM integration to real-time web communication.

[Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
[License](https://img.shields.io/badge/License-MIT-blue.svg)
[AI](https://img.shields.io/badge/AI-Ollama-green.svg)
[Frontend](https://img.shields.io/badge/Frontend-Leptos%2BTailwind-blue.svg)
[WebSocket](https://img.shields.io/badge/Real--Time-WebSocket-brightgreen.svg)


## ✨ Features

- 🤖 **Local LLM Support** - Uses Ollama for privacy-friendly, free AI inference
- 🔎 **Web Search Integration** - DuckDuckGo integration (no API key required!)
- 🛠️ **Tool-Using Agent** - Demonstrates agentic AI patterns with function calling
- 💻 **Web Interface** - Modern Leptos + TailwindCSS frontend with real-time WebSocket chat
- 🔄 **Async Architecture** - Tokio-based backend with concurrent request handling
- 🔌 **Custom Tool System** - Extensible tool framework for adding AI capabilities
- 📚 **Beginner Friendly** - Extensive comments explaining Rust and AI concepts
- 🚀 **Production Ready** - Proper error handling, logging, and deployment-ready builds

## 🏗️ Tech Stack

### Backend Technologies
| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Web Server** | [Axum](https://github.com/tokio-rs/axum) v0.7 | HTTP + WebSocket server framework |
| **Async Runtime** | [Tokio](https://tokio.rs/) v1 | Async/await runtime with full feature set |
| **AI Agent Framework** | [Rig Core](https://rig.rs/) v0.27 | LLM integration and tool-using agents |
| **HTTP Client** | [Reqwest](https://github.com/seanmonstar/reqwest) v0.12 | Async HTTP requests |
| **Web Search** | [DuckDuckGo Search](https://crates.io/crates/duckduckgo_search) v0.1 | Privacy-respecting search integration |
| **Serialization** | [Serde](https://serde.rs/) + JSON | Data serialization with custom deserializers |
| **Error Handling** | [Anyhow](https://docs.rs/anyhow/), [ThisError](https://docs.rs/thiserror/) | Error types and context propagation |
| **Logging** | [Tracing](https://docs.rs/tracing/) + [Tracing Subscriber](https://docs.rs/tracing-subscriber/) | Structured logging and debugging |
| **CLI** | [Clap](https://docs.rs/clap/) v4 | Command-line argument parsing |

### Frontend Technologies
| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Framework** | [Leptos](https://leptos.dev/) v0.6 | Full-stack Rust web framework (CSR mode) |
| **Routing** | [Leptos Router](https://docs.rs/leptos_router/) v0.6 | Client-side routing |
| **Styling** | [TailwindCSS](https://tailwindcss.com/) 2.2.19 (CDN) | Utility-first CSS framework |
| **Real-Time** | [WebSocket API](https://docs.rs/web-sys/) via gloo-net | Bidirectional real-time communication |
| **Compilation** | [Trunk](https://trunkrs.io/) v0.21.14 | WASM bundler and development server |
| **WASM** | [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) | JS/Rust interop for WebAssembly |

### Development & Deployment
- **Language**: Rust 2021 Edition
- **Local LLM**: [Ollama](https://ollama.ai/) (llama3.2 or compatible models)
- **Version Control**: Git
- **Architecture**: Dual-server (Frontend: 8080, Backend: 3000)

## 🚀 Quick Start for Beginners

### Step 1: Install Prerequisites (10 minutes)

**Windows:**
```powershell
# Install Rust (includes Cargo)
# Visit https://rustup.rs/ and run the installer, OR:
winget install rustup

# Install Ollama
winget install ollama

# Verify installations
rustc --version
ollama --version
```

**macOS:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Ollama
brew install ollama

# Verify
rustc --version
ollama --version
```

**Linux (Ubuntu/Debian):**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Verify
rustc --version
ollama --version
```

### Step 2: Download a Model (10-15 minutes)

```bash
# Open a terminal and start Ollama (runs in background)
ollama serve

# In another terminal, download a model (first run takes a while - it's ~4.7GB)
ollama pull llama3.2

# Verify it's installed
ollama list
# Should show: llama3.2   2.0GB   ...
```

**First time tip**: The first `ollama pull` downloads the model (slow). Subsequent runs use the cached version (instant).

### Step 3: Clone and Build the Project (5-10 minutes)

```bash
# Clone this repository
git clone this repositry
cd Dimensionless-Rust-AI-

# Build the backend (Rust compilation, takes ~2-3 minutes first time)
cargo build --release

# Install Trunk for frontend builds (one-time)
cargo install trunk

# Build the frontend (takes ~1-2 minutes first time)
trunk build --release
```

**What's happening**:
- `cargo build --release` compiles Rust code with optimizations
- `trunk build --release` compiles Rust to WebAssembly for the browser

### Step 4: Run the Application

**Option A: Web Interface (Recommended for Beginners)**

Terminal 1 - Start Backend:
```bash
cd Dimensionless-Rust-AI-
cargo run --release -- --web --port 3000
```

Terminal 2 - Start Frontend:
```bash
cd Dimensionless-Rust-AI-
trunk serve --release
```

Then open browser to: **http://localhost:8080**

**Option B: CLI Tool**

```bash
# Simple query
cargo run -- "What is Rust?"

# Longer query
cargo run -- "What are the latest developments in AI agents?"

# With specific model
OLLAMA_MODEL=llama3.2 cargo run -- "Your question here"

# Show all options
cargo run -- --help
```

### Step 5: Try It Out!

**Example Queries to Test**:
1. "What are the latest AI frameworks?" - Tests web search + synthesis
2. "Explain quantum computing" - Tests complex reasoning
3. "Best Rust web frameworks 2024" - Tests timely information retrieval

**Watch it Work**:
- You'll see the agent thinking...
- It searches the web (via DuckDuckGo)
- LLM synthesizes results into a coherent answer
- Web interface shows real-time response streaming

### Step 6: Understanding the Code

**Best file to start with**: [src/main.rs](src/main.rs)
- Shows CLI argument parsing
- Demonstrates how to create an agent
- Shows how to call the agent

**Next file**: [src/tools.rs](src/tools.rs)
- Shows how tools work
- Implements web search (shortest tool example)
- Demonstrates custom deserializers

**Advanced**: [src/agent.rs](src/agent.rs)
- Shows agent loop implementation
- Demonstrates streaming responses
- Shows error handling patterns

**Frontend**: [src/components/](src/components/)
- Leptos components for UI
- WebSocket communication
- Real-time message updates

## 🔧 Environment Configuration

Create a `.env` file in the project root:

```bash
# Required: Which model to use (must be installed via ollama)
OLLAMA_MODEL=llama3.2

# Optional: Ollama server location (default: localhost:11434)
OLLAMA_HOST=http://localhost:11434

# Optional: Response creativity (0.0 = deterministic, 1.0 = random)
TEMPERATURE=0.7

# Optional: How many search results to use
MAX_SEARCH_RESULTS=5

# Optional: Logging level (debug, info, warn, error)
RUST_LOG=info
```

## 📁 Project Structure

```
Dimensionless-Rust-AI-/
├── Cargo.toml              # Dependencies and project metadata
├── .env.example            # Environment variable template
├── README.md               # This file
├── index.html              # Frontend entry point (WASM)
├── style/
│   └── custom.css          # Custom CSS (hover effects, animations)
└── src/
    ├── main.rs             # CLI entry point and application logic
    ├── config.rs           # Configuration management
    ├── agent.rs            # AI agent loop implementation
    ├── tools.rs            # Web search tool (DuckDuckGo)
    ├── app.rs              # Leptos app component (WebSocket setup)
    └── components/
        ├── landing.rs      # Marketing landing page
        ├── chat_area.rs    # Message display component
        └── type_area.rs    # Input field component
```



## 🎓 Rust Concepts Used

### 1. **Ownership & Borrowing** (Memory Safety)
```rust
// Function takes ownership of string
fn process_query(query: String) { ... }

// Borrows reference - caller keeps ownership
fn analyze_results(results: &[String]) { ... }
```
**Used in**: Message ownership in WebSocket handler, search result processing

### 2. **Async/Await** (Non-blocking I/O)
```rust
async fn search_web(query: &str) -> Result<SearchResults> {
    let response = reqwest::Client::new()
        .get(SEARCH_URL)
        .send()
        .await?  // Yield to runtime if slow
        .json()
        .await?
}
```
**Used in**: Backend HTTP requests, LLM inference, WebSocket communication

### 3. **Traits** (Polymorphism)
```rust
pub trait Tool {
    const NAME: &'static str;
    fn description(&self) -> String;
    async fn execute(&self, input: String) -> Result<String>;
}

// Custom implementation
impl Tool for WebSearchTool { ... }
```
**Used in**: Tool system allowing agents to use multiple tools

### 4. **Custom Deserializers** (Serde Macros)
```rust
#[derive(Deserialize)]
pub struct SearchArgs {
    #[serde(deserialize_with = "deserialize_search_query")]
    pub query: String,
}

fn deserialize_search_query<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    // Custom parsing logic to handle multiple formats
}
```
**Used in**: Tool parameter handling, LLM response deserialization

### 5. **Enums & Pattern Matching** (Type Safety)
```rust
enum Message {
    UserMessage { text: String, timestamp: DateTime<Utc> },
    ModelMessage { text: String, timestamp: DateTime<Utc> },
}

match message {
    Message::UserMessage { text, .. } => { /* render user message */ },
    Message::ModelMessage { text, .. } => { /* render model response */ },
}
```
**Used in**: Message routing, error handling, configuration options

### 6. **Result & Error Handling** (Railway-Oriented)
```rust
// The ? operator chains errors automatically
async fn handle_request(query: String) -> Result<String> {
    let search_results = web_search(&query).await?;  // Propagate error if fails
    let synthesized = llm_synthesize(search_results).await?;
    Ok(synthesized)
}
```
**Used in**: Error propagation throughout application

### 7. **Macros & Derives** (Code Generation)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]  // Auto-implement traits
pub struct ChatMessage {
    sender: String,
    content: String,
}

// Use tracing macros for structured logging
debug!("Processing query: {}", query);
error!("Failed to fetch: {:?}", err);
```
**Used in**: Logging, serialization, trait derivation

### 8. **Generics & Type Parameters** (Code Reusability)
```rust
async fn process<T: AsRef<str>>(input: T) -> Result<String> {
    let text = input.as_ref();
    // Works with String, &str, or any type implementing AsRef<str>
}
```
**Used in**: Flexible API design, stream processing

### 9. **Reactive State Management** (Leptos)
```rust
// Frontend reactive signals
let (message_count, set_message_count) = create_signal(0);

// Auto-rerun when signal changes
create_effect(move || {
    log_analytics(message_count.get());
});
```
**Used in**: Frontend message updates, auto-scroll behavior

### 10. **WebAssembly (WASM)** (Browser Execution)
```rust
// Compiles to WebAssembly binary for browser execution
#[component]
pub fn ChatInterface() -> impl IntoView {
    // Runs in browser via JavaScript interop
}
```
**Used in**: Entire frontend - interactive UI without JavaScript

## 🤖 AI/ML Concepts Used

### 1. **Agent Pattern** (Autonomous AI)
The application implements the classic agent loop:
```
┌─────────────────┐
│   User Query    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────┐
│  LLM Thinks     │────▶│  Need Tools? │
│ (uses tools)    │     └──────────────┘
└─────────────────┘      Yes │ No
         ▲                   │
         │              ┌────┘
         │              │
         │         ┌────▼──────────┐
         │         │ Execute Tools │
         └─────────│ (Web Search)  │
                   └───────────────┘
```

**File**: [src/agent.rs](src/agent.rs)
**Concepts**:
- Iterative reasoning - LLM decides when to use tools
- Tool selection - Choose relevant tools based on query
- Result synthesis - Combine tool outputs into final answer

### 2. **Function Calling** (Tool-Using LLMs)
The LLM declares intent to use tools via structured parameters:
```json
{
  "tool_name": "web_search",
  "parameters": {
    "query": "latest Rust frameworks"
  }
}
```

**Implementation**: Custom deserializer handles multiple parameter formats from different LLM models (e.g., llama3.2's schema nesting)

**File**: [src/tools.rs](src/tools.rs) - `deserialize_search_query()` function

### 3. **Prompt Engineering** (System Instructions)
The agent uses system prompts to guide LLM behavior:
```rust
const SYSTEM_PROMPT: &str = "You are a research assistant. \
    For queries, search the web and synthesize findings. \
    Always cite sources.";
```

**Concepts**:
- Role definition - "You are a research assistant"
- Task specification - What actions to take
- Output format - How to structure responses

### 4. **RAG Pattern** (Retrieval-Augmented Generation)
```
Query ─────▶ Web Search ─────▶ LLM Synthesis ─────▶ Answer
(retrieval)                  (augmentation)
```

The application retrieves current web data (search results) before generating responses, ensuring answers are grounded in up-to-date information.

**Files**: [src/tools.rs](src/tools.rs) (retrieval), [src/agent.rs](src/agent.rs) (augmentation)

### 5. **Context Window Management** (Token Limits)
LLMs have maximum context windows (e.g., llama3.2 ~8K tokens). The agent:
- Limits number of search results (`MAX_SEARCH_RESULTS=5`)
- Provides concise search summaries
- Preserves conversation history efficiently

### 6. **Temperature & Sampling** (Response Variability)
```rust
let response = client
    .model(&model_name)
    .temperature(0.7)  // 0.0 = focused, 1.0 = creative
    .prompt(query)
    .completion();
```

**Concept**: Controls randomness of LLM outputs for balance between consistency and creativity

### 7. **Structured Output** (JSON Mode)
The tool system expects structured LLM responses:
```json
{
  "tool_name": "web_search",
  "parameters": { "query": "..." }
}
```

Ensures deterministic tool invocation vs. free-form text responses.

### 8. **Model Selection** (Inference Routing)
```bash
# Switch models without code changes
OLLAMA_MODEL=deepseek-v3.2 cargo run -- "query"
```

**Allows testing**:
- Larger models for complex reasoning
- Faster models for latency-sensitive applications
- Specialized models (e.g., code models for programming queries)

### 9. **Real-Time User Interaction** (Streaming)
The web interface streams responses via WebSocket:
```rust
// Backend chunks response and sends incrementally
for chunk in response.chunks(10) {
    send_websocket_message(format!("token: {}", chunk)).await?;
}
```

Improves UX perception of speed and provides interactive experience.

### 10. **Error Recovery** (Fallback Patterns)
The custom deserializer implements multiple fallback strategies:
```rust
// Try direct string first
if let Ok(s) = value.as_str() { return Ok(s.to_string()); }

// Fallback to schema description
if let Some(desc) = value.get("description") { ... }

// Final fallback to toString()
Ok(value.to_string())
```

Ensures robustness across different LLM models and parameter formats.


## 🧪 Testing & Troubleshooting

### "Connection refused" Error
```
Error: connection refused
```
**Solution**: Make sure Ollama is running:
```bash
ollama serve
```

### "Model not found" Error
```
Error: model "llama3.2" not found
```
**Solution**: Pull the model first:
```bash
ollama pull llama3.2
ollama list  # Verify installation
```

### Slow Responses
**Cause**: LLMs are computationally intensive
**Solutions**:
- Try a smaller model: `ollama pull gemma2:2b`
- Check hardware: LLMs need 8GB+ RAM or VRAM
- Use faster models: mistral, neural-chat

### Frontend Not Loading
```
Error: localhost:8080 refused to connect
```
**Solution**: Rebuild and restart frontend:
```bash
trunk build --release
trunk serve --release
```

### WebSocket Connection Failed
**Solution**: Ensure backend is running on port 3000:
```bash
cargo run --release -- --web --port 3000
```

## 🛠️ Extending the Application

### Adding a New AI Tool

1. Create a new tool struct in [src/tools.rs](src/tools.rs):
```rust
pub struct MyTool;

impl Tool for MyTool {
    const NAME: &'static str = "my_tool";
    // ... implement trait methods
}
```

2. Register it in [src/agent.rs](src/agent.rs):
```rust
let agent = client
    .agent(&model)
    .tool(web_search_tool)
    .tool(my_tool)  // Add here
    .build();
```

### Using Different Models

Any Ollama-compatible model works:
```bash
ollama pull mistral
ollama pull codellama
ollama pull gemma2
ollama pull deepseek-v3.2
```

Then either:
- Set in `.env`: `OLLAMA_MODEL=mistral`
- Or pass as flag: `cargo run -- --model mistral "query"`

### Customizing the Frontend

Edit [src/components/](src/components/) files:
- `landing.rs` - Change homepage design
- `chat_area.rs` - Modify message styling
- `type_area.rs` - Customize input field

Rebuild with: `trunk build --release`

## 📚 Learning Resources

### Rust Concepts
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Async Rust](https://rust-lang.github.io/async-book/) - Async/await patterns
- [Trait Objects](https://doc.rust-lang.org/book/ch17-02-using-trait-objects.html) - Dynamic dispatch

### AI Concepts
- [Agent Pattern](https://microsoft.github.io/autogen/docs/concepts/agent/) - How agents work
- [Function Calling](https://platform.openai.com/docs/guides/function-calling) - Tool invocation
- [Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering) - Better prompts
- [RAG Pattern](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/) - Context augmentation

### Frameworks
- [Rig Documentation](https://docs.rs/rig-core/) - AI agent framework
- [Leptos Guide](https://leptos.dev/) - Web framework
- [Axum Guide](https://github.com/tokio-rs/axum) - Web server

## 📊 Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| First Ollama startup | 30-60s | One-time, loads model to VRAM |
| Subsequent startups | <5s | Model already cached |
| Web search | 2-5s | DuckDuckGo API latency |
| LLM response | 5-30s | Depends on model size and hardware |
| Frontend build | 1-2 min | First build slower, cached builds faster |
| WASM binary | ~5MB | Optimized release build |

## 🐛 Common Issues & Solutions

| Issue | Cause | Solution |
|-------|-------|----------|
| "Model not found" | Model not downloaded | `ollama pull llama3.2` |
| Slow responses | Insufficient VRAM | Use smaller model: `gemma2:2b` |
| Frontend won't load | Backend not running | `cargo run --release -- --web --port 3000` |
| WASM memory error | Release build needed | Use `trunk build --release` |
| WebSocket timeout | Long-running operation | Increase timeout in browser DevTools |

## 📜 License

MIT License - feel free to use this for learning and building!

## 🙏 Acknowledgments

- [Rig Framework](https://rig.rs) - The Rust AI agent framework
- [Ollama](https://ollama.ai) - Local LLM runner
- [Leptos](https://leptos.dev/) - Rust web framework
- [DuckDuckGo](https://duckduckgo.com) - Privacy-respecting search

## 📧 Questions?

This project is designed as a learning resource. I have made a lot of comments that explain the code & the rust concepts needed to be understood to implement the code to work. Check the code comments for detailed explanations of each concept!