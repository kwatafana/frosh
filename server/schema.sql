-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.1.1
-- PostgreSQL version: 16.0
-- Project Site: pgmodeler.io
-- Model Author: ---

-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 

-- object: public.gender | type: TYPE --
-- DROP TYPE IF EXISTS public.gender CASCADE;
CREATE TYPE public.gender AS
ENUM ('Male','Female','Other');
-- ddl-end --
ALTER TYPE public.gender OWNER TO postgres;
-- ddl-end --

-- object: public.students | type: TABLE --
-- DROP TABLE IF EXISTS public.students CASCADE;
CREATE TABLE public.students (
	id serial NOT NULL,
	username varchar(15) NOT NULL,
	firstname varchar(50) NOT NULL,
	middlenames varchar(50)[] NOT NULL,
	lastname varchar(50) NOT NULL,
	email varchar(320) NOT NULL,
	photo integer,
	bio varchar(200),
	dob date,
	student_number varchar(100),
	national_id varchar(100),
	physical_address varchar(200),
	mobile varchar(15),
	gender public.gender NOT NULL,
	last_login date,
	joined date NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT primary_key PRIMARY KEY (id),
	CONSTRAINT id_must_be_unique UNIQUE (id),
	CONSTRAINT username_must_be_unique UNIQUE (username),
	CONSTRAINT emails_must_be_unique UNIQUE (email)
);
-- ddl-end --
ALTER TABLE public.students OWNER TO postgres;
-- ddl-end --

-- object: public.courses | type: TABLE --
-- DROP TABLE IF EXISTS public.courses CASCADE;
CREATE TABLE public.courses (
	id serial NOT NULL,
	name varchar(50) NOT NULL,
	description varchar(500) NOT NULL,
	topics varchar(50)[] NOT NULL,
	goals varchar(50)[] NOT NULL,
	textbooks varchar(100)[] NOT NULL,
	grading varchar(300),
	calendar json,
	level integer,
	price money,
	duration interval,
	added date NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT courses_primary_key PRIMARY KEY (id),
	CONSTRAINT id_mus_be_unique UNIQUE (id),
	CONSTRAINT course_name_should_be_unique UNIQUE (name)
);
-- ddl-end --
ALTER TABLE public.courses OWNER TO postgres;
-- ddl-end --

-- object: public.level | type: TABLE --
-- DROP TABLE IF EXISTS public.level CASCADE;
CREATE TABLE public.level (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key PRIMARY KEY (id),
	CONSTRAINT id_is_unique UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.level OWNER TO postgres;
-- ddl-end --

-- object: public.faculty | type: TABLE --
-- DROP TABLE IF EXISTS public.faculty CASCADE;
CREATE TABLE public.faculty (
	id serial NOT NULL,
	CONSTRAINT faculty_primary_key PRIMARY KEY (id),
	CONSTRAINT faculty_id_is_unique UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.faculty OWNER TO postgres;
-- ddl-end --

-- object: public."facultyJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."facultyJunction" CASCADE;
CREATE TABLE public."facultyJunction" (
	id serial NOT NULL,
	course integer NOT NULL,
	faculty integer NOT NULL,
	CONSTRAINT course_faculty_id_is_primary_key PRIMARY KEY (id),
	CONSTRAINT course_faculty_id_is_unique UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."facultyJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.prerequisites | type: TABLE --
-- DROP TABLE IF EXISTS public.prerequisites CASCADE;
CREATE TABLE public.prerequisites (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key_11 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_10 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.prerequisites OWNER TO postgres;
-- ddl-end --

-- object: public."prerequisitesJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."prerequisitesJunction" CASCADE;
CREATE TABLE public."prerequisitesJunction" (
	id integer NOT NULL,
	course integer NOT NULL,
	prerequisite integer NOT NULL,
	CONSTRAINT id_is_primary_key_10 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_9 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."prerequisitesJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.exercises | type: TABLE --
-- DROP TABLE IF EXISTS public.exercises CASCADE;
CREATE TABLE public.exercises (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key_9 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_8 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.exercises OWNER TO postgres;
-- ddl-end --

-- object: public."exercisesJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."exercisesJunction" CASCADE;
CREATE TABLE public."exercisesJunction" (
	id serial NOT NULL,
	course integer NOT NULL,
	exercise integer NOT NULL,
	module integer NOT NULL,
	CONSTRAINT id_is_primary_key_8 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_7 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."exercisesJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.modules | type: TABLE --
-- DROP TABLE IF EXISTS public.modules CASCADE;
CREATE TABLE public.modules (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key_7 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_6 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.modules OWNER TO postgres;
-- ddl-end --

-- object: public."modulesJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."modulesJunction" CASCADE;
CREATE TABLE public."modulesJunction" (
	id serial NOT NULL,
	course integer NOT NULL,
	module integer NOT NULL,
	CONSTRAINT id_is_primary_key_6 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_5 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."modulesJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.exams | type: TABLE --
-- DROP TABLE IF EXISTS public.exams CASCADE;
CREATE TABLE public.exams (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key_5 PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.exams OWNER TO postgres;
-- ddl-end --

-- object: public."examsJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."examsJunction" CASCADE;
CREATE TABLE public."examsJunction" (
	id serial NOT NULL,
	course integer NOT NULL,
	exam integer NOT NULL,
	CONSTRAINT id_is_primary_key_4 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_4 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."examsJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.library | type: TABLE --
-- DROP TABLE IF EXISTS public.library CASCADE;
CREATE TABLE public.library (
	id serial NOT NULL,
	CONSTRAINT id_is_primary_key_3 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_3 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.library OWNER TO postgres;
-- ddl-end --

-- object: public."libraryJunction" | type: TABLE --
-- DROP TABLE IF EXISTS public."libraryJunction" CASCADE;
CREATE TABLE public."libraryJunction" (
	id serial NOT NULL,
	course integer,
	library integer NOT NULL,
	CONSTRAINT id_is_primary_key_2 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_2 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public."libraryJunction" OWNER TO postgres;
-- ddl-end --

-- object: public.student_photos | type: TABLE --
-- DROP TABLE IF EXISTS public.student_photos CASCADE;
CREATE TABLE public.student_photos (
	id serial NOT NULL,
	data bytea NOT NULL,
	mime varchar(255) NOT NULL,
	filename varchar(255),
	added date NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT id_is_primary_key_1 PRIMARY KEY (id),
	CONSTRAINT id_is_unique_1 UNIQUE (id)
);
-- ddl-end --
ALTER TABLE public.student_photos OWNER TO postgres;
-- ddl-end --

-- object: photo_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public.students DROP CONSTRAINT IF EXISTS photo_is_foreign_key CASCADE;
ALTER TABLE public.students ADD CONSTRAINT photo_is_foreign_key FOREIGN KEY (photo)
REFERENCES public.student_photos (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: level_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public.courses DROP CONSTRAINT IF EXISTS level_foreign_key CASCADE;
ALTER TABLE public.courses ADD CONSTRAINT level_foreign_key FOREIGN KEY (level)
REFERENCES public.level (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."facultyJunction" DROP CONSTRAINT IF EXISTS course_foreign_key CASCADE;
ALTER TABLE public."facultyJunction" ADD CONSTRAINT course_foreign_key FOREIGN KEY (id)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: faculty_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."facultyJunction" DROP CONSTRAINT IF EXISTS faculty_foreign_key CASCADE;
ALTER TABLE public."facultyJunction" ADD CONSTRAINT faculty_foreign_key FOREIGN KEY (id)
REFERENCES public.faculty (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."prerequisitesJunction" DROP CONSTRAINT IF EXISTS course_is_foreign_key CASCADE;
ALTER TABLE public."prerequisitesJunction" ADD CONSTRAINT course_is_foreign_key FOREIGN KEY (course)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: prerequisite_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."prerequisitesJunction" DROP CONSTRAINT IF EXISTS prerequisite_is_foreign_key CASCADE;
ALTER TABLE public."prerequisitesJunction" ADD CONSTRAINT prerequisite_is_foreign_key FOREIGN KEY (prerequisite)
REFERENCES public.prerequisites (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."exercisesJunction" DROP CONSTRAINT IF EXISTS course_is_foreign_key CASCADE;
ALTER TABLE public."exercisesJunction" ADD CONSTRAINT course_is_foreign_key FOREIGN KEY (course)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: exercise_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."exercisesJunction" DROP CONSTRAINT IF EXISTS exercise_is_foreign_key CASCADE;
ALTER TABLE public."exercisesJunction" ADD CONSTRAINT exercise_is_foreign_key FOREIGN KEY (exercise)
REFERENCES public.exercises (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: module_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."exercisesJunction" DROP CONSTRAINT IF EXISTS module_is_foreign_key CASCADE;
ALTER TABLE public."exercisesJunction" ADD CONSTRAINT module_is_foreign_key FOREIGN KEY (module)
REFERENCES public.modules (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."modulesJunction" DROP CONSTRAINT IF EXISTS course_is_foreign_key CASCADE;
ALTER TABLE public."modulesJunction" ADD CONSTRAINT course_is_foreign_key FOREIGN KEY (course)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: module_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."modulesJunction" DROP CONSTRAINT IF EXISTS module_is_foreign_key CASCADE;
ALTER TABLE public."modulesJunction" ADD CONSTRAINT module_is_foreign_key FOREIGN KEY (module)
REFERENCES public.modules (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."examsJunction" DROP CONSTRAINT IF EXISTS course_is_foreign_key CASCADE;
ALTER TABLE public."examsJunction" ADD CONSTRAINT course_is_foreign_key FOREIGN KEY (course)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: exam_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."examsJunction" DROP CONSTRAINT IF EXISTS exam_is_foreign_key CASCADE;
ALTER TABLE public."examsJunction" ADD CONSTRAINT exam_is_foreign_key FOREIGN KEY (exam)
REFERENCES public.exams (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: course_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."libraryJunction" DROP CONSTRAINT IF EXISTS course_is_foreign_key CASCADE;
ALTER TABLE public."libraryJunction" ADD CONSTRAINT course_is_foreign_key FOREIGN KEY (id)
REFERENCES public.courses (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: library_is_foreign_key | type: CONSTRAINT --
-- ALTER TABLE public."libraryJunction" DROP CONSTRAINT IF EXISTS library_is_foreign_key CASCADE;
ALTER TABLE public."libraryJunction" ADD CONSTRAINT library_is_foreign_key FOREIGN KEY (library)
REFERENCES public.library (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --


