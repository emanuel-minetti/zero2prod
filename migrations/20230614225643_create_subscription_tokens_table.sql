CREATE TABLE subscription_tokens(
    subsrption_token text NOT NULL,
    subscriber_id uuid NOT NULL
        REFERENCES subscriptions (id),
    PRIMARY KEY (subsrption_token)
);
