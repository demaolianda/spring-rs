use std::{ops::Deref, sync::Arc};

use spring_boot::async_trait;
use spring_web::{
    extractor::FromRequestParts,
    http::{request::Parts, StatusCode},
    AppState,
};

pub(crate) struct App(Arc<spring_boot::app::App>);

#[async_trait]
impl FromRequestParts<AppState> for App {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        Ok(Self(state.app.clone()))
    }
}

impl Deref for App {
    type Target = spring_boot::app::App;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
