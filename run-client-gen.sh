# https://openapi-generator.tech/docs/installation/#jar
java -jar openapi-generator-cli.jar generate -g dart -i public-api.json -o ./gen -c client-gen-conf.yml
