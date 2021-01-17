# Addons

## Install

### OpenAPI Generator
    docker pull openapitools/openapi-generator-cli

    docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://raw.githubusercontent.com/openapitools/openapi-generator/master/modules/openapi-generator/src/test/resources/3_0/petstore.yaml \
    -g elm \
    -o /local/out/elm

    docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://raw.githubusercontent.com/openapitools/openapi-generator/master/modules/openapi-generator/src/test/resources/3_0/petstore.yaml \
    -g rust \
    -o /local/server-models \
    --global-property models

    docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -h

    docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli \
        config-help \
        -g rust-server 
        


### Install PostgreSQL

    docker run --name warp -e POSTGRES_PASSWORD=password -d postgres

    docker run -d --name dev-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres

### Copy .env.template into .env and fill

    cp .env.template .env

### Inside server

    movine init
    movine migrate