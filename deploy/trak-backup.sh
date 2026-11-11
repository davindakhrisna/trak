#!/usr/bin/env bash
set -euo pipefail

# Trak Daily SQLite WAL Backup Script
BACKUP_DIR="/var/backups/trak"
DB_FILE="/var/lib/trak/trak.db"
TIMESTAMP="$(date +%F_%H%M%S)"

mkdir -p "${BACKUP_DIR}"

# Safe hot-backup using SQLite backup API without table locks
sqlite3 "${DB_FILE}" ".backup '${BACKUP_DIR}/trak-${TIMESTAMP}.db'"

# Retain backups for 14 days
find "${BACKUP_DIR}" -name "trak-*.db" -mtime +14 -delete
