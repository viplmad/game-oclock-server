SET client_encoding = 'UTF8';

CREATE TABLE IF NOT EXISTS "Device" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	name text NOT NULL,
	icon_url text NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "Device_pk" PRIMARY KEY (id),
	CONSTRAINT "Device_unique" UNIQUE (user_id, name)
);

CREATE TABLE "User" (
	id uuid NOT NULL,
	username text NOT NULL,
	password text NOT NULL,
	admin bool NOT NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "User_pk" PRIMARY KEY (id),
	CONSTRAINT "User_unique" UNIQUE (username)
);

CREATE TABLE "Game" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	title text NOT NULL,
	edition text NOT NULL,
	release_date date NULL,
	base_game_id uuid NULL,
	cover_url text NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "Game_pk" PRIMARY KEY (id),
	CONSTRAINT "Game_unique" UNIQUE (user_id, title, edition),
	CONSTRAINT "Game_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "Game_fk_BaseGame" FOREIGN KEY (base_game_id) REFERENCES "Game"(id) ON DELETE CASCADE
);

CREATE TABLE "GameFinish" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	datetime timestamp with time zone NOT NULL,
	status smallint NOT NULL,
	device_id uuid NULL,
	CONSTRAINT "GameFinish_pk" PRIMARY KEY (user_id, game_id, datetime),
	CONSTRAINT "GameFinish_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameFinish_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE,
	CONSTRAINT "GameFinish_fk_Device" FOREIGN KEY (device_id) REFERENCES "Device"(id) ON DELETE CASCADE
);

CREATE TABLE "GameLink" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	url text NOT NULL,
	description text NOT NULL,
	CONSTRAINT "GameLink_pk" PRIMARY KEY (user_id, game_id, url),
	CONSTRAINT "GameLink_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameLink_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE
);

CREATE TABLE "GameLog" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	start_datetime timestamp with time zone NOT NULL,
	end_datetime timestamp with time zone NOT NULL,
	device_id uuid NULL,
	CONSTRAINT "GameLog_pk" PRIMARY KEY (user_id, game_id, start_datetime),
	CONSTRAINT "GameLog_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameLog_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE,
	CONSTRAINT "GameLog_fk_Device" FOREIGN KEY (device_id) REFERENCES "Device"(id) ON DELETE CASCADE
);

CREATE TABLE "GameUserInfo" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	status smallint NOT NULL,
	rating smallint NOT NULL,
	notes text NOT NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "GameUserInfo_pk" PRIMARY KEY (user_id, game_id),
	CONSTRAINT "GameUserInfo_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameUserInfo_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE
);

CREATE TABLE "Genre" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	name text NOT NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "Genre_pk" PRIMARY KEY (id),
	CONSTRAINT "Genre_unique" UNIQUE (user_id, name),
	CONSTRAINT "Genre_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE
);

CREATE TABLE "Location" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	name text NOT NULL,
	icon_url text NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "Platform_pk" PRIMARY KEY (id),
	CONSTRAINT "Platform_unique" UNIQUE (user_id, name),
	CONSTRAINT "Platform_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE
);

CREATE TABLE "Tag" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	name text NOT NULL,
	added_datetime timestamp with time zone NOT NULL,
	updated_datetime timestamp with time zone NOT NULL,
	CONSTRAINT "Tag_pk" PRIMARY KEY (id),
	CONSTRAINT "Tag_unique" UNIQUE (user_id, name),
	CONSTRAINT "Tag_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE
);

CREATE TABLE "GameAvailable" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	location_id uuid NOT NULL,
	date date NOT NULL,
	CONSTRAINT "GameAvailable_pk" PRIMARY KEY (user_id, game_id, location_id),
	CONSTRAINT "GameAvailable_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameAvailable_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE,
	CONSTRAINT "GameAvailable_fk_Location" FOREIGN KEY (location_id) REFERENCES "Location"(id) ON DELETE CASCADE
);

CREATE TABLE "GameGenre" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	genre_id uuid NOT NULL,
	CONSTRAINT "GameGenre_pk" PRIMARY KEY (user_id, game_id, genre_id),
	CONSTRAINT "GameGenre_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameGenre_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE,
	CONSTRAINT "GameGenre_fk_Genre" FOREIGN KEY (genre_id) REFERENCES "Genre"(id) ON DELETE CASCADE
);

CREATE TABLE "GameTag" (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	tag_id uuid NOT NULL,
	CONSTRAINT "GameTag_pk" PRIMARY KEY (user_id, game_id, tag_id),
	CONSTRAINT "GameTag_fk_User" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE,
	CONSTRAINT "GameTag_fk_Game" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE,
	CONSTRAINT "GameTag_fk_Tag" FOREIGN KEY (tag_id) REFERENCES "Tag"(id) ON DELETE CASCADE
);