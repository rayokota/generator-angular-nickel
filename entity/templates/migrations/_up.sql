CREATE TABLE <%= pluralize(name) %> (
    <% _.each(attrs, function (attr) { %>
    <%= attr.attrName %> <%= attr.attrSqlType %><% if (attr.attrType == 'Enum' || attr.attrType == 'String') { if (attr.maxLength) { %>(<%= attr.maxLength %>)<% } else { %>(255)<% }} %><% if (attr.required) { %> NOT NULL<% }; %>, <%}); %>
    id SERIAL
);
