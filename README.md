# portfolio_website

First project on web development.

## Deploy on Render

1. Push the latest repository changes to GitHub.
2. In Render, click **New +** → **Web Service**.
3. Select `SHREE-PRAVEEN/portfolio_website`.
4. Use these settings:
   - **Runtime**: Rust
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/portfolio_website`
5. Deploy the service.

The app automatically reads Render's `PORT` environment variable and binds to `0.0.0.0:$PORT`.

## Verify deployment

- Open the Render service URL and confirm `/` loads.
- Open `<your-render-url>/api/visits` and confirm JSON response.
