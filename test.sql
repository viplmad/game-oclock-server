DROP DATABASE "game-col_new";

CREATE DATABASE "game-col_new" ENCODING = 'UTF8';


CREATE TABLE public."DLC" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    base_game_id integer,
    release_year integer,
    cover_filename text,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE TABLE public."DLCAvailable" (
    dlc_id integer NOT NULL,
    user_id integer NOT NULL,
    platform_id integer NOT NULL,
    added_date date NOT NULL
);

CREATE TABLE public."DLCFinish" (
    dlc_id integer NOT NULL,
    user_Id integer NOT NULL,
    date date NOT NULL
);

CREATE SEQUENCE public."DLC_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public."DLC_Id_seq" OWNED BY public."DLC".id;

CREATE TABLE public."Game" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    edition text DEFAULT ''::text NOT NULL,
    release_year integer,
    cover_filename text,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE TABLE public."GameAvailable" (
    game_id integer NOT NULL,
    user_id integer NOT NULL,
    platform_id integer NOT NULL,
    added_date date NOT NULL
);

CREATE TABLE public."GameTag" (
    game_id integer NOT NULL,
    user_id integer NOT NULL,
    tag_id integer NOT NULL
);

CREATE TABLE public."GameFinish" (
    game_id integer NOT NULL,
    user_id integer NOT NULL,
    date date NOT NULL
);

CREATE TABLE public."GameLog" (
    game_id integer NOT NULL,
    user_id integer NOT NULL,
    datetime timestamp without time zone NOT NULL,
    time interval NOT NULL
);

CREATE SEQUENCE public."Game_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public."Game_Id_seq" OWNED BY public."Game".id;

CREATE TABLE public."Platform" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    icon_filename text,
    type smallint DEFAULT 0 NOT NULL,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE public."Platform_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public."Platform_Id_seq" OWNED BY public."Platform".id;

CREATE TABLE public."Tag" (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name text NOT NULL,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE public."Tag_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public."Tag_Id_seq" OWNED BY public."Tag".id;

CREATE TABLE public."GameUserInfo" (
    game_id integer NOT NULL,
    user_id integer NOT NULL,
    status smallint DEFAULT 0 NOT NULL,
    rating integer DEFAULT 0 NOT NULL,
    notes text DEFAULT ''::text NOT NULL,
    save_folder text DEFAULT ''::text NOT NULL,
    screenshot_folder text DEFAULT ''::text NOT NULL,
    backup boolean DEFAULT false NOT NULL
);

CREATE TABLE public."User" (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    added_datetime timestamp without time zone NOT NULL,
    updated_datetime timestamp without time zone NOT NULL
);

CREATE SEQUENCE public."User_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public."User_Id_seq" OWNED BY public."User".id;


ALTER TABLE ONLY public."DLC" ALTER COLUMN id SET DEFAULT nextval('public."DLC_Id_seq"'::regclass);

ALTER TABLE ONLY public."Game" ALTER COLUMN id SET DEFAULT nextval('public."Game_Id_seq"'::regclass);

ALTER TABLE ONLY public."Platform" ALTER COLUMN id SET DEFAULT nextval('public."Platform_Id_seq"'::regclass);

ALTER TABLE ONLY public."Tag" ALTER COLUMN id SET DEFAULT nextval('public."Tag_Id_seq"'::regclass);

ALTER TABLE ONLY public."User" ALTER COLUMN id SET DEFAULT nextval('public."User_Id_seq"'::regclass);


ALTER TABLE ONLY public."DLCFinish"
    ADD CONSTRAINT "DLCFinish_pk" PRIMARY KEY (dlc_id, user_id, date);

ALTER TABLE ONLY public."DLC"
    ADD CONSTRAINT "DLC_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY public."DLC"
    ADD CONSTRAINT "DLC_pk" PRIMARY KEY (id);

ALTER TABLE ONLY public."GameAvailable"
    ADD CONSTRAINT "GameAvailable_pk" PRIMARY KEY (game_id, user_id, platform_id);

ALTER TABLE ONLY public."GameTag"
    ADD CONSTRAINT "GameTag_pk" PRIMARY KEY (game_id, user_id, tag_id);

ALTER TABLE ONLY public."GameFinish"
    ADD CONSTRAINT "GameFinish_pk" PRIMARY KEY (game_id, user_id, date);

ALTER TABLE ONLY public."Game"
    ADD CONSTRAINT "Game_unique" UNIQUE (user_id, name, edition);

ALTER TABLE ONLY public."Game"
    ADD CONSTRAINT "Game_pk" PRIMARY KEY (id);

ALTER TABLE ONLY public."GameLog"
    ADD CONSTRAINT "GameLog_pk" PRIMARY KEY (game_id, user_id, datetime);

ALTER TABLE ONLY public."Platform"
    ADD CONSTRAINT "Platform_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY public."Platform"
    ADD CONSTRAINT "Platform_pk" PRIMARY KEY (id);

ALTER TABLE ONLY public."Tag"
    ADD CONSTRAINT "Tag_unique" UNIQUE (user_id, name);

ALTER TABLE ONLY public."Tag"
    ADD CONSTRAINT "Tag_pk" PRIMARY KEY (id);

ALTER TABLE ONLY public."DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_pk" PRIMARY KEY (dlc_id, user_id, platform_id);

ALTER TABLE ONLY public."GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_pk" PRIMARY KEY (game_id, user_id);

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_unique" UNIQUE (username);

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_pk" PRIMARY KEY (id);


ALTER TABLE ONLY public."DLCFinish"
    ADD CONSTRAINT "DLCFinish_fk0" FOREIGN KEY (dlc_id) REFERENCES public."DLC"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLCFinish"
    ADD CONSTRAINT "DLCFinish_fk1" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLC"
    ADD CONSTRAINT "DLC_fk0" FOREIGN KEY (base_game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLC"
    ADD CONSTRAINT "DLC_fk1" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."Game"
    ADD CONSTRAINT "Game_fk0" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."Platform"
    ADD CONSTRAINT "Platform_fk0" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."Tag"
    ADD CONSTRAINT "Tag_fk0" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk0" FOREIGN KEY (game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk1" FOREIGN KEY (platform_id) REFERENCES public."Platform"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameAvailable"
    ADD CONSTRAINT "GameAvailable_fk2" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameTag"
    ADD CONSTRAINT "GameTag_fk0" FOREIGN KEY (game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameTag"
    ADD CONSTRAINT "GameTag_fk1" FOREIGN KEY (tag_id) REFERENCES public."Tag"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameTag"
    ADD CONSTRAINT "GameTag_fk2" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameFinish"
    ADD CONSTRAINT "GameFinish_fk0" FOREIGN KEY (game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameFinish"
    ADD CONSTRAINT "GameFinish_fk1" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameLog"
    ADD CONSTRAINT "GameLog_fk0" FOREIGN KEY (game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameLog"
    ADD CONSTRAINT "GameLog_fk1" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk0" FOREIGN KEY (dlc_id) REFERENCES public."DLC"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk1" FOREIGN KEY (platform_id) REFERENCES public."Platform"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."DLCAvailable"
    ADD CONSTRAINT "DLCAvailable_fk2" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_fk0" FOREIGN KEY (game_id) REFERENCES public."Game"(id) ON DELETE CASCADE;

ALTER TABLE ONLY public."GameUserInfo"
    ADD CONSTRAINT "GameUserInfo_fk1" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE;