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

### table with classes
- Contains references to:
  - students
  - teachers
  - badges
- Has in it:
  - assignments

### table with badges
- Has in it:
  - Information about the badge

### table with teachers
- Contains references to:
  - classes
- Has in it
  - Information about the teacher

### table with students
- Contains references to:
  - classes
- Has in it
  - Information about the student
