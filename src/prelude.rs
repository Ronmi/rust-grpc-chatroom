//! 共用的必要程式

/// 處理 sql no row error，改成回傳空陣列
pub fn skip_no_row<T>(res: sqlx::Result<Vec<T>>) -> sqlx::Result<Vec<T>> {
    match res {
        Ok(x) => Ok(x),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(vec![]),
            x => Err(x),
        },
    }
}
