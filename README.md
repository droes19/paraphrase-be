# Paraphrase Backend (paraphrase-be)

The Rust backend for the Paraphrase application, a text paraphrasing tool that leverages AI to rewrite text while preserving the original meaning.

## Features

- RESTful API for text paraphrasing
- Integration with Anthropic's Claude API for high-quality paraphrasing
- CORS setup for frontend communication
- Error handling and validation
- Support for environment variables

## Technology Stack

- Rust (2021 edition)
- Axum web framework
- Tokio async runtime
- Serde for serialization/deserialization
- reqwest for HTTP requests
- tower-http for middleware

## API Endpoints

- `GET /` - Health check endpoint
- `POST /api/paraphrase` - Paraphrase the provided text

## Project Structure

```
backend/
└── src/
    ├── main.rs       # Application entry point
    ├── routes.rs     # API route handlers
    ├── models.rs     # Data structures
    ├── error.rs      # Error handling
    └── services/     # Service layer
        ├── mod.rs    # Service module definition
        └── ai.rs     # AI service for paraphrasing
```

## Getting Started

### Prerequisites

- Rust (2021 edition) and Cargo
- An Anthropic API key (Claude)
- [Shuttle CLI](https://docs.shuttle.rs/introduction/installation) version matching your project's dependencies (for deployment)

### Local Development

1. Clone the repository:
   ```
   git clone <repository-url>
   cd paraphrase-be
   ```

2. Create a `.env` file from the template:
   ```
   cp .env.example .env
   ```

3. Edit the `.env` file to add your Anthropic API key:
   ```
   AI_API_KEY=your_anthropic_api_key_here
   ```

4. Build and run the server locally:
   ```
   cargo run
   ```

5. The server will start on [http://localhost:8080](http://localhost:8080).

### Deployment Options

The backend can be deployed to various platforms. Here are several options:

#### Option 1: Deploy with Docker

1. Build the Docker image:
   ```
   docker build -t paraphrase-be .
   ```

2. Run the container locally:
   ```
   docker run -p 8080:8080 -e AI_API_KEY=your_api_key_here paraphrase-be
   ```

3. Deploy to a platform that supports Docker:
   - Railway
   - Fly.io
   - Render
   - DigitalOcean App Platform
   - AWS App Runner
   - Google Cloud Run

#### Option 2: Deploy to Heroku

1. Create a new Heroku app:
   ```
   heroku create
   ```

2. Set up environment variables:
   ```
   heroku config:set AI_API_KEY=your_anthropic_api_key_here
   heroku config:set FRONTEND_URL=https://paraprhase-fe.vercel.app
   ```

3. Deploy the app:
   ```
   git push heroku main
   ```

#### Option 3: Manual Deployment

1. Build the project:
   ```
   cargo build --release
   ```

2. Create a `.env` file with your configuration:
   ```
   cp .env.example .env
   # Edit .env with your API key
   ```

3. Run the server:
   ```
   ./target/release/paraphrase-be
   ```

**Important Environment Variables:**
- `AI_API_KEY`: Your Anthropic API key (required)
- `AI_API_URL`: The Anthropic API URL (defaults to "https://api.anthropic.com/v1/messages")
- `FRONTEND_URL`: Your frontend URL (defaults to "http://localhost:3000" in development)
- `PORT`: The port to run the server on (defaults to 8080)

## API Usage

### Paraphrase Text

**Endpoint:** `POST /api/paraphrase`

**Request:**
```json
{
  "text": "The text you want to paraphrase"
}
```

**Response:**
```json
{
  "paraphrasedText": "The paraphrased version of your text"
}
```

## Error Handling

The API returns appropriate HTTP status codes for different types of errors:

- `400 Bad Request` - Invalid input (e.g., empty text)
- `503 Service Unavailable` - AI service error
- `500 Internal Server Error` - Other server errors

## Development Progress

### Completed
- ✅ Basic project structure and configuration
- ✅ API endpoint for paraphrasing
- ✅ Integration with Anthropic Claude API
- ✅ Error handling and validation
- ✅ CORS configuration for frontend communication
- ✅ Docker and cloud deployment configuration

### In Progress
- 🔄 Comprehensive test coverage
- 🔄 API documentation with Swagger/OpenAPI

### Planned
- 📝 Rate limiting and request throttling
- 📝 User authentication (optional)
- 📝 Usage statistics and logging
- 📝 Multiple AI model options

## License

This project is licensed under the MIT License - see the LICENSE file for details.