CREATE TYPE target_muscle AS ENUM ('Chest', 'Back', 'Shoulders', 'Arms', 'Core', 'Legs');

CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    target_group target_muscle NOT NULL
);

CREATE TABLE exercise_target_groups (
    exercise_id INT NOT NULL,
    target_muscle target_muscle NOT NULL,
    FOREIGN KEY (exercise_id) REFERENCES exercises(id) ON DELETE CASCADE,
    PRIMARY KEY (exercise_id, target_muscle)
);

-- Create the exercise instances table
CREATE TABLE exercise_instances (
    id SERIAL PRIMARY KEY,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id),
    repetitions INTEGER NOT NULL,
    description TEXT NOT NULL
);

-- Create the routines table
CREATE TABLE routines (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

-- Create the routine sets (junction table between routines and exercise instances)
CREATE TABLE routine_sets (
    routine_id INTEGER NOT NULL REFERENCES routines(id),
    exercise_instance_id INTEGER NOT NULL REFERENCES exercise_instances(id),
    PRIMARY KEY (routine_id, exercise_instance_id)
);
