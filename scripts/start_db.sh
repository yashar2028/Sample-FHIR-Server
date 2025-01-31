#!/bin/bash

# Define variables for the database
DB_NAME="server_resources"
DB_USER="user1"
DB_PASSWORD="12345678910"
DB_PORT=27017
DB_CONTAINER_NAME="mongo_database"

# Check if the container exists (whether stopped or running)
EXISTING_CONTAINER=$(docker ps -aq -f name=$DB_CONTAINER_NAME)

if [ "$EXISTING_CONTAINER" ]; then
  # If container exists, try to start it
  echo "Starting existing MongoDB container."
  docker start $DB_CONTAINER_NAME
else
  # If container doesn't exist, run a new MongoDB container
  echo "Running a new MongoDB container."
  docker run --name $DB_CONTAINER_NAME \
    -e MONGO_INITDB_ROOT_USERNAME=$DB_USER \
    -e MONGO_INITDB_ROOT_PASSWORD=$DB_PASSWORD \
    -e MONGO_INITDB_DATABASE=$DB_NAME \
    -p $DB_PORT:27017 \
    -d mongo:4.4.29
fi

echo "MongoDB is running on port $DB_PORT."
