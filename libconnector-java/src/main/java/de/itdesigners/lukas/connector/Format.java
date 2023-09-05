package de.itdesigners.lukas.connector;

public enum Format {
    Uper(0),
    Json(1),
    JsonPretty(2),
    Bson(3),
    Protobuf(4);

    protected final int numeric;

    Format(int numeric) {
        this.numeric = numeric;
    }

    public int getNumeric() {
        return numeric;
    }
}
