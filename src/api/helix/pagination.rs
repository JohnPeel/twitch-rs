use std::error::Error;
use std::future::Future;
use serde::{Serialize, Deserialize};
use super::result::ApiResult;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct ForwardPagination {
    pub(crate) after: Option<String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct BackwardPagination {
    pub(crate) before: Option<String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Pagination {
    #[serde(flatten)]
    pub(crate) forward: Option<ForwardPagination>,
    #[serde(flatten)]
    pub(crate) backward: Option<BackwardPagination>
}

pub async fn get_all<T, F, R>(grabber: F, limit: Option<usize>) -> Result<Vec<T>, Box<dyn Error>>
where
    F: Fn(Option<String>) -> R,
    R: Future<Output = Result<ApiResult<T>, Box<dyn Error>>>
{
    let mut list = vec![];
    let mut current_pagination = None;

    loop {
        let mut result = grabber(current_pagination.clone()).await?;
        list.append(&mut result.data);

        if let Some(pagination) = result.pagination {
            current_pagination = pagination.cursor.clone();
        }

        if current_pagination.is_none() || limit.map_or(false, |x| x < list.len()) {
            break;
        }
    }

    Ok(list)
}