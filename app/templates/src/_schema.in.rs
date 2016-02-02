use diesel::*;

<% _.each(entities, function (entity) { %>
infer_table_from_schema!("postgres://postgres:postgres@localhost/my_db", "<%= pluralize(entity.name) %>");

#[derive(RustcDecodable, RustcEncodable)]
#[changeset_for(<%= pluralize(entity.name) %>)]
#[insertable_into(<%= pluralize(entity.name) %>)]
pub struct New<%= _.capitalize(entity.name) %> {
    <% _.each(entity.attrs, function (attr) { %>
    pub <%= attr.attrName %>: <% if (!attr.required) { %>Option<<% }; %><%= attr.attrImplType %><% if (!attr.required) {%>><% }; %>,<% }); %>
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Queriable)]
#[changeset_for(<%= pluralize(entity.name) %>)]
pub struct <%= _.capitalize(entity.name) %> {
    <% _.each(entity.attrs, function (attr) { %>
    pub <%= attr.attrName %>: <% if (!attr.required) { %>Option<<% }; %><%= attr.attrImplType %><% if (!attr.required) {%>><% }; %>,<% }); %>
    pub id: i32
}
<% }); %>
