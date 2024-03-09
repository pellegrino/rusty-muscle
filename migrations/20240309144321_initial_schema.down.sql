-- Start of Down Migrations

-- To revert, we drop the tables and the enum in the reverse order of creation
DROP TABLE IF EXISTS routine_sets;
DROP TABLE IF EXISTS exercise_instances;
DROP TABLE IF EXISTS routines;
DROP TABLE IF EXISTS exercises;
DROP TYPE IF EXISTS target_muscle;

-- End of Down Migrations
