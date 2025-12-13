#!/usr/bin/env bash
set -e

APP_DIR="/var/www/html"

echo "[php] container starting"

if [ ! -f "$APP_DIR/artisan" ]; then
  echo "[php] ERROR: Laravel application not found in image"
  exit 1
fi

echo "[php] waiting for database..."
until php -r "
try {
  new PDO(
    'pgsql:host=' . getenv('DB_HOST') . ';port=' . getenv('DB_PORT') . ';dbname=' . getenv('DB_DATABASE'),
    getenv('DB_USERNAME'),
    getenv('DB_PASSWORD')
  );
} catch (Exception \$e) {
  exit(1);
}
"; do
  sleep 2
done

echo "[php] database is ready"

# миграции
php artisan migrate --force || true

# кэш
php artisan config:clear || true
php artisan config:cache || true
php artisan route:cache || true

echo "[php] starting php-fpm"
exec php-fpm -F
