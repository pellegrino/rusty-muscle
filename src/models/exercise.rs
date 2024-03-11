use anyhow::{anyhow, Error}; // Import anyhow::Error and the anyhow! macro for creating errors
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "target_muscle", rename_all = "snake_case")]
pub enum TargetMuscle {
    Chest,
    Back,
    Shoulders,
    Arms,
    Core,
    Legs,
}

impl std::fmt::Display for TargetMuscle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target_muscle = match self {
            TargetMuscle::Chest => "chest",
            TargetMuscle::Back => "back",
            TargetMuscle::Shoulders => "shoulders",
            TargetMuscle::Arms => "arms",
            TargetMuscle::Core => "core",
            TargetMuscle::Legs => "legs",
        };
        write!(f, "{}", target_muscle)
    }
}
//
impl TryFrom<&str> for TargetMuscle {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "chest" => Ok(TargetMuscle::Chest),
            "back" => Ok(TargetMuscle::Back),
            "shoulders" => Ok(TargetMuscle::Shoulders),
            "arms" => Ok(TargetMuscle::Arms),
            "core" => Ok(TargetMuscle::Core),
            "legs" => Ok(TargetMuscle::Legs),
            _ => Err(anyhow!("Unknown target muscle: {}", value)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Exercise {
    pub id: i32,
    pub target_group: TargetMuscle,
    pub additional_groups: Option<Vec<TargetMuscle>>,
    pub name: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result; // Import the Result type from anyhow for use in the tests.
    use std::convert::TryInto; // Import the TryInto trait which provides the try_into method.

    #[test]
    fn test_try_from_str_for_target_muscle_valid() -> Result<()> {
        // Test for valid conversions
        assert_eq!(TargetMuscle::try_from("chest")?, TargetMuscle::Chest);
        assert_eq!(TargetMuscle::try_from("back")?, TargetMuscle::Back);
        assert_eq!(
            TargetMuscle::try_from("shoulders")?,
            TargetMuscle::Shoulders
        );
        // Continue for other variants...

        Ok(()) // Return Ok if all assertions pass
    }

    #[test]
    fn test_try_from_str_for_target_muscle_invalid() {
        // Test for an invalid conversion
        let result: Result<TargetMuscle, _> = "unknown".try_into(); // Using try_into for a more idiomatic approach
        assert!(result.is_err()); // Assert that the result is an error

        // If you want to check the error message as well, you can do something like:
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Unknown target muscle: unknown");
        }
    }
}
