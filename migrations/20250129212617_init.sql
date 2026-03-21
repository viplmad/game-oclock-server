SET client_encoding = 'UTF8';

CREATE TABLE IF NOT EXISTS "User" (
	id uuid NOT NULL,
	username text NOT NULL,
	"password" text NOT NULL,
	"role" text NOT NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "User_pk" PRIMARY KEY (id),
	CONSTRAINT "User_unique" UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS "Device" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	"name" text NOT NULL,
	image_url text NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "Device_pk" PRIMARY KEY (id),
	CONSTRAINT "Device_unique" UNIQUE (user_id, name),
	CONSTRAINT "Device_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Location" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	"name" text NOT NULL,
	image_url text NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "Platform_pk" PRIMARY KEY (id),
	CONSTRAINT "Platform_unique" UNIQUE (user_id, name),
	CONSTRAINT "Platform_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Media" (
	id uuid NOT NULL,
	kind text NOT NULL,
	title text NOT NULL,
	edition text NOT NULL,
	release_datetime timestamptz NULL,
	genres _text NOT NULL,
	series _text NOT NULL,
	image_url text NULL,
	parent_id uuid NULL,
	parent_order int4 NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "Media_pk" PRIMARY KEY (id),
	CONSTRAINT "Media_unique" UNIQUE (type, title, edition),
	CONSTRAINT "Media_fk_Parent" FOREIGN KEY (parent_id) REFERENCES public."Media"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "MediaAvailable" (
	user_id uuid NOT NULL,
	media_id uuid NOT NULL,
	location_id uuid NOT NULL,
	"date" timestamptz NOT NULL,
	CONSTRAINT "MediaAvailable_pk" PRIMARY KEY (user_id, media_id, location_id),
	CONSTRAINT "MediaAvailable_fk_Location" FOREIGN KEY (location_id) REFERENCES public."Location"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaAvailable_fk_Media" FOREIGN KEY (media_id) REFERENCES public."Media"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaAvailable_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "MediaSessionGroup" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	"name" text NOT NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	media_id uuid NULL,
	CONSTRAINT "MediaSessionGroup_pk" PRIMARY KEY (id),
	CONSTRAINT "MediaSessionGroup_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE,
	CONSTRAINT mediasessiongroup_fk_media FOREIGN KEY (media_id) REFERENCES public."Media"(id)
);

CREATE TABLE IF NOT EXISTS "MediaUserInfo" (
	user_id uuid NOT NULL,
	media_id uuid NOT NULL,
	status int2 NOT NULL,
	rating int2 NOT NULL,
	notes text NOT NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "MediaUserInfo_pk" PRIMARY KEY (user_id, media_id),
	CONSTRAINT "MediaUserInfo_fk_Media" FOREIGN KEY (media_id) REFERENCES public."Media"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaUserInfo_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "Tag" (
	id uuid NOT NULL,
	user_id uuid NOT NULL,
	"name" text NOT NULL,
	added_datetime timestamptz NOT NULL,
	updated_datetime timestamptz NOT NULL,
	CONSTRAINT "Tag_pk" PRIMARY KEY (id),
	CONSTRAINT "Tag_unique" UNIQUE (user_id, name),
	CONSTRAINT "Tag_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "ExternalMedia" (
	media_id uuid NOT NULL,
	external_source text NOT NULL,
	external_id text NOT NULL,
	external_title text NULL,
	external_release_date text NULL,
	CONSTRAINT "ExternalMedia_pk" PRIMARY KEY (media_id, external_source),
	CONSTRAINT "ExternalMedia_fk_Media" FOREIGN KEY (media_id) REFERENCES public."Media"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "MediaSession" (
	user_id uuid NOT NULL,
	media_id uuid NOT NULL,
	start_datetime timestamptz NOT NULL,
	end_datetime timestamptz NOT NULL,
	group_id uuid NOT NULL,
	device_id uuid NULL,
	started bool NULL,
	finished_status int2 NULL,
	CONSTRAINT "MediaSession_pk" PRIMARY KEY (user_id, media_id, start_datetime),
	CONSTRAINT "MediaSession_fk_Device" FOREIGN KEY (device_id) REFERENCES public."Device"(id) ON DELETE SET NULL,
	CONSTRAINT "MediaSession_fk_Group" FOREIGN KEY (group_id) REFERENCES public."MediaSessionGroup"(id) ON DELETE SET NULL,
	CONSTRAINT "MediaSession_fk_Media" FOREIGN KEY (media_id) REFERENCES public."Media"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaSession_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "MediaTag" (
	user_id uuid NOT NULL,
	media_id uuid NOT NULL,
	tag_id uuid NOT NULL,
	"order" int4 NULL,
	CONSTRAINT "MediaTag_pk" PRIMARY KEY (user_id, media_id, tag_id),
	CONSTRAINT "MediaTag_fk_Media" FOREIGN KEY (media_id) REFERENCES public."Media"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaTag_fk_Tag" FOREIGN KEY (tag_id) REFERENCES public."Tag"(id) ON DELETE CASCADE,
	CONSTRAINT "MediaTag_fk_User" FOREIGN KEY (user_id) REFERENCES public."User"(id) ON DELETE CASCADE
);