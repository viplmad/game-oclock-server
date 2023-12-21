docker build -t n2tmad/game-col:<version> .
docker compose up -d
docker compose down
docker save n2tmad/game-col:<version> -o game-col.tar
