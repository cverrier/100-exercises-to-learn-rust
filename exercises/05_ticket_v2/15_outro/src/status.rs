use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    invalid_status: String,
}

// NOTE: unlike `TicketTitle`/`TicketDescription`, `Status` doesn't store the input
// string, so `TryFrom<&str>` is the primary impl and `TryFrom<String>` delegates to it.
// Don't mirror their `validate` + `TryFrom<String>`-first pattern here: it would add a
// throwaway allocation to every `&str` parse.
impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") {
            Ok(Self::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") {
            Ok(Self::InProgress)
        } else if value.eq_ignore_ascii_case("done") {
            Ok(Self::Done)
        } else {
            Err(ParseStatusError {
                invalid_status: value.to_string(),
            })
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
