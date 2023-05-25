# https://openapi-generator.tech/docs/installation/#jar
java -jar openapi-generator-cli.jar generate -g dart -i public-api.json -c client-gen-conf.yml

cargo update
cargo fmt
docker build -t n2tmad/game-col:<version> .
docker compose up -d
docker compose down
docker save n2tmad/game-col:<version> -o game-col.tar