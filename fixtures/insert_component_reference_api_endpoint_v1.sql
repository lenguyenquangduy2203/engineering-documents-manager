-- 1. Insert parent component
INSERT INTO components (type, current_title, latest_version_number)
VALUES (
    'Reference:ApiEndpoint',
    'User Registration API Reference',
    1
);

-- 2. Insert corresponding version with Serde-compatible JSON string
INSERT INTO component_versions (component_id, version_number, data)
VALUES (
    (SELECT id FROM components WHERE current_title = 'User Registration API Reference'),
    1,
    '{
        "group": "Reference",
        "data": {
            "type": "ApiEndpoint",
            "data": {
                "endpoint": "/api/v1/users/register",
                "method": "POST",
                "requestBodyExample": "{\n  \"email\": \"dev@example.com\",\n  \"password\": \"Secret123!\"\n}"
            }
        }
    }'
);