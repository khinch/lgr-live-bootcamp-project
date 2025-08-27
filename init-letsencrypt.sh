#!/bin/env bash

# nginx won't start without certs, but letsencrypt needs nginx up to complete the challenge.
# If no certs exist then create dummy ones to get nginx started. Once it's up, delete the
# dummy and invoke letsencrypt to get real ones, then reload nginx to pick up the certs.

domain="lgr.testwebsitepleaseignore.uk"
email="admin@testwebsitepleaseignore.uk"
staging=0 # Set to 1 to test with staging environment

# Exit if certificates already exist
if [ -f "./certbot/conf/live/$domain/fullchain.pem" ]; then
    echo "### Certificate for $domain already exists. Skipping certificate generation."
    exit 0
fi

echo "### Creating dummy certificate for $domain ..."
mkdir -p "./certbot/conf/live/$domain"
docker compose run --rm --entrypoint "\
    openssl req -x509 -nodes -newkey rsa:2048 -days 1 \
    -keyout '/etc/letsencrypt/live/$domain/privkey.pem' \
    -out '/etc/letsencrypt/live/$domain/fullchain.pem' \
    -subj '/CN=localhost'" certbot

echo "### Starting nginx ..."
docker compose up --force-recreate -d nginx

echo "### Deleting dummy certificate for $domain ..."
docker compose run --rm --entrypoint "\
    rm -Rf /etc/letsencrypt/live/$domain* && \
    rm -Rf /etc/letsencrypt/archive/$domain* && \
    rm -Rf /etc/letsencrypt/renewal/$domain*.conf" certbot

echo "### Requesting Let's Encrypt certificate for $domain ..."

case "$email" in
  "") email_arg="--register-unsafely-without-email" ;;
  *) email_arg="--email $email" ;;
esac

if [ $staging != "0" ]; then staging_arg="--staging"; fi

docker compose run --rm --entrypoint "\
    certbot certonly --webroot -w /var/www/certbot \
    $staging_arg \
    $email_arg \
    -d $domain \
    --rsa-key-size 4096 \
    --agree-tos \
    --force-renewal" certbot

echo "### Reloading nginx ..."
docker compose exec nginx nginx -s reload
