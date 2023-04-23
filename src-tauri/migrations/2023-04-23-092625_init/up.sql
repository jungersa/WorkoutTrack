-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages (
     id INTEGER NOT NULL PRIMARY KEY,
     uuid VARCHAR NOT NULL UNIQUE,
     content varchar(128) not null
);

CREATE TABLE IF NOT EXISTS exopredefs (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid VARCHAR NOT NULL UNIQUE,
    name varchar not null
);

CREATE TABLE IF NOT EXISTS workouts (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid VARCHAR NOT NULL UNIQUE,
    title varchar not null,
    work_date DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS exos (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid VARCHAR NOT NULL UNIQUE,
    reps_exo NUMERIC(64) NOT NULL,
    reps_rep NUMERIC(64) NOT NULL,
    poids NUMERIC(64),
    exopredef_id INTEGER not null,
    workout_id INTEGER not null,
    FOREIGN KEY (exopredef_id) REFERENCES ExoPredef(id),
    FOREIGN KEY(workout_id) REFERENCES Workout(id)
);

-- Index Section
-- _____________

create unique index ID_ExoPredef_IND
     on exopredefs (id);

create unique index ID_Workout_IND
     on workouts (id);

create unique index ID_Exo_IND
     on exos (id);

create index FKR_IND
     on exos (exopredef_id);

create index FKExos_IND
     on exos (workout_id);
