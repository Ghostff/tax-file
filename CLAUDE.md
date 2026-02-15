# Rental Core

## Overview
Brief description of the rental core project.

### Technical Stack

#### Client
- Vue 3
- Node 24 (SSR)

#### Server
- Rust 1.93

#### Database
- Postgres

#### Infrastructure
- Containerized (Docker)
- AWS (ECS, RDS, S3, CloudFront, Route53, ACM)

## Getting Started

### Prerequisites
- List any dependencies or requirements

### Installation
```bash
# Installation commands
```

### Usage
```bash
# Usage examples
```

## Project Structure
```
/src - API server and routes
/frontend - Client-side dashboard
```


## Project Guidelines (frontend)

### TypeScript Rules (Strict – No Exceptions)

* **All Vue files MUST be written in TypeScript**
    * `.vue` files must use `<script setup lang="ts">`
    * No `any` types are allowed
    * No implicit typing for API data

* **All data fetched from APIs MUST be typed**
    * Responses must use explicitly defined interfaces
    * Inline object typing is not allowed for models

### Models & Interfaces

* All model interfaces **MUST** be defined under:
  ```
  frontend/types/Models/
  ```

* Folder structure should mirror domain concepts:
  ```
  types/
    Models/
      user.ts
      rental.ts
      booking.ts
  ```

* **All interfaces must start with `I`**

* **One primary model per file**

#### Example: User Model

```ts
// frontend/types/Models/user.ts

export interface IUser {
  id: string
  first_name: string
  last_name: string
  email: string
  phone: string | null
  created_at: string
  updated_at: string
}
```

#### Usage Example

```ts
import type { IUser } from '@/types/Models/user'

const user = ref<IUser | null>(null)
```

### API Layer Typing

* API functions must always return typed results
* Use generics where applicable
* Axios / fetch responses must be wrapped with proper interfaces

```ts
async function fetchUser(id: string): Promise<IUser> {
  return await axios.get<IUser>(`/users/${id}`)
}
```

### Props, Emits, and Slots

* All props must be explicitly typed
* All emits must be typed using `defineEmits`
* Slot props must be typed when exposed

```ts
const props = defineProps<{
  user: IUser
}>()
```

### State Management

* Store state must be fully typed
* No untyped reactive objects
* Prefer `interface` over `type` for models

---

## Project Guidelines (src)

### Rust (Actix web) Rules (Strict – No Exceptions)
ALL Rust commands MUST be executed inside the Docker container.
- Never run raw `cargo` commands directly on the host machine.
- Always execute them from the container. Example:
  ```bash
  docker compose exec src cargo check
  docker compose exec src cargo build
  docker compose exec src cargo test
  ```

This project uses a Makefile for common tasks. When performing actions.
 - Always prefer Makefile targets over raw shell commands. Examples, Use `make create-migration` instead of manually creating migration files 
 - If a required task exists as a Makefile target, use it. If unsure, ask before inventing new commands.
  This project is fully containerized.

### Architecture

* **Models (`src/models`)**: All database structures and schemas must be stored here. This includes domain models, request/response schemas, and validation logic.
* **Repositories (`src/repositories`)**: Database queries should ONLY be performed here.
* **Services (`src/services`)**: Business logic should be placed here.
* **Macros (`src/macros`)**: Custom Rust macros should be placed here.
* **Helpers (`src/helpers`)**: Utility functions and helper modules should be placed here.

### Database Access (Strict – No Exceptions)

* **All database queries must go through the repository layer** (no SQL in controllers, services, middleware, or utilities).
* **Use SQLx macros (`query!`, `query_as!`, `query_scalar!`) wherever possible** to ensure compile-time validation.

### Testing
 - All tests must be placed under the top-level tests/ directory. 
 - No inline `#[cfg(test)]` modules are allowed.
 - All tests must be named with the `*_test.rs` pattern.
 - Test files must mirror the feature being tested
 - Tests must not be placed inside controllers, services, models, repositories. 
 - Required Structure
    ```
    src/
      controllers/
      services/
    
    tests/
      integration_controller_test.rs
      facility_unit_service_test.rs
    ```
- Run tests with: `cargo test` inside `src`.
