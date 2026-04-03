# Build the Vue frontend
build-ui:
    cd app-ui && npm run build

# Build the Rust server (release)
build-server:
    cd app-server-rs && cargo build --release

# Build everything
build: build-ui build-server
