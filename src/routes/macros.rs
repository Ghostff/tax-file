macro_rules! resource {
    ($path:expr,$controller:ident, [$($action:ident),+ $(,)?]) => {{
        use actix_web::web;
        let mut scope = web::scope($path);
        $(scope = resource!(@route scope, $controller, $action);)+

        scope
    }};

    // Collection routes
    (@route $scope:expr, $controller:ident, index) => {
        $scope.service(web::resource("").route(web::get().to($controller::index)))
    };

    (@route $scope:expr, $controller:ident, create) => {
        $scope.service(web::resource("").route(web::post().to($controller::create)))
    };

    // Member routes
    (@route $scope:expr, $controller:ident, show) => {
        $scope.service(web::resource("/{id}").route(web::get().to($controller::show)))
    };

    (@route $scope:expr, $controller:ident, update) => {
        $scope.service(web::resource("/{id}").route(web::put().to($controller::update)))
    };

    (@route $scope:expr, $controller:ident, delete) => {
        $scope.service(web::resource("/{id}").route(web::delete().to($controller::delete)))
    };
}

macro_rules! get {
    ($handler:path) => {actix_web::web::get().to($handler)};
}
macro_rules! post {
    ($handler:path) => {actix_web::web::post().to($handler)};
}
macro_rules! put {
    ($handler:path) => {actix_web::web::put().to($handler)};
}

macro_rules! delete {
    ($handler:path) => {actix_web::web::delete().to($handler)};
}

