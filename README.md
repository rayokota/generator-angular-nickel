# The Angular-Nickel generator 

A [Yeoman](http://yeoman.io) generator for [AngularJS](http://angularjs.org) and [Nickel](http://www.nickel.rs/).

Nickel is a Rust-based micro-framework.  For AngularJS integration with other micro-frameworks, see https://github.com/rayokota/MicroFrameworkRosettaStone.

## Installation

Install [Git](http://git-scm.com), [node.js](http://nodejs.org), [Rust](https://www.rust-lang.org/), and [PostgreSQL](http://www.postgresql.org).


Install Yeoman:

    npm install -g yo

Install the Angular-Nickel generator:

    npm install -g generator-angular-nickel

The above prerequisites can be installed to a VM using the [Angular-Nickel provisioner](https://github.com/rayokota/provision-angular-nickel).

## Creating a Nickel service

In a new directory, generate the service:

    yo angular-nickel

Run the service:

    cargo run

Your service will run at [http://localhost:8080](http://localhost:8080).


## Creating a persistent entity

First, create a database named `my_db` in PostgreSQL.

Generate the entity:

    yo angular-nickel:entity [myentity]

You will be asked to specify attributes for the entity, where each attribute has the following:

- a name
- a type (String, Integer, Float, Boolean, Date, Enum)
- for a String attribute, an optional minimum and maximum length
- for a numeric attribute, an optional minimum and maximum value
- for a Date attribute, an optional constraint to either past values or future values
- for an Enum attribute, a list of enumerated values
- whether the attribute is required

Files that are regenerated will appear as conflicts.  Allow the generator to overwrite these files as long as no custom changes have been made.

Run the service:

    cargo run
    
A client-side AngularJS application will now be available by running

	grunt server
	
The Grunt server will run at [http://localhost:9000](http://localhost:9000).  It will proxy REST requests to the Nickel service running at [http://localhost:8080](http://localhost:8080).

At this point you should be able to navigate to a page to manage your persistent entities.  

The Grunt server supports hot reloading of client-side HTML/CSS/Javascript file changes.

