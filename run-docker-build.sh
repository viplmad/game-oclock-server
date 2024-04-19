sudo docker build -t viplmad/game-oclock-server:<version> .
docker compose up -d
docker compose down
docker save viplmad/game-oclock-server:<version> -o game-oclock-server.tar
