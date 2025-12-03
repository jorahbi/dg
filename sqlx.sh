#!/bin/bash
set -e
echo "Step 1: Cleaning cache..."
cargo clean

echo "Step 2: Reinstalling sqlx-cli for MySQL..."
cargo install sqlx-cli --no-default-features --features mysql,rustls --force

echo "Step 3: Testing MySQL connection..."
mysql -h 127.0.0.1 -P 3306 -u root -e "USE coin_dgai; SELECT 1;" || echo "Connection failed! Check MySQL service."

echo "Step 4: Full prepare (may take 10-30s)..."
cargo sqlx prepare --database-url "mysql://root:@127.0.0.1:3306/coin_dgai"

echo "Step 5: Check..."
cargo sqlx prepare --check

echo "Done! .sqlx/ generated."
