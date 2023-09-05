DROP TABLE IF EXISTS DebugRequestMode CASCADE;
DROP TABLE IF EXISTS DebugRequestMessageId CASCADE;
DROP TABLE IF EXISTS DebugRequest CASCADE;
DROP TABLE IF EXISTS ComponentStatus CASCADE;

CREATE UNLOGGED TABLE DebugRequestMode (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO DebugRequestMode (id, name) VALUES
    (0, 'RetrieveOnce'), 
    (1, 'Subscribe'), 
    (2, 'Unsubscribe');

CREATE UNLOGGED TABLE DebugRequestMessageId (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO DebugRequestMessageId (id, name) VALUES
    (0, 'ComponentStatus'), 
    (1, 'All');

CREATE UNLOGGED TABLE DebugRequest (
    id SERIAL PRIMARY KEY,
    mode INTEGER REFERENCES DebugRequestMode(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    message_id INTEGER REFERENCES DebugRequestMessageId(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL
);
ALTER SEQUENCE DebugRequest_id_seq CYCLE;

CREATE RULE SilentlyPreventAnyDeleteOnDebugRequestMode AS ON DELETE TO DebugRequestMode
    DO INSTEAD NOTHING;

CREATE RULE SilentlyPreventAnyDeleteOnDebugRequestMessageId AS ON DELETE TO DebugRequestMessageId
    DO INSTEAD NOTHING;

CREATE INDEX DebugRequest_Index_mode ON DebugRequest(mode);

CREATE INDEX DebugRequest_Index_message_id ON DebugRequest(message_id);

CREATE OR REPLACE FUNCTION DelChilds_DebugRequest() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM DebugRequestMode WHERE id = OLD.mode;
    DELETE FROM DebugRequestMessageId WHERE id = OLD.message_id;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_DebugRequest AFTER DELETE ON DebugRequest
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_DebugRequest();

CREATE UNLOGGED TABLE ComponentStatus (
    id SERIAL PRIMARY KEY,
    unix_timestamp_millis BIGINT NOT NULL,
    alive_sensors BYTEA NOT NULL,
    alive_server_modules BYTEA NOT NULL,
    alive_autonomous_vehicles BYTEA NOT NULL,
    alive_connected_vehicles BYTEA NOT NULL,
    alive_nomadic_devices BYTEA NOT NULL,
    other BYTEA NOT NULL
);
ALTER SEQUENCE ComponentStatus_id_seq CYCLE;
