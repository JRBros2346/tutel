mod app;
mod pipeline;
mod registry;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app::run()
}
