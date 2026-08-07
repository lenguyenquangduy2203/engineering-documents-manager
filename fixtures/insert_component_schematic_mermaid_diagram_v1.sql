-- 1. Insert parent component
INSERT INTO components (type, current_title, latest_version_number)
VALUES (
    'Schematic:MermaidDiagram',
    'Auth Flow Sequence Diagram',
    1
);

-- 2. Insert corresponding version with Serde-compatible JSON string
INSERT INTO component_versions (component_id, version_number, data)
VALUES (
    (SELECT id FROM components WHERE current_title = 'Auth Flow Sequence Diagram'),
    1,
    '{
        "group": "Schematic",
        "data": {
            "type": "MermaidDiagram",
            "data": {
                "definition": "sequenceDiagram\n    Client->>API: POST /login\n    API-->>Client: 200 OK (Token)"
            }
        }
    }'
);