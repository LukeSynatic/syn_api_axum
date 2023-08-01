pub mod state {
    pub mod state;
}

pub mod middleware {
    pub mod mongo;
    pub mod headers;
}

pub mod routes {
    pub mod mongo;
}

pub mod utils {
    pub mod mongo;
}

pub mod types {
    pub mod mongo {
        pub mod traits {
            pub mod requests;
        }
        pub mod requests {
            pub mod aggregate;
            pub mod delete;
            pub mod find_one;
            pub mod find;
            pub mod insert_one;
            pub mod insert_many;
            pub mod update;
        }
    }
}

use std::env;
use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use routes::mongo::mongo_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .nest("/v1", mongo_router().await);

    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("PORT")
        .expect("Error: Failed to get PORT from environment")
        .parse::<u16>()
        .expect("Error: Failed to parse PORT from environment")));
    
    println!("🚀 Server starting on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
