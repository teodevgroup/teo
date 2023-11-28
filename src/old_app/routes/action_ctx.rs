use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::app::routes::req::Req;
use crate::app::routes::req_local::ReqLocal;
use crate::app::routes::res::Res;
use crate::core::result::Result;
use crate::prelude::Value;
use crate::server::ReqCtx;

impl ExtractValueFromReqCtx for Req {
    fn extract(ctx_base: &ReqCtx) -> Self {
        ctx_base.req.clone()
    }
}

impl ExtractValueFromReqCtx for ReqLocal {
    fn extract(req_ctx: &ReqCtx) -> Self {
        req_ctx.req_local.clone()
    }
}

impl ExtractValueFromReqCtx for Value {
    fn extract(ctx_base: &ReqCtx) -> Self {
        ctx_base.transformed_teon_body.clone()
    }
}