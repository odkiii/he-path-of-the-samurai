FROM php:8.3-fpm

# пакеты и расширения
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
RUN mkdir -p /var/www/html /opt/laravel-patches
WORKDIR /var/www/html

# entrypoint
COPY entrypoint.sh /usr/local/bin/php_entrypoint.sh
RUN chmod +x /usr/local/bin/php_entrypoint.sh

CMD ["bash", "/usr/local/bin/php_entrypoint.sh"]

