-- This file should undo anything in `up.sql`
DROP TABLE messages;
DROP TABLE exopredefs;
DROP TABLE workouts;
DROP TABLE exos;

-- Index Section
-- _____________
DROP INDEX ID_ExoPredef_IND;
DROP INDEX ID_Workout_IND;
DROP INDEX ID_Exo_IND;
DROP INDEX FKR_IND;
DROP INDEX FKExos_IND;