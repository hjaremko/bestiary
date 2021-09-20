create table if not exists beasts (
    id int not null,
    first_name varchar(255) not null,
    last_name varchar(255) not null,
    titles varchar(255)
);

create table if not exists courses (
    id int not null,
    name varchar(255) not null,
    semester int not null,
    year int not null
);

create table if not exists opinions
(
    id int not null,
    beast_id int not null,
    course_id int not null,
    opinion varchar(1000)
);

insert into beasts (id, first_name, last_name, titles)
values (1, 'Marcin', 'Marchewka', 'dr');

insert into courses (id, name, semester, year)
values (1, 'Programowanie 3', 1, 3);

insert into opinions (id, beast_id, course_id, opinion)
values (1, 1, 1, 'trudne kolokwia');