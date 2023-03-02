CREATE TABLE IF NOT EXISTS "DLC" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    base_game_id integer,
    release_year integer,
    cover_filename text,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE TABLE IF NOT EXISTS "DLCAvailable" (
    user_id integer NOT NULL,
    dlc_id integer NOT NULL,
    platform_id integer NOT NULL,
    added_date date NOT NULL
);

CREATE TABLE IF NOT EXISTS "DLCFinish" (
    user_Id integer NOT NULL,
    dlc_id integer NOT NULL,
    date date NOT NULL
);

CREATE SEQUENCE IF NOT EXISTS "DLC_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE "DLC_Id_seq" OWNED BY "DLC".id;

CREATE TABLE IF NOT EXISTS "Game" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    edition text DEFAULT ''::text NOT NULL,
    release_year integer,
    cover_filename text,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE TABLE IF NOT EXISTS "GameAvailable" (
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    platform_id integer NOT NULL,
    added_date date NOT NULL
);

CREATE TABLE IF NOT EXISTS "GameTag" (
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    tag_id integer NOT NULL
);

CREATE TABLE IF NOT EXISTS "GameFinish" (
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    date date NOT NULL
);

CREATE TABLE IF NOT EXISTS "GameLog" (
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    datetime timestamp without time zone NOT NULL,
    time interval NOT NULL
);

CREATE SEQUENCE IF NOT EXISTS "Game_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE "Game_Id_seq" OWNED BY "Game".id;

CREATE TABLE IF NOT EXISTS "Platform" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    icon_filename text,
    type smallint,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE IF NOT EXISTS "Platform_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE "Platform_Id_seq" OWNED BY "Platform".id;

CREATE TABLE IF NOT EXISTS "Tag" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE IF NOT EXISTS "Tag_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE "Tag_Id_seq" OWNED BY "Tag".id;

CREATE TABLE IF NOT EXISTS "GameUserInfo" (
    user_id integer NOT NULL,
    game_id integer NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    rating integer DEFAULT 0 NOT NULL,
    notes text DEFAULT ''::text NOT NULL,
    save_folder text DEFAULT ''::text NOT NULL,
    screenshot_folder text DEFAULT ''::text NOT NULL,
    backup boolean DEFAULT false NOT NULL
);

CREATE TABLE IF NOT EXISTS "User" (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    admin boolean DEFAULT false NOT NULL,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE IF NOT EXISTS "User_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE "User_Id_seq" OWNED BY "User".id;


ALTER TABLE ONLY "DLC" ALTER COLUMN id SET DEFAULT nextval("DLC_Id_seq"::regclass);

ALTER TABLE ONLY "Game" ALTER COLUMN id SET DEFAULT nextval("Game_Id_seq"::regclass);

ALTER TABLE ONLY "Platform" ALTER COLUMN id SET DEFAULT nextval("Platform_Id_seq"::regclass);

ALTER TABLE ONLY "Tag" ALTER COLUMN id SET DEFAULT nextval("Tag_Id_seq"::regclass);

ALTER TABLE ONLY "User" ALTER COLUMN id SET DEFAULT nextval("User_Id_seq"::regclass);


ALTER TABLE ONLY "DLCFinish"
    ADD CONSTRAINT "DLCFinish_pk" PRIMARY KEY (user_id, dlc_id, date);

ALTER TABLE ONLY "DLC"
    ADD CONSTRAINT "DLC_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY "DLC"
    ADD CONSTRAINT "DLC_pk" PRIMARY KEY (id);

ALTER TABLE ONLY "GameAvailable"
    ADD CONSTRAINT "GameAvailable_pk" PRIMARY KEY (user_id, game_id, platform_id);

ALTER TABLE ONLY "GameTag"
    ADD CONSTRAINT "GameTag_pk" PRIMARY KEY (user_id, game_id, tag_id);

ALTER TABLE ONLY "GameFinish"
    ADD CONSTRAINT "GameFinish_pk" PRIMARY KEY (user_id, game_id, date);

ALTER TABLE ONLY "Game"
    ADD CONSTRAINT "Game_unique" UNIQUE (user_id, name, edition);

ALTER TABLE ONLY "Game"
    ADD CONSTRAINT "Game_pk" PRIMARY KEY (id);

ALTER TABLE ONLY "GameLog"
    ADD CONSTRAINT "GameLog_pk" PRIMARY KEY (user_id, game_id, datetime);

ALTER TABLE ONLY "Platform"
    ADD CONSTRAINT "Platform_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY "Platform"
    ADD CONSTRAINT "Platform_pk" PRIMARY KEY (id);

ALTER TABLE ONLY "Tag"
    ADD CONSTRAINT "Tag_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY "Tag"
    ADD CONSTRAINT "Tag_pk" PRIMARY KEY (id);

ALTER TABLE ONLY "DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_pk" PRIMARY KEY (user_id, dlc_id, platform_id);

ALTER TABLE ONLY "GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_pk" PRIMARY KEY (user_id, game_id);

ALTER TABLE ONLY "User"
    ADD CONSTRAINT "User_unique" UNIQUE (username);

ALTER TABLE ONLY "User"
    ADD CONSTRAINT "User_pk" PRIMARY KEY (id);


ALTER TABLE ONLY "DLCFinish"
    ADD CONSTRAINT "DLCFinish_fk0" FOREIGN KEY (dlc_id) REFERENCES "DLC"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLCFinish"
    ADD CONSTRAINT "DLCFinish_fk1" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLC"
    ADD CONSTRAINT "DLC_fk0" FOREIGN KEY (base_game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLC"
    ADD CONSTRAINT "DLC_fk1" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "Game"
    ADD CONSTRAINT "Game_fk0" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "Platform"
    ADD CONSTRAINT "Platform_fk0" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "Tag"
    ADD CONSTRAINT "Tag_fk0" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk0" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk1" FOREIGN KEY (platform_id) REFERENCES "Platform"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk2" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameTag"
    ADD CONSTRAINT "GameTag_fk0" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameTag"
    ADD CONSTRAINT "GameTag_fk1" FOREIGN KEY (tag_id) REFERENCES "Tag"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameTag"
    ADD CONSTRAINT "GameTag_fk2" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameFinish"
    ADD CONSTRAINT "GameFinish_fk0" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameFinish"
    ADD CONSTRAINT "GameFinish_fk1" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameLog"
    ADD CONSTRAINT "GameLog_fk0" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameLog"
    ADD CONSTRAINT "GameLog_fk1" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk0" FOREIGN KEY (dlc_id) REFERENCES "DLC"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk1" FOREIGN KEY (platform_id) REFERENCES "Platform"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk2" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_fk0" FOREIGN KEY (game_id) REFERENCES "Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY "GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_fk1" FOREIGN KEY (user_id) REFERENCES "User"(id) ON DELETE CASCADE;


INSERT INTO "User" (username, password, admin, added_datetime, updated_datetime)
VALUES ('admin', '$2a$12$4PiH98/.9OiLpkXPIdqwu.rwzkjacoAZt1UCGwY8bujuYxLSR.wG6', TRUE, NOW(), NOW());
