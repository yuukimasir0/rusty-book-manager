INSERT INTO 
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO 
    users (name, email, password_hash, role_id)
SELECT
    'admin',
    'admin@example.com',
    '$2b$12$rgPlfh8L1WnMP1eRvf0MruHCmTiz6GceIN6b31u6N8Zmm9bypqIsS',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';