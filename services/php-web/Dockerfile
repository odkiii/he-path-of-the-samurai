FROM php:8.3-fpm

# системные пакеты + pgsql
RUN apt-get update && apt-get install -y \
    bash \
    curl \
    git \
    unzip \
    rsync \
    libpq-dev \
 && docker-php-ext-install pdo pdo_pgsql \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# composer
COPY --from=composer:2 /usr/bin/composer /usr/bin/composer

# каталоги
WORKDIR /var/www/html

COPY . /var/www/html

# права (критично для Laravel)
RUN chown -R www-data:www-data /var/www/html \
 && chmod -R 775 /var/www/html/storage /var/www/html/bootstrap/cache

# entrypoint
COPY entrypoint.sh /usr/local/bin/php_entrypoint.sh
RUN chmod +x /usr/local/bin/php_entrypoint.sh

CMD ["bash", "/usr/local/bin/php_entrypoint.sh"]
