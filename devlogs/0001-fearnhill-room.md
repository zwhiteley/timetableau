* Identifier: DL#0001
* Created: 2023-01-06
* Status: Active

# Summary

The following development log details the reasons why Fearnhill room support is
necessary, how Fearnhill room support is implemented, and the justifications
for any design choices made during its development.

# Motivation

The Highfield school is in a consortium with the Fearnhill school, meaning that
Highfield students can enroll in Fearnhill courses (and vice versa). Whilst
Fearnhill students are not a primary concern, the application should be
fully accessible to all Highfield students (including those who utilise the 
consortium) -- as a result, proper Fearnhill support will need implemented
such that the consortium students aren't prevented from using the application
or forced to rely on "hacks" for Fearnhill lessons.

At the time of writing, Fearnhill's RNS (Room Numbering Scheme) is unknown and
assumed to be incompatible with Highfield's RNS -- in order to avoid delays in
the development of the application, a separate `FearnhillRoom` struct is to be
created with a universal default value such that other code can be built using
this type. Additionally, the type should be easily modifiable such that, when
information about Fearnhill's RNS is obtained, large amounts of consuming code
(code which uses the `FearnhillRoom` struct) does not have to be modified to
accommodate the modification to the `FearnhillRoom` struct.

# Implementation

* The `FearnhillRoom` struct is to be an empty struct until information about
  Fearnhill's RNS is obtained in order to reduce the complexity of the code --
  a developer may be confused if fields are included but the RNS isn't fully
  known.

* The `FearnhillRoom` struct is to be marked with the `#[non_exhaustive]`
  attribute such that consumers aren't able to create a value of that type
  directly (this allows new fields to be added without causing a breaking
  change).

* The `FearnhillRoom` struct is to have a `Default` trait implementation such
  consumers are able to create the universal value[^1] for that type (allowing
  consumers to partially implement Fearnhill support before information
  is obtained detailing Fearnhill's RNS).

* The `Display` implementation for the `FearnhillRoom` struct is to format
  a universally recognised value (e.g., `FH`) to allow the application to
  be deployed should Fearnhill's RNS information not be gathered in
  time.

* Once information about Fearnhill's RNS is obtained and implemented, all
  fields should become public, the `#[non_exhaustive]` attribute should be
  removed, and this development log should be marked as `inactive`.

# Alternatives

* Delay development until information about Fearnhill's RNS is obtained -- this
  may severely delay the completion of the application and is incredibly
  inefficient.

* Provide a basic implementation for the `FearnhillRoom` struct (e.g., a tuple 
  struct which contains the room identifier as a string of characters) and 
  disregard any specific details about Fearnhill's RNS -- the main drawback 
  of this approach is it becomes incredibly difficult to contextualise room
  identifiers; for example, if `HighfieldRoom` did the same, it would be
  relatively difficult to give the student advice on the exact location
  of the room (e.g., it would become difficult to tell a student that
  the room `HG04` is on the ground floor of the Howard block).

[^1]: This universal value must be fully backwards-compatible if the
      application is deployed before Fearnhill RNS support is fully
      implemented (meaning that the universal value must still exist
      both before and after Fearnhill RNS support is implemented).