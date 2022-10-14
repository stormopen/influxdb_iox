use crate::internal::{expect, ParseResult};
use crate::show::{on_clause, OnClause};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::sequence::{preceded, tuple};
use std::fmt::{Display, Formatter};

/// Represents a `SHOW RETENTION POLICIES` statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowRetentionPoliciesStatement {
    /// Name of the database to list the retention policies, or all if this is `None`.
    pub database: Option<OnClause>,
}

impl Display for ShowRetentionPoliciesStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SHOW RETENTION POLICIES")?;
        if let Some(ref database) = self.database {
            write!(f, " {}", database)?;
        }
        Ok(())
    }
}

pub(crate) fn show_retention_policies(
    i: &str,
) -> ParseResult<&str, ShowRetentionPoliciesStatement> {
    let (remaining, (_, _, _, database)) = tuple((
        tag_no_case("RETENTION"),
        multispace1,
        expect(
            "invalid SHOW RETENTION POLICIES statement, expected POLICIES",
            tag_no_case("POLICIES"),
        ),
        opt(preceded(multispace1, on_clause)),
    ))(i)?;

    Ok((remaining, ShowRetentionPoliciesStatement { database }))
}

#[cfg(test)]
mod test {
    use crate::assert_expect_error;
    use crate::show_retention_policies::show_retention_policies;

    #[test]
    fn test_show_retention_policies() {
        // no ON clause
        show_retention_policies("RETENTION POLICIES").unwrap();

        // with ON clause
        show_retention_policies("RETENTION POLICIES ON foo").unwrap();

        // Fallible cases

        // missing POLICIES keyword
        assert_expect_error!(
            show_retention_policies("RETENTION ON foo"),
            "invalid SHOW RETENTION POLICIES statement, expected POLICIES"
        );

        // missing database
        assert_expect_error!(
            show_retention_policies("RETENTION POLICIES ON "),
            "invalid ON clause, expected identifier"
        );
    }
}
