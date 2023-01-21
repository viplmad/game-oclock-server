# https://openapi-generator.tech/docs/installation/#jar
java -jar openapi-generator-cli.jar generate -g dart -i public-api.json -c client-gen-conf.yml

docker build -t n2tmad/game-col:0.1.0 .
docker compose up -d
docker compose down
docker save n2tmad/game-col:0.1.0 -o game-col.tar