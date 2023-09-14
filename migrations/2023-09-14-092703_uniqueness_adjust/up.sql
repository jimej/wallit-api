DROP INDEX logins@logins_cname_key CASCADE;-- ALTER TABLE logins ADD CONSTRAINT logins_cname_key UNIQUE(cname) didn't work - this may be cockroach specific
DROP INDEX logins@logins_email_key CASCADE;-- ALTER TABLE logins ADD CONSTRAINT logins_email_key UNIQUE(email);
ALTER TABLE groups ADD CONSTRAINT uniq_user_group UNIQUE(user_id, group_name); 

-- ALTER TABLE logins DROP CONSTRAINT logins_cname_key; -- cannot drop UNIQUE constraint "logins_email_key" using ALTER TABLE DROP CONSTRAINT, use DROP INDEX CASCADE instead
-- ALTER TABLE logins DROP CONSTRAINT logins_email_key;