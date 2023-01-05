#!/bin/bash

echo "Now in entrypoint.sh for Game collection"
echo "Script:       0.0.1"
echo "User:         '$(whoami)'"
echo "Group:        '$(id -g -n)'"
echo "Working dir:  '$(pwd)'"
echo "Build number:  $(cat /var/www/counter-main.txt)"
echo "Build date:    $(cat /var/www/build-date-main.txt)"

# https://github.com/docker-library/wordpress/blob/master/docker-entrypoint.sh
echo "Wait for the database."
if [[ -z "$DB_PORT" ]]; then
  DB_PORT=5432
fi
#if [[ -n "$DB_PORT" ]]; then
#  /usr/local/bin/wait-for-it.sh "${DB_HOST}:${DB_PORT}" -t 60 -- echo "DB is up."
#fi

echo "Wait another 5 seconds in case the DB needs to boot."
sleep 5
echo "Done waiting for the DB to boot."

echo "Init database, if needed"

psql postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_DATABASE} -a -f /sql/init.sql

if [[ $DKR_RUN_MIGRATION == "false" ]]; then
  echo "Will NOT run migration commands."
else
  echo "Running migration commands..."
fi

if [[ $DKR_RUN_UPGRADE == "false" ]]; then
  echo 'Will NOT run upgrade commands.'
else
  echo 'Running upgrade commands...'
fi

# set docker var.
export IS_DOCKER=true
