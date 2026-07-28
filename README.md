# Engineering Documents Manager

Engineering Documents Manager is a Rust-based web service for managing engineering documentation, design components, and publication workflows. It provides REST endpoints for creating and querying documents, linking layouts, publishing documents, and managing reusable document components.

## Installation

Build the project with Cargo:

```bash
cargo build --release
```

## Usage

Run the application:

```bash
cargo run --release
```

By default, the server listens on `127.0.0.1:3000` and uses a local SQLite database at database.db.

### Configuration

Environment variables can be used to customize runtime settings:

- `HOST` - server host (default: `127.0.0.1`)
- `PORT` - server port (default: `3000`)
- `DB_URL` - database URL (default: `sqlite://data/database.db`)
- `EXPORT_DIR` - directory for generated export files (default: export)

## API Endpoints

### Documents

- `POST /documents` - create a new document
- `PUT /documents/{id}/layouts` - update document layouts by linking component IDs
- `GET /documents` - list documents with optional filters such as `status`
- `GET /documents/{id}` - fetch a document by ID
- `PATCH /documents/{id}` - partially update document metadata
- `DELETE /documents/{id}` - delete a document and its layouts
- `POST /documents/{id}/publish` - publish a document

### Components

- `POST /components` - create a new engineering component
- `GET /components` - list latest components with optional filters such as `group`
- `GET /components/{id}` - fetch a component by ID
- `PUT /components/{id}` - update a component by ID
- `DELETE /components/{id}` - remove a component by ID

## Development

Use the following command to run the server during development:

```bash
cargo run
```
