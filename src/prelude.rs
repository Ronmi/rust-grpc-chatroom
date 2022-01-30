//! 共用的必要程式
use tonic::Status;
use tracing::error;

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
                err => {
                    error!(?err, "未預期的 sql 錯誤");
                    Err(err)
                }
            },
        }
    }
}

/// 把各種有問題的情況轉成 internal server error
pub trait To500<T> {
    /// 把各種有問題的情況轉成 internal server error
    fn to500(self) -> Result<T, Status>;
}

impl<T, E: std::fmt::Display> To500<T> for Result<T, E> {
    fn to500(self) -> Result<T, Status> {
        match self {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(%err, "未預期的錯誤");

                Err(Status::internal(err.to_string()))
            }
        }
    }
}
