//! Request Extensions

use std::convert::Infallible;

use futures::future;
use http::Extensions;

use crate::filter::{filter_fn, filter_fn_one, Filter};
use crate::reject::{self, Rejection};

/// Get a previously set extension of the current route.
///
/// If the extension doesn't exist, this rejects with a `MissingExtension`.
pub fn get<T: Clone + Send + Sync + 'static>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    filter_fn_one(|route| {
        let route = route
            .extensions()
            .get::<T>()
            .cloned()
            .ok_or_else(|| reject::known(MissingExtension { _p: () }));
        future::ready(route)
    })
}

/// Get a previously set extension of the current route.
///
/// If the extension doesn't exist, it yields `None`.
pub fn optional<T: Clone + Send + Sync + 'static>(
) -> impl Filter<Extract = (Option<T>,), Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok(route.extensions().get::<T>().cloned()))
}

/// Change the extensions of the current route.
pub fn with_mut<F>(func: F) -> impl Filter<Extract = (), Error = Rejection> + Clone
where
    F: Fn(&mut Extensions) -> Result<(), Rejection> + Clone + 'static,
{
    filter_fn(move |route| future::ready(func(route.extensions_mut())))
}

unit_error! {
    /// An error used to reject if `get` cannot find the extension.
    pub MissingExtension: "Missing request extension"
}
