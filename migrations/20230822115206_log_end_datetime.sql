ALTER TABLE ONLY "GameLog"
    ADD end_datetime timestamp without time zone;

UPDATE "GameLog"
    SET end_datetime = (datetime + "time");