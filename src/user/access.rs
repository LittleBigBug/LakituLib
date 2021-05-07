/*
    Access levels are generic known permission levels that
    are assigned to certain conditions per platform
 */
pub enum AccessLevel {
    EVERYONE,
    SUPPORTER,
    MODERATOR,
    ADMIN,
    ROOT,
}