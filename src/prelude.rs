//! 共用的必要程式

/// 把沒有資料的情況改成空陣列
pub trait SkipNoData<T> {
    /// 把沒有資料的情況改成空陣列
    fn skip_no_data(self) -> sqlx::Result<T>;
}

/// 針對 Vec 實作
impl<X> SkipNoData<Vec<X>> for sqlx::Result<Vec<X>> {
    fn skip_no_data(self) -> sqlx::Result<Vec<X>> {
        match self {
            Ok(x) => Ok(x),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Ok(vec![]),
                x => Err(x),
            },
        }
    }
}
