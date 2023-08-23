ALTER TABLE ONLY "GameLog"
    ADD end_datetime timestamp without time zone;

ALTER TABLE ONLY "GameLog"
    ALTER COLUMN "time" SET NOT NULL;

UPDATE "GameLog"
    SET end_datetime = (datetime + "time");