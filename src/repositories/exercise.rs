use sqlx::PgPool;

use crate::models::exercise::{Exercise, TargetMuscle};

#[derive(sqlx::FromRow)]
struct ExerciseDTO {
    id: i32,
    target_group: String,
    name: String,
    additional_groups: Vec<String>,
}

pub struct ExerciseRepository {
    pool: PgPool,
}

impl ExerciseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_exercise(&self, exercise_id: i32) -> Result<Exercise, anyhow::Error> {
        let exercise_dto = sqlx::query_as!(
            ExerciseDTO,
            r#"
            SELECT exercises.id, exercises.name, exercises.target_group,
            COALESCE(array_agg(exercise_target_groups.target_muscle) FILTER (WHERE exercise_target_groups.target_muscle IS NOT NULL), '{}'::target_muscle[]) AS additional_groups
            FROM exercises
            LEFT JOIN exercise_target_groups ON exercises.id = exercise_target_groups.exercise_id
            WHERE exercises.id = $1
            GROUP BY exercises.id
            "#,
            exercise_id
        )
        .fetch_one(&self.pool)
        .await?;

        Exercise::try_from(exercise_dto)
    }
}

impl TryFrom<ExerciseDTO> for Exercise {
    type Error = anyhow::Error;

    fn try_from(dto: ExerciseDTO) -> Result<Self, Self::Error> {
        let target_muscle: TargetMuscle = dto.target_group.as_str().try_into()?;

        let additional_groups = dto
            .additional_groups
            .map(|target_group| target_group.trim().try_into().map_err(anyhow::Error::from))
            .ok_or(anyhow::anyhow!("No additional groups"))?;

        Ok(Exercise {
            id: dto.id,
            target_group: target_muscle,
            name: dto.name,
            additional_groups: additional_groups?,
        })
    }
}
// Rust struct for inserting a new exercise (without an ID)
pub struct NewExercise<'a> {
    name: &'a str,
    target_group: TargetMuscle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    // Helper function to create an ExerciseDTO for testing
    fn create_exercise_dto(
        target_group: &str,
        additional_groups: Option<&str>,
        name: &str,
    ) -> ExerciseDTO {
        ExerciseDTO {
            id: 1,
            target_group: target_group.into(),
            name: name.into(),
            additional_groups: additional_groups.map(|s| s.into()),
        }
    }

    #[test]
    fn test_try_from_exercise_dto_success() -> Result<()> {
        let dto = create_exercise_dto("Chest", Some("Back,Arms"), "Push Up");
        let exercise = Exercise::try_from(dto)?;

        assert_eq!(exercise.id, 1);
        assert_eq!(exercise.name, "Push Up");
        assert_eq!(exercise.target_group, TargetMuscle::Chest);
        assert!(exercise.additional_groups.is_some());

        let additional_groups = exercise
            .additional_groups
            .expect("additional_groups is not None");
        assert_eq!(additional_groups.len(), 2);
        assert!(additional_groups.contains(&TargetMuscle::Back));
        assert!(additional_groups.contains(&TargetMuscle::Arms));

        Ok(())
    }

    #[test]
    fn test_try_from_exercise_dto_invalid_target_group() {
        let dto = create_exercise_dto("InvalidMuscle", None, "Invalid Exercise");
        let result = Exercise::try_from(dto);

        assert!(result.is_err());
        assert!(result.is_ok());
    }
}
