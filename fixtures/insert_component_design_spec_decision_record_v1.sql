-- 1. Insert parent component
INSERT INTO components (type, current_title, latest_version_number)
VALUES (
    'DesignSpec:DecisionRecord',
    'Architecture Decision: Database Migration',
    1
);

-- 2. Insert corresponding version with Serde-compatible JSON string
INSERT INTO component_versions (component_id, version_number, data)
VALUES (
    (SELECT id FROM components WHERE current_title = 'Architecture Decision: Database Migration'),
    1,
    '{
        "group": "DesignSpec",
        "data": {
            "type": "DecisionRecord",
            "data": {
                "decision": "Migrate core ledger to PostgreSQL",
                "rationale": "We require robust native JSONB support and atomic multi-table transactions for workspace history tracking.",
                "alternativesConsidered": [
                    "MySQL (Limited JSON optimization options)",
                    "MongoDB (Lacks relational integrity constraints for transactional history logs)"
                ]
            }
        }
    }'
);