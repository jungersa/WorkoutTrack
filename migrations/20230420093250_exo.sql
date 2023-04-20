CREATE TABLE IF NOT EXISTS Exo (
    id INTEGER NOT NULL,
    reps_exo NUMERIC(64) NOT NULL,
    reps_rep NUMERIC(64) NOT NULL,
    poids NUMERIC(64),
    constraint ID_Exo_ID PRIMARY KEY (id)
);


create unique index ID_Exo_IND on Exo (id);