use actix_web::App;



pub trait Router {
    fn setup_router(self) -> Self;
}

impl <T> Router for App<T> {

    fn setup_router(self) -> Self {
        self
    }

}
