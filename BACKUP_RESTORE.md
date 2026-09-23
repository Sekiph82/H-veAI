# H!veAI Backup and Restore

## Backup

Use Settings and the backup action to select a destination and create a verified backup. The backup contains supported H!veAI SQLite application state, schema/version metadata, and integrity information. It does not contain registered project repositories, arbitrary source files, provider credentials, API keys, or external GitHub data beyond bounded application records retained by the schema.

## Restore safety

Select a backup produced by a compatible H!veAI schema. The restore path checks file integrity and schema compatibility before applying it, requires explicit confirmation, and creates a safety backup of the current state. Restart H!veAI after a successful restore and keep the safety backup until the registry, Command Center, sessions, and Settings are verified.

## Failure handling

Missing paths, permission failures, checksum mismatch, malformed data, incompatible schema, and interrupted restore are reported as failures. Do not retry by deleting application data. Preserve the original database and safety backup, collect redacted logs, and contact the maintainer with the exact failure class.
