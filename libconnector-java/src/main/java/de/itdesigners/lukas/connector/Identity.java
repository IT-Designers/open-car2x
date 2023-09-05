package de.itdesigners.lukas.connector;

public enum Identity {
    Vehicle(100),
    NomadicDevice(101),
    FusionModule(200),
    PlanningModule(201),
    WarningModule(202);

    protected final int numeric;

    Identity(int id) {
        this.numeric = id;
    }

    public int getNumeric() {
        return numeric;
    }
}
