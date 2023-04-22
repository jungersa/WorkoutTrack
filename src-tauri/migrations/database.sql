-- Tables Section
-- _____________

create table Message (
    content varchar(128) not null,
);

create table ExoPredef (
    id INTEGER NOT NULL,
    name varchar(128) not null,
    constraint ID_ExoPredef_ID primary key (id)
);

create table Workout (
    id INTEGER NOT NULL,
    name varchar(128) not null,
    constraint ID_Workout_ID primary key (id)
);

CREATE TABLE IF NOT EXISTS Exo (
    id INTEGER NOT NULL,
    reps_exo NUMERIC(64) NOT NULL,
    reps_rep NUMERIC(64) NOT NULL,
    poids NUMERIC(64),
    exopredef_id INTEGER not null,
    workout_id INTEGER not null,
    constraint ID_Exo_ID PRIMARY KEY (id),
    FOREIGN KEY (exopredef_id) REFERENCES ExoPredef(id)
    FOREIGN KEY(workout_id) REFERENCES Workout(id)
);


-- Index Section
-- _____________

create unique index ID_ExoPredef_IND
     on ExoPredef (id);

create unique index ID_Workout_IND
     on Workout (id);

create unique index ID_Exo_IND
     on Exo (id);

create index FKR_IND
     on Exo (exopredef_id);

create index FKExos_IND
     on Exo (workout_id);