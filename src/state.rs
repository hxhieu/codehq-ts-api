use actix_web::{dev::ServiceRequest, HttpMessage, HttpRequest};

#[derive(Debug, Default, Clone)]
pub struct RequestContext {
    pub user: String,
}

impl RequestContext {
    pub fn set_user(req: &ServiceRequest, user: &str) {
        let mut ctx = RequestContext::default();
        // Load existing ctx, if there is any
        if let Some(current_ctx) = req.extensions().get::<RequestContext>() {
            ctx = current_ctx.clone();
            req.extensions_mut().remove::<RequestContext>();
        }

        // Update request extensions
        ctx.user = user.to_string();
        req.extensions_mut().insert(ctx);
    }

    pub fn new(req: &HttpRequest) -> RequestContext {
        match req.extensions().get::<RequestContext>() {
            Some(ctx) => ctx.clone(),
            None => RequestContext::default(),
        }
    }
}
