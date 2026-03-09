//! # typewriter Example
//!
//! This example demonstrates how to use `#[derive(TypeWriter)]` to automatically
//! generate TypeScript interfaces and Python Pydantic models from Rust types.
//!
//! Run `cargo build -p typewriter-example` to generate the output files.

use serde::{Deserialize, Serialize};
use typewriter::TypeWriter;

// ─────────────────────────────────────────────────────────────
// 1. Simple Struct — generates interface/BaseModel
// ─────────────────────────────────────────────────────────────

/// A user profile in the system.
#[derive(Debug, Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python)]
pub struct UserProfile {
    /// Unique user identifier
    pub id: String,
    /// User's email address
    pub email: String,
    /// User's display name
    pub display_name: String,
    /// User's age (optional)
    pub age: Option<u32>,
    /// Whether the user's email is verified
    pub is_verified: bool,
    /// List of tags associated with the user
    pub tags: Vec<String>,
}

// ─────────────────────────────────────────────────────────────
// 2. Struct with serde rename — field names transform
// ─────────────────────────────────────────────────────────────

/// An API response wrapper.
#[derive(Debug, Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python)]
pub struct ApiResponse {
    /// HTTP status code
    pub status_code: u32,
    /// Response message
    pub message: String,
    /// Whether the request was successful
    pub success: bool,
    /// Number of items returned
    pub total_count: Option<u64>,
}

// ─────────────────────────────────────────────────────────────
// 3. Simple Enum — generates string union / Python Enum
// ─────────────────────────────────────────────────────────────

/// User roles in the application.
#[derive(Debug, Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python)]
pub enum UserRole {
    Admin,
    Moderator,
    Member,
    Guest,
}

// ─────────────────────────────────────────────────────────────
// 4. Tagged Enum — generates discriminated union
// ─────────────────────────────────────────────────────────────

/// Notification types sent to users.
#[derive(Debug, Serialize, Deserialize, TypeWriter)]
#[serde(tag = "type")]
#[sync_to(typescript, python)]
pub enum Notification {
    /// A simple text message
    Message { title: String, body: String },
    /// A friend request
    FriendRequest { from_user: String },
    /// A system alert
    SystemAlert {
        severity: String,
        message: String,
        code: u32,
    },
}

// ─────────────────────────────────────────────────────────────
// 5. Struct with skipped fields — sensitive data excluded
// ─────────────────────────────────────────────────────────────

/// Internal user record (password_hash excluded from generated types).
#[derive(Debug, Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python)]
pub struct UserRecord {
    pub id: String,
    pub username: String,
    pub email: String,
    /// This field is excluded from generated types
    #[serde(skip)]
    pub password_hash: String,
    /// This field is also excluded
    #[tw(skip)]
    pub internal_notes: String,
}

fn main() {
    println!("✅ typewriter example built successfully!");
    println!("📁 Check ./generated/typescript/ and ./generated/python/ for output files.");
    println!();
    println!("Generated types:");
    println!("  • UserProfile   → user-profile.ts / user_profile.py");
    println!("  • ApiResponse   → api-response.ts / api_response.py");
    println!("  • UserRole      → user-role.ts / user_role.py");
    println!("  • Notification  → notification.ts / notification.py");
    println!("  • UserRecord    → user-record.ts / user_record.py");
}
