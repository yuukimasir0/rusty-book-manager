INSERT INTO 
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO 
    users (name, email, password_hash, role_id)
SELECT
    'Eleazae Fig',
    'eleazar.fig@example.com',
    '$2b$12$UmB6n1GYOETgyfEcCkUW1OQjfrsMtUV0Q8hxPDV3RmHL/76bT2szC',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';