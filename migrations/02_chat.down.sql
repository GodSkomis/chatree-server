

DROP INDEX IF EXISTS ix_chat_message_last_updated;

DROP TRIGGER IF EXISTS trigger_update_user_updated_at ON chat_messages;

DROP TABLE IF EXISTS chat_users;

DROP TABLE IF EXISTS chat_messages;

DROP TABLE IF EXISTS chats;