# Backend

## Models refractor
- Class, teacher, admin, student
- Nadenken over hoe teachers en students bij de class horen
  - [cross reference table](https://en.wikipedia.org/wiki/Associative_entity)
  - many to many
  - class stores users
  - users store classes
  - both

## Database
- Use rust-mysql instead of diesel