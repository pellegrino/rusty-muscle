use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::Type;

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "target_muscle", rename_all = "snake_case")]
enum TargetMuscle {
    Chest,
    Back,
    Shoulders,
    Arms,
    Core,
    Legs,
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct Exercise {
    id: i32,
    target_group: TargetMuscle,
    additional_groups: Option<Vec<TargetMuscle>>,
    name: String,
}

#[derive(Clone, Debug)]
struct ExerciseInstance {
    exercise: Exercise,
    repetitions: i32,
    description: String,
}

#[derive(Clone, Debug)]
struct Routine {
    name: String,
    sets: Vec<ExerciseInstance>,
}

// Rust struct for inserting a new exercise (without an ID)
struct NewExercise<'a> {
    name: &'a str,
    target_group: &'a str, // Target muscle as string
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> &'static str {
    "Hello root"
}

async fn create_user() -> &'static str {
    "Hello root"
}
// Function to add a new exercise to the database
async fn add_exercise(
    pool: &sqlx::PgPool,
    new_exercise: NewExercise<'_>,
) -> Result<Exercise, sqlx::Error> {
    let exercise = sqlx::query_as!(
        Exercise,
        "INSERT INTO exercises (name, target_group) VALUES ($1, $2) RETURNING id, name, target_group",
        new_exercise.name,
        new_exercise.target_group
    )
    .fetch_one(pool)
    .await?;

    Ok(exercise)
}

// Function to retrieve all exercises from the database
async fn get_exercises(pool: &sqlx::PgPool) -> Result<Vec<Exercise>, sqlx::Error> {
    let exercises = sqlx::query_as!(Exercise, "SELECT id, name, target_group FROM exercises")
        .fetch_all(pool)
        .await?;

    Ok(exercises)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_muscle_display() {
        let muscle = TargetMuscle::Chest;
        assert_eq!(muscle.to_string(), "Chest");
    }

    // Add more unit tests for functions that don't interact with the database
}
