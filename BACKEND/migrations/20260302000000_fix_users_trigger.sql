-- Fix: The original migration referenced a non-existent function
-- 'update_users_updated_at_column' instead of 'update_updated_at_column'

DROP TRIGGER IF EXISTS update_users_updated_at ON users;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
