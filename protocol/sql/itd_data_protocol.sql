DROP TABLE IF EXISTS Identity CASCADE;
DROP TABLE IF EXISTS Action CASCADE;
DROP TABLE IF EXISTS Version CASCADE;
DROP TABLE IF EXISTS BuildInfo CASCADE;
DROP TABLE IF EXISTS MessageMode CASCADE;
DROP TABLE IF EXISTS TrackingEvent CASCADE;
DROP TABLE IF EXISTS TrackingInformation CASCADE;
DROP TABLE IF EXISTS ApplicationInfo CASCADE;
DROP TABLE IF EXISTS DataMessage CASCADE;
DROP TABLE IF EXISTS TrackingInformation_Events CASCADE;
DROP TABLE IF EXISTS DataSupplement CASCADE;
DROP TABLE IF EXISTS MalformedMessage CASCADE;
DROP TABLE IF EXISTS ApplicationInfo_Args CASCADE;
DROP TABLE IF EXISTS MalformedContent CASCADE;

CREATE UNLOGGED TABLE Identity (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO Identity (id, name) VALUES
    (0, 'Client'), 
    (1, 'Server'), 
    (2, 'DataServer'), 
    (3, 'Persistence'), 
    (4, 'FusionAlgo'), 
    (5, 'RoadClearanceModule'), 
    (6, 'FusionAlgoTimeoutDetector'), 
    (7, 'EdgeApplication'), 
    (8, 'Vehicle'), 
    (9, 'NomadicDevice'), 
    (10, 'PlanningModule'), 
    (11, 'WarningModule'), 
    (12, 'Reserved');

CREATE UNLOGGED TABLE Action (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO Action (id, name) VALUES
    (0, 'Received'), 
    (1, 'Sent'), 
    (2, 'Connect'), 
    (3, 'Disconnect'), 
    (4, 'DecodingFailed'), 
    (5, 'Created'), 
    (6, 'DecodingSucceeded'), 
    (7, 'EncodingFailed'), 
    (8, 'EncodingSucceeded');

CREATE UNLOGGED TABLE Version (
    id SERIAL PRIMARY KEY,
    major SMALLINT NOT NULL,
    minor SMALLINT NOT NULL,
    patch SMALLINT NOT NULL
);
ALTER SEQUENCE Version_id_seq CYCLE;

CREATE UNLOGGED TABLE BuildInfo (
    id SERIAL PRIMARY KEY,
    git_hash TEXT NOT NULL,
    ci_platform TEXT NOT NULL,
    profile TEXT NOT NULL,
    time TEXT NOT NULL,
    compiler TEXT NOT NULL
);
ALTER SEQUENCE BuildInfo_id_seq CYCLE;

CREATE UNLOGGED TABLE MessageMode (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO MessageMode (id, name) VALUES
    (0, 'Received'), 
    (1, 'Sent'), 
    (2, 'Connect'), 
    (3, 'Disconnect');

CREATE UNLOGGED TABLE TrackingEvent (
    id SERIAL PRIMARY KEY,
    timestamp BIGINT NOT NULL,
    submilli_nanos INTEGER NOT NULL,
    session_id BIGINT NOT NULL,
    session_ip TEXT NOT NULL,
    session_identity INTEGER REFERENCES Identity(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    action INTEGER REFERENCES Action(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL
);
ALTER SEQUENCE TrackingEvent_id_seq CYCLE;

CREATE UNLOGGED TABLE TrackingInformation (
    id SERIAL PRIMARY KEY,
    timestamp BIGINT NOT NULL,
    protocol INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    message_body BYTEA NOT NULL
);
ALTER SEQUENCE TrackingInformation_id_seq CYCLE;

CREATE UNLOGGED TABLE ApplicationInfo (
    id SERIAL PRIMARY KEY,
    identity INTEGER REFERENCES Identity(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    version INTEGER REFERENCES Version(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    build INTEGER REFERENCES BuildInfo(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    additional TEXT NOT NULL
);
ALTER SEQUENCE ApplicationInfo_id_seq CYCLE;

CREATE UNLOGGED TABLE DataMessage (
    id SERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL,
    session_ip TEXT NOT NULL,
    session_identity INTEGER REFERENCES Identity(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    msg_mode INTEGER REFERENCES MessageMode(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    timestamp BIGINT NOT NULL,
    encoded_data BYTEA
);
ALTER SEQUENCE DataMessage_id_seq CYCLE;

CREATE UNLOGGED TABLE TrackingInformation_Events (
    list INTEGER REFERENCES TrackingInformation(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    value INTEGER REFERENCES TrackingEvent(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    PRIMARY KEY(list, value)
);

CREATE UNLOGGED TABLE DataSupplement (
    id SERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL,
    session_ip TEXT NOT NULL,
    session_identity INTEGER REFERENCES Identity(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    msg_mode INTEGER REFERENCES MessageMode(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    timestamp BIGINT NOT NULL,
    encoded_data_length SMALLINT NOT NULL
);
ALTER SEQUENCE DataSupplement_id_seq CYCLE;

CREATE UNLOGGED TABLE MalformedMessage (
    id SERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL,
    session_ip TEXT NOT NULL,
    session_identity INTEGER REFERENCES Identity(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    msg_mode INTEGER REFERENCES MessageMode(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    timestamp BIGINT NOT NULL,
    encoded_data BYTEA NOT NULL
);
ALTER SEQUENCE MalformedMessage_id_seq CYCLE;

CREATE UNLOGGED TABLE ApplicationInfo_Args (
    list INTEGER REFERENCES ApplicationInfo(id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY(list, value)
);

CREATE RULE SilentlyPreventAnyDeleteOnIdentity AS ON DELETE TO Identity
    DO INSTEAD NOTHING;

CREATE RULE SilentlyPreventAnyDeleteOnMessageMode AS ON DELETE TO MessageMode
    DO INSTEAD NOTHING;

CREATE RULE SilentlyPreventAnyDeleteOnAction AS ON DELETE TO Action
    DO INSTEAD NOTHING;

CREATE INDEX TrackingEvent_Index_session_identity ON TrackingEvent(session_identity);

CREATE INDEX TrackingEvent_Index_action ON TrackingEvent(action);

CREATE OR REPLACE FUNCTION DelChilds_TrackingEvent() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM Identity WHERE id = OLD.session_identity;
    DELETE FROM Action WHERE id = OLD.action;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_TrackingEvent AFTER DELETE ON TrackingEvent
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_TrackingEvent();

CREATE INDEX DataMessage_Index_session_identity ON DataMessage(session_identity);

CREATE INDEX DataMessage_Index_msg_mode ON DataMessage(msg_mode);

CREATE OR REPLACE FUNCTION DelChilds_DataMessage() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM Identity WHERE id = OLD.session_identity;
    DELETE FROM MessageMode WHERE id = OLD.msg_mode;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_DataMessage AFTER DELETE ON DataMessage
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_DataMessage();

CREATE INDEX TrackingInformation_Events_Index_list ON TrackingInformation_Events(list);

CREATE INDEX TrackingInformation_Events_Index_value ON TrackingInformation_Events(value);

CREATE OR REPLACE FUNCTION DelChilds_TrackingInformation_Events() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM TrackingEvent WHERE id = OLD.value;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_TrackingInformation_Events AFTER DELETE ON TrackingInformation_Events
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_TrackingInformation_Events();

CREATE INDEX DataSupplement_Index_session_identity ON DataSupplement(session_identity);

CREATE INDEX DataSupplement_Index_msg_mode ON DataSupplement(msg_mode);

CREATE OR REPLACE FUNCTION DelChilds_DataSupplement() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM Identity WHERE id = OLD.session_identity;
    DELETE FROM MessageMode WHERE id = OLD.msg_mode;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_DataSupplement AFTER DELETE ON DataSupplement
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_DataSupplement();

CREATE INDEX MalformedMessage_Index_session_identity ON MalformedMessage(session_identity);

CREATE INDEX MalformedMessage_Index_msg_mode ON MalformedMessage(msg_mode);

CREATE OR REPLACE FUNCTION DelChilds_MalformedMessage() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM Identity WHERE id = OLD.session_identity;
    DELETE FROM MessageMode WHERE id = OLD.msg_mode;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_MalformedMessage AFTER DELETE ON MalformedMessage
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_MalformedMessage();

CREATE UNLOGGED TABLE MalformedContent (
    id SERIAL PRIMARY KEY,
    encoded_data BYTEA NOT NULL
);
ALTER SEQUENCE MalformedContent_id_seq CYCLE;

CREATE INDEX ApplicationInfo_Index_identity ON ApplicationInfo(identity);

CREATE INDEX ApplicationInfo_Index_version ON ApplicationInfo(version);

CREATE INDEX ApplicationInfo_Index_build ON ApplicationInfo(build);

CREATE OR REPLACE FUNCTION DelChilds_ApplicationInfo() RETURNS TRIGGER AS
$$ BEGIN
    DELETE FROM Identity WHERE id = OLD.identity;
    DELETE FROM Version WHERE id = OLD.version;
    DELETE FROM BuildInfo WHERE id = OLD.build;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER OnDeleteDelChilds_ApplicationInfo AFTER DELETE ON ApplicationInfo
    FOR EACH ROW
    EXECUTE PROCEDURE DelChilds_ApplicationInfo();

CREATE INDEX ApplicationInfo_Args_Index_list ON ApplicationInfo_Args(list);

CREATE INDEX ApplicationInfo_Args_Index_value ON ApplicationInfo_Args(value);
