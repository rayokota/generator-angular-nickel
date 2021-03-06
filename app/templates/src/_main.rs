#[macro_use] 
extern crate nickel;
#[macro_use] 
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate plugin;
extern crate typemap;
extern crate rustc_serialize;

mod middleware;
mod schema;

use diesel::*;
use diesel::query_builder::*;
use nickel::{ Nickel, JsonBody, HttpRouter, StaticFilesHandler };
use nickel::status::StatusCode;
use rustc_serialize::json;

use r2d2::NopErrorHandler;
use r2d2_diesel::ConnectionManager;

use middleware::*;
use schema::*;

fn main() {
    let mut server = Nickel::new();
    let dbpool = DieselMiddleware::new("postgres://postgres:postgres@localhost:5432/my_db",
                                       5,
                                       Box::new(NopErrorHandler)).unwrap();
    server.utilize(dbpool);
    server.utilize(StaticFilesHandler::new("public/"));

    <% _.each(entities, function (entity) { %>
    server.get("/<%= baseName %>/<%= pluralize(entity.name) %>", middleware! { |request, response|
        use self::schema::<%= pluralize(entity.name) %>::dsl::*;
        let connection = request.db_conn();
        let entities: Vec<<%= _.capitalize(entity.name) %>> = <%= pluralize(entity.name) %>.load(&connection).unwrap().collect();
        json::encode(&entities).unwrap()
    });

    server.get("/<%= baseName %>/<%= pluralize(entity.name) %>/:id", middleware! { |request, response|
        use self::schema::<%= pluralize(entity.name) %>::dsl::*;
        let connection = request.db_conn();
        let id_param = request.param("id").unwrap().parse::<i32>().unwrap();
        let entities: Vec<<%= _.capitalize(entity.name) %>> = <%= pluralize(entity.name) %>.filter(id.eq(id_param)).load(&connection).unwrap().collect();
        if !entities.is_empty() {
            (StatusCode::Ok, json::encode(&entities[0]).unwrap())
        } else {
            (StatusCode::NotFound, "Not found".to_string())
        }
    });

    server.post("/<%= baseName %>/<%= pluralize(entity.name) %>", middleware! { |request, response|
        let connection = request.db_conn();
        let new_entity = request.json_as::<New<%= _.capitalize(entity.name) %>>().unwrap();
        let entity = insert(&new_entity).into(<%= pluralize(entity.name) %>::table).get_result::<<%= _.capitalize(entity.name) %>>(&connection);
        match entity {
            Ok(data) => (StatusCode::Created, json::encode(&data).unwrap()),
            Err(e) => (StatusCode::NotFound, "Not found".to_string())
        }
    });

    server.put("/<%= baseName %>/<%= pluralize(entity.name) %>/:id", middleware! { |request, response|
        use self::schema::<%= pluralize(entity.name) %>::dsl::*;
        let connection = request.db_conn();
        let id_param = request.param("id").unwrap().parse::<i32>().unwrap();
        let new_entity = request.json_as::<New<%= _.capitalize(entity.name) %>>().unwrap();
        let entity = update(<%= pluralize(entity.name) %>.filter(id.eq(id_param))).set(&new_entity).get_result::<<%= _.capitalize(entity.name) %>>(&connection);
        match entity {
            Ok(data) => (StatusCode::Ok, json::encode(&data).unwrap()),
            Err(e) => (StatusCode::NotFound, "Not found".to_string())
        }
    });

    server.delete("/<%= baseName %>/<%= pluralize(entity.name) %>/:id", middleware! { |request, response|
        use self::schema::<%= pluralize(entity.name) %>::dsl::*;
        let connection = request.db_conn();
        let id_param = request.param("id").unwrap().parse::<i32>().unwrap();
        let count = delete(<%= pluralize(entity.name) %>.filter(id.eq(id_param))).execute(&connection).unwrap();
        if count > 0 {
            (StatusCode::NoContent, "Deleted".to_string())
        } else {
            (StatusCode::InternalServerError, "Error".to_string())
        }
    });
    <% }); %>

    server.listen("127.0.0.1:8080");
}
