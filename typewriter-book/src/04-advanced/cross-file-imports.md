# Cross-File Imports

When a struct references another custom type, imports are auto-generated.

## TypeScript

```rust
pub struct User {
    pub id: String,
}

pub struct UserProfile {
    pub user: User,
    pub bio: String,
}
```

**Generated `user-profile.ts`:**
```typescript
import type { User } from './user';

export interface UserProfile {
  user: User;
  bio: string;
}
```

## Python

**Generated `user_profile.py`:**
```python
from .user import User

class UserProfile(BaseModel):
    user: User
    bio: str
```

## Go

Go doesn't need cross-file imports as long as all types are in the same package.

## Deep Nesting

Imports work with deeply nested types:

```rust
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: u64,
}

pub struct UserList {
    pub data: Pagination<User>,
}
```

Both files get appropriate imports.
