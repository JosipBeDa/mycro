use crate::api::{
    middleware::auth::interceptor::AuthGuard,
    router::auth::{
        adapter::{Cache, Repository},
        o_auth::{handler, service::OAuthService},
    },
};
use actix_web::web::{self, Data};
use alx_core::clients::{
    db::{
        postgres::{PgPoolConnection, Postgres},
        redis::Redis,
    },
    oauth::github::GithubOAuth,
};
use std::sync::Arc;
use storage::{
    adapters::postgres::{oauth::PgOAuthAdapter, session::PgSessionAdapter, user::PgUserAdapter},
    models::role::Role,
};

pub(crate) fn routes(pg: Arc<Postgres>, rd: Arc<Redis>, cfg: &mut web::ServiceConfig) {
    let service = OAuthService {
        provider: GithubOAuth,
        repo: Repository::<
            Postgres,
            PgPoolConnection,
            PgUserAdapter,
            PgSessionAdapter,
            PgOAuthAdapter,
        >::new(pg.clone()),
        cache: Cache { client: rd.clone() },
    };

    let auth_guard = AuthGuard::new(pg, rd, Role::User);

    cfg.app_data(Data::new(service));

    cfg.service(
        web::resource("/auth/oauth/github/login").route(web::post().to(handler::login::<
            OAuthService<
                GithubOAuth,
                Repository<
                    Postgres,
                    PgPoolConnection,
                    PgUserAdapter,
                    PgSessionAdapter,
                    PgOAuthAdapter,
                >,
                Cache,
            >,
        >)),
    );

    cfg.service(
        web::resource("/auth/oauth/github/scope")
            .route(web::put().to(handler::request_scopes::<
                OAuthService<
                    GithubOAuth,
                    Repository<
                        Postgres,
                        PgPoolConnection,
                        PgUserAdapter,
                        PgSessionAdapter,
                        PgOAuthAdapter,
                    >,
                    Cache,
                >,
            >))
            .wrap(auth_guard),
    );
}
