-- 1. Insert parent components
INSERT INTO components (type, current_title, latest_version_number)
VALUES 
    ('Schematic:MermaidDiagram', 'System Architecture Diagram', 1),
    ('Schematic:ImageLink', 'Network Topology Image', 2),
    ('Reference:ApiEndpoint', 'User Registration Endpoint', 1);

-- 2. Insert corresponding version with Serde-compatible JSON string
INSERT INTO component_versions (component_id, version_number, data)
VALUES
    (
        (SELECT id FROM components WHERE current_title = 'System Architecture Diagram'),
        1,
        '{
            "group": "Schematic",
            "data": {
                "type": "MermaidDiagram",
                "data": {
                    "definition": "graph TD;\n  Client-->API;\n  API-->Database;"
                }
            }
        }'
    ),
    (
        (SELECT id FROM components WHERE current_title = 'Network Topology Image'),
        1,
        '{
            "group": "Schematic",
            "data": {
                "type": "ImageLink",
                "data": {
                    "path": "/assets/network_v1.png"
                }
            }
        }'
    ),
    (
        (SELECT id FROM components WHERE current_title = 'Network Topology Image'),
        2,
        '{
            "group": "Schematic",
            "data": {
                "type": "ImageLink",
                "data": {
                    "path": "/assets/network_v2.png"
                }
            }
        }'
    ),
    (
        (SELECT id FROM components WHERE current_title = 'User Registration Endpoint'),
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

-- 3. Insert document metadata
INSERT INTO documents (type, title, status)
VALUES ('API', 'User Service API Specification', 'DRAFT');

-- 4. Link the document layouts
INSERT INTO document_layouts (document_id, component_version_id, position)
VALUES
    (
        (SELECT id FROM documents WHERE title = 'User Service API Specification'),
        (SELECT id FROM component_versions 
            WHERE component_id = (SELECT id FROM components WHERE current_title = 'System Architecture Diagram') 
            AND version_number = 1
        ),
        0
    ),
    (
        (SELECT id FROM documents WHERE title = 'User Service API Specification'),
        (SELECT id FROM component_versions 
            WHERE component_id = (SELECT id FROM components WHERE current_title = 'Network Topology Image') 
            AND version_number = 2
        ),
        1
    ),
    (
        (SELECT id FROM documents WHERE title = 'User Service API Specification'),
        (SELECT id FROM component_versions 
            WHERE component_id = (SELECT id FROM components WHERE current_title = 'User Registration Endpoint') 
            AND version_number = 1
        ),
        2
    );