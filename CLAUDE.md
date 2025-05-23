# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check for compilation errors without building
cargo check

# Run linting and formatting
cargo clippy
cargo fmt
```

## Architecture Overview

This is an AWS Lambda service for managing EC2 instance service discoverability. The service tracks which instances are assigned to handle specific stocks and groups.

### Core Architecture

- **AWS Lambda Handler**: Main entry point in `src/main.rs` that routes GET/PUT requests
- **DynamoDB Storage**: Uses `cloud_util` library for table operations with partition key (PK) as `group` and sort key (SK) as `instance_id`  
- **EC2 Integration**: Fetches instance metadata and tags via `cloud_util::Instance`
- **Request Flow**: Routes → Service Layer → Cloud Utilities (DynamoDB/EC2)

### Key Components

- **InstanceAssignment**: Core data model stored in DynamoDB with expiration timestamps
- **Group**: Enum representing different service groups/environments 
- **Deps**: Dependency injection container with `table_client` and `instance_client`
- **GET /instances/{id}**: Retrieves assignment for instance, validates expiration
- **PUT /instances/{id}**: Creates/updates assignment, validates instance has "App" tag

### Important Patterns

- Instance ID is extracted from URL path parameter
- Group is determined by EC2 instance "App" tag
- All assignments have expiration times checked on retrieval
- Uses structured logging with request IDs and instance IDs
- Error handling distinguishes client vs server errors via `CloudError`

## Environment Variables

- `TABLE_NAME`: DynamoDB table name for storing instance assignments