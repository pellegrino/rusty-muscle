use my_crate::test_utils::setup_test_db; // Adjust the import path as necessary
use my_crate::*;
use sqlx::postgres::PgPoolOptions; // Import your modules

#[tokio::test]
async fn test_add_exercise() {
    let pool = setup_test_db().await;

    let new_exercise = NewExercise {
        name: "Push-up",
        target_group: "Chest",
    };

    let result = add_exercise(&pool, new_exercise).await;
    assert!(result.is_ok());

    let exercise = result.unwrap();
    assert_eq!(exercise.name, "Push-up");
    assert_eq!(exercise.target_group, TargetMuscle::Chest);
    // Add more assertions as needed

    // Optionally, clean up the test data from the database
}
