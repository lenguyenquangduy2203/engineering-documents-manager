-- 1. Insert parent component
INSERT INTO components (type, current_title, latest_version_number)
VALUES (
    'Schematic:ImageLink',
    'System Topology Diagram',
    2
);

-- 2. Insert corresponding version with Serde-compatible JSON string
INSERT INTO component_versions (component_id, version_number, data)
VALUES 
    (
        (SELECT id FROM components WHERE current_title = 'System Topology Diagram'),
        1,
        '{
            "group": "Schematic",
            "data": {
                "type": "ImageLink",
                "data": {
                    "path": "/assets/diagrams/system-v1.png"
                }
            }
        }'
    ),
    (
        (SELECT id FROM components WHERE current_title = 'System Topology Diagram'),
        2,
        '{
            "group": "Schematic",
            "data": {
                "type": "ImageLink",
                "data": {
                    "path": "/assets/diagrams/system-v2.png"
                }
            }
        }'
    );