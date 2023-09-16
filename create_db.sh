#!/bin/bash
set -ex

NAME=eigen-prover-statedb
PORT=5432

function create() {
    if [ $# -ne 3 ] && [ $# -ne 4 ]; then
        echo "usage: create_db <database> <user> <password> [<file.sql>]"
        exit
    fi
    db=$1
    user=$2
    pswd=$3
    echo "StateDB database creation"
    echo "Installing PostgreSQL..."
    docker run -d --name $NAME -p $PORT:5432 -e POSTGRES_PASSWORD=$pswd -e POSTGRES_USER=$user -e POSTGRES_DB=$db postgres
    sleep 2
    echo "Creating table state.merkletree..."
    docker exec -ti $NAME psql -U $user -d $db -c 'create schema state;'
    docker exec -ti $NAME psql -U $user -d $db -c 'create table state.nodes (hash bytea primary key, data bytea not null);'
    docker exec -ti $NAME psql -U $user -d $db -c 'create table state.program (hash bytea primary key, data bytea not null);'
    if [ $# == 4 ]; then
        docker exec -ti $NAME psql -U $user -d $db -f $4
    fi

    echo "Done."
    echo "Example of connection string to use in the config.json file:"
    echo "  \"databaseURL\": \"postgresql://$user:$pswd@127.0.0.1:${PORT}/$db\""
    echo
}

function delete() {
    if [ $# -ne 2 ]
        then
            echo "usage: delete_db <database> <user>"
            exit
    fi
    echo "StateDB database deletion"
    echo "Deleting database $1..."
    docker rm -f $NAME
    echo "Done."
    echo
}

action=$1
shift
$action $@
