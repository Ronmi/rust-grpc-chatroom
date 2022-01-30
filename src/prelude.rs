//! 共用的必要程式

/// 處理 sql no row error，改成回傳空陣列
#[macro_export]
macro_rules! skip_no_row {
    ($var:ident) => {
        match $var {
            Ok(x) => Ok(x),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Ok(vec![]),
                x => Err(x),
            },
        }
    };
}
