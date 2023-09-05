package de.itdesigners.lukas.connector;

import java.util.List;
import java.util.Objects;

import javax.annotation.Nullable;

public class ConnectionConfig {

    protected @Nullable String          address;
    protected @Nullable Long            reconnectTimeoutMillis;
    protected @Nullable Long            sendTimeoutMillis;
    protected @Nullable Boolean         receiveOwnMessage;
    protected @Nullable List<MessageId> filterOptions;
    protected @Nullable String          loginUser;
    protected @Nullable String          loginPassword;
    protected @Nullable Boolean         anonymous;
    protected @Nullable String          targetExchange;
    protected @Nullable String          sourceExchange;
    protected @Nullable Integer         stationId;
    protected @Nullable Integer         stationIdReceiveFilter;

    public ConnectionConfig() {

    }

    @Nullable
    public String getAddress() {
        return address;
    }

    public ConnectionConfig setAddress(@Nullable String address) {
        this.address = address;
        return this;
    }

    @Nullable
    public Long getReconnectTimeoutMillis() {
        return reconnectTimeoutMillis;
    }

    public ConnectionConfig setReconnectTimeoutMillis(Long reconnectTimeoutMillis) {
        this.reconnectTimeoutMillis = reconnectTimeoutMillis;
        return this;
    }

    @Nullable
    public Long getSendTimeoutMillis() {
        return sendTimeoutMillis;
    }

    public ConnectionConfig setSendTimeoutMillis(Long sendTimeoutMillis) {
        this.sendTimeoutMillis = sendTimeoutMillis;
        return this;
    }

    @Nullable
    public Boolean getReceiveOwnMessage() {
        return receiveOwnMessage;
    }

    public ConnectionConfig setReceiveOwnMessage(@Nullable Boolean receiveOwnMessage) {
        this.receiveOwnMessage = receiveOwnMessage;
        return this;
    }

    @Nullable
    public List<MessageId> getFilterOptions() {
        return filterOptions;
    }

    public ConnectionConfig setFilterOptions(@Nullable List<MessageId> filterOptions) {
        this.filterOptions = filterOptions;
        return this;
    }

    @Nullable
    public String getLoginUser() {
        return loginUser;
    }

    public ConnectionConfig setLoginUser(@Nullable String loginUser) {
        this.loginUser = loginUser;
        return this;
    }

    @Nullable
    public String getLoginPassword() {
        return loginPassword;
    }

    public ConnectionConfig setLoginPassword(@Nullable String loginPassword) {
        this.loginPassword = loginPassword;
        return this;
    }

    @Nullable
    public Boolean getAnonymous() {
        return anonymous;
    }

    public ConnectionConfig setAnonymous(@Nullable Boolean anonymous) {
        this.anonymous = anonymous;
        return this;
    }

    @Nullable
    public String getTargetExchange() {
        return targetExchange;
    }

    public ConnectionConfig setTargetExchange(@Nullable String targetExchange) {
        this.targetExchange = targetExchange;
        return this;
    }

    @Nullable
    public String getSourceExchange() {
        return sourceExchange;
    }

    public ConnectionConfig setSourceExchange(@Nullable String sourceExchange) {
        this.sourceExchange = sourceExchange;
        return this;
    }

    @Nullable
    public Integer getStationId() {
        return stationId;
    }

    public ConnectionConfig setStationId(@Nullable Integer stationId) {
        this.stationId = stationId;
        return this;
    }

    @Nullable
    public Integer getStationIdReceiveFilter() {
        return stationIdReceiveFilter;
    }

    public ConnectionConfig setStationIdReceiveFilter(@Nullable Integer stationIdReceiveFilter) {
        this.stationIdReceiveFilter = stationIdReceiveFilter;
        return this;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o)
            return true;
        if (o == null || getClass() != o.getClass())
            return false;
        ConnectionConfig that = (ConnectionConfig) o;
        return Objects.equals(address, that.address) && Objects.equals(
                reconnectTimeoutMillis,
                that.reconnectTimeoutMillis
        ) && Objects.equals(sendTimeoutMillis, that.sendTimeoutMillis) && Objects.equals(
                receiveOwnMessage,
                that.receiveOwnMessage
        ) && Objects.equals(filterOptions, that.filterOptions) && Objects.equals(
                loginUser,
                that.loginUser
        ) && Objects.equals(loginPassword, that.loginPassword) && Objects.equals(
                anonymous,
                that.anonymous
        ) && Objects.equals(targetExchange, that.targetExchange) && Objects.equals(
                sourceExchange,
                that.sourceExchange
        ) && Objects.equals(stationId, that.stationId) && Objects.equals(
                stationIdReceiveFilter,
                that.stationIdReceiveFilter
        );
    }

    @Override
    public int hashCode() {
        return Objects.hash(
                address,
                reconnectTimeoutMillis,
                sendTimeoutMillis,
                receiveOwnMessage,
                filterOptions,
                loginUser,
                loginPassword,
                anonymous,
                targetExchange,
                sourceExchange,
                stationId,
                stationIdReceiveFilter
        );
    }

    @Override
    public String toString() {
        return "ConnectionConfig{" +
                "address='" + address + '\'' +
                ", reconnectTimeoutMillis=" + reconnectTimeoutMillis +
                ", sendTimeoutMillis=" + sendTimeoutMillis +
                ", receiveOwnMessage=" + receiveOwnMessage +
                ", filterOptions=" + filterOptions +
                ", loginUser='" + loginUser + '\'' +
                ", loginPassword='" + loginPassword + '\'' +
                ", anonymous=" + anonymous +
                ", targetExchange='" + targetExchange + '\'' +
                ", sourceExchange='" + sourceExchange + '\'' +
                ", stationId=" + stationId +
                ", stationIdReceiveFilter=" + stationIdReceiveFilter +
                "}@" + hashCode();
    }
}
