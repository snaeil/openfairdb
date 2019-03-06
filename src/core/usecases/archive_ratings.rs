use crate::core::prelude::*;

use chrono::Utc;

pub fn archive_ratings<D: Db>(db: &D, ids: &[&str]) -> Result<()> {
    debug!("Archiving ratings {:?}", ids);
    let archived = Utc::now().timestamp() as u64;
    db.archive_comments_of_ratings(ids, archived)?;
    db.archive_ratings(ids, archived)?;
    Ok(())
}
