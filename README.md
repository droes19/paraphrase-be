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
- [Shuttle CLI](https://docs.shuttle.rs/introduction/installation) (for deployment)

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

### Deployment to Shuttle

1. Install the Shuttle CLI:
   ```
   cargo install cargo-shuttle
   ```

2. Login to Shuttle:
   ```
   cargo shuttle login
   ```

3. Deploy the application:
   ```
   cargo shuttle deploy
   ```

4. During the first deployment, you'll be prompted to set up your secrets:
   - AI_API_KEY: Your Anthropic API key
   - FRONTEND_URL: The URL of your deployed frontend

5. Once deployed, Shuttle will provide a URL for your API.

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
- ✅ Shuttle deployment configuration

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