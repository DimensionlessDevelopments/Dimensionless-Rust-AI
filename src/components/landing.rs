// Dimensionless Developments Rust Ai

use leptos::*;
use leptos_router::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900">
            {/* Navigation */}
            <nav class="border-b border-gray-700 bg-gray-900/80 backdrop-blur">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <div class="text-2xl font-bold text-orange-500">
                            "Dimensionless Rust AI 🚀"
                        </div>
                        <div class="flex space-x-4">
                            <a href="#about" class="text-gray-300 hover:text-orange-500 transition">
                                "About"
                            </a>
                            <a href="#features" class="text-gray-300 hover:text-orange-500 transition">
                                "Features"
                            </a>
                            <a href="/chat" class="text-gray-300 hover:text-orange-500 transition">
                                "Chat Now!"
                            </a>
                        </div>
                    </div>
                </div>
            </nav>

            {/* Hero Section */}
            <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="text-center">
                    <h1 class="text-5xl sm:text-7xl font-bold text-white mb-6">
                        "Welcome to Dimensionless Rust Ai 🚀"
                    </h1>
                    <p class="text-xl text-gray-400 mb-12 max-w-2xl mx-auto">
                        "An advanced AI research agent built with Rust and cutting-edge technology. Experience fast, reliable, and powerful AI-powered research at your fingertips."
                    </p>
                </div>
            </section>

            {/* About Section */}
            <section id="about" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="bg-gray-800/50 backdrop-blur border hover:bg-orange-500 border-gray-700 rounded-lg p-12 transition">
                    <h2 class="text-4xl font-bold text-white mb-6 text-center">
                        "About Dimensionless Rust AI:"
                    </h2>
                    <p class="text-lg text-gray-300 mb-4">
                        "Dimensionless Rust AI (DRA) is a state-of-the-art AI research agent built with Rust, Leptos, and WebAssembly. We combine the performance of Rust with modern web frameworks to deliver seamless AI-powered research."
                    </p>
                    <p class="text-lg text-gray-300 mb-4">
                        "Powered by advanced language models through Ollama, our agent can search the web, understand context, and provide comprehensive research summaries. Whether you need information, analysis, or insights, Dimensionless Rust AI is here to help."
                    </p>
                    <p class="text-lg text-gray-300">
                        "Experience the future of AI-powered research with an agent that's as fast and reliable as Rust itself."
                    </p>
                </div>
            </section>

            {/* Tech-stack Section */}
            <section id="about" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="bg-gray-800/50 backdrop-blur border hover:bg-orange-500 border-gray-700 rounded-lg p-12 transition">
                    <h2 class="text-4xl font-bold text-white mb-6 text-center">
                        "Technical Stack of Dimensionless Rust AI:"
                    </h2>
                    <p class="text-lg text-gray-300 mb-4">
                        "- A full-stack AI research agent built with Rust backend and Leptos WebAssembly frontend. This project combines a CLI tool for AI research with a modern web interface, demonstrating the complete AI agent architecture from LLM integration to real-time web communication."
                    </p>
                    <p class="text-lg text-gray-300 mb-4">
                        "- Uses Ollama for privacy-friendly, free AI inference DuckDuckGo integration (no API key required!)"
                    </p>
                    <p class="text-lg text-gray-300">
                        "- Demonstrates agentic AI patterns with function calling Modern Leptos + TailwindCSS frontend with real-time WebSocket chat."
                    </p>
                    <p class="text-lg text-gray-300">
                        "- Tokio-based backend with concurrent request handling    "
                    </p>
                </div>
            </section>

            {/* Features Section */}
            <section id="features" class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <h2 class="text-4xl font-bold text-white mb-12 text-center">
                    "Why Choose Dimensionless Rust AI?"
                </h2>
                <div class="grid md:grid-cols-3 gap-8">
                    <div class="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-lg p-8 hover:bg-orange-500 hover:border-orange-500 transition">
                        <div class="text-4xl mb-4">
                            "⚡"
                        </div>
                        <h3 class="text-2xl font-bold text-white mb-4">
                            "Lightning Fast"
                        </h3>
                        <p class="text-gray-200">
                            "Built with Rust for exceptional performance. Experience instant responses with local AI models via Ollama."
                        </p>
                    </div>

                    <div class="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-lg p-8 hover:bg-orange-500 hover:border-orange-500 transition">
                        <div class="text-4xl mb-4">
                            "🔒"
                        </div>
                        <h3 class="text-2xl font-bold text-white mb-4">
                            "Private & Secure"
                        </h3>
                        <p class="text-gray-200">
                            "Your data stays on your machine. All AI processing happens locally with no external API calls."
                        </p>
                    </div>

                    <div class="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-lg p-8 hover:bg-orange-500 hover:border-orange-500 transition">
                        <div class="text-4xl mb-4">
                            "🔍"
                        </div>
                        <h3 class="text-2xl font-bold text-white mb-4">
                            "Web Search Integration"
                        </h3>
                        <p class="text-gray-200">
                            "Automatically searches the web and synthesizes information from multiple sources for comprehensive answers."
                        </p>
                    </div>
                </div>
            </section>

            {/* CTA Section */}
            <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="text-center">
                    <h2 class="text-4xl font-bold text-white mb-8">
                        "Ready to Research?"
                    </h2>
                    <p class="text-xl text-gray-400 mb-12">
                        "Start your AI-powered research journey today"
                    </p>
                    <A href="/chat" class="inline-block bg-orange-500 hover:bg-orange-700 text-white font-bold py-4 px-8 rounded-lg text-lg transition transform hover:scale-105">
                        "Start Now"
                    </A>
                </div>
            </section>

            {/* Footer */}
            <footer class="border-t border-gray-700 bg-gray-900/80 mt-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 text-center text-gray-400">
                    <p>
                        "© 2026 Dimensionless Rust AI. Built with Rust, Leptos, and ❤️"
                        <a href="https://www.dimensionlessdevelopments.com" class="text-gray-300 hover:text-orange-500 transition">
                                "Made by Dimensionless Developments"
                        </a>
                    </p>
                </div>
            </footer>
        </div>
    }
}